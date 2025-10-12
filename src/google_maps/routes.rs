use anyhow::{Context, Result, anyhow};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

const ENDPOINT: &str = "https://routes.googleapis.com/distanceMatrix/v2:computeRouteMatrix";
const DEFAULT_FIELD_MASK: &str = "originIndex,destinationIndex,status,distanceMeters,duration";

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPoint {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutesClient {
    http: reqwest::Client,
    api_key: String,
}

impl RoutesClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::new_with_client(reqwest::Client::new(), api_key)
    }

    pub fn new_with_client(client: reqwest::Client, api_key: impl Into<String>) -> Self {
        Self {
            http: client,
            api_key: api_key.into(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let api_key =
            env::var("GOOGLE_API_KEY").context("GOOGLE_API_KEY environment variable is not set")?;
        Ok(Self::new(api_key))
    }

    pub async fn compute_route_matrix(
        &self,
        origins: &[GeoPoint],
        destinations: &[GeoPoint],
    ) -> Result<Vec<RouteMatrixElement>> {
        let url = format!("{ENDPOINT}?key={}", self.api_key);
        let body = ComputeRouteMatrixRequest::from_points(origins, destinations);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Goog-FieldMask",
            HeaderValue::from_static(DEFAULT_FIELD_MASK),
        );

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .context("Failed to call Google Routes API")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Google Routes response body")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<GoogleErrorResponse>(&text) {
                let message = err
                    .error
                    .message
                    .unwrap_or_else(|| "Unknown error from Google Routes".to_string());
                return Err(anyhow!(
                    "Google Routes API error ({code}): {message}",
                    code = err.error.code
                ));
            }

            return Err(anyhow!(
                "Google Routes API returned {status}: {text}",
                status = status,
                text = text
            ));
        }

        serde_json::from_str(&text)
            .with_context(|| "Failed to deserialize route matrix response".to_string())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ComputeRouteMatrixRequest {
    origins: Vec<RouteMatrixWaypoint>,
    destinations: Vec<RouteMatrixWaypoint>,
}

impl ComputeRouteMatrixRequest {
    fn from_points(origins: &[GeoPoint], destinations: &[GeoPoint]) -> Self {
        Self {
            origins: origins
                .iter()
                .map(|point| RouteMatrixWaypoint::from_point(*point))
                .collect(),
            destinations: destinations
                .iter()
                .map(|point| RouteMatrixWaypoint::from_point(*point))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RouteMatrixWaypoint {
    waypoint: Waypoint,
}

impl RouteMatrixWaypoint {
    fn from_point(point: GeoPoint) -> Self {
        Self {
            waypoint: Waypoint {
                location: Location { lat_lng: point },
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Waypoint {
    location: Location,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    #[serde(rename = "latLng")]
    lat_lng: GeoPoint,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMatrixElement {
    pub origin_index: i32,
    pub destination_index: i32,
    #[serde(default)]
    pub status: Option<RouteMatrixStatus>,
    #[serde(default)]
    pub distance_meters: Option<f64>,
    #[serde(default)]
    pub duration: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMatrixStatus {
    #[serde(default)]
    pub code: Option<i32>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleErrorResponse {
    error: GoogleErrorBody,
}

#[derive(Debug, Deserialize)]
struct GoogleErrorBody {
    code: i32,
    #[serde(default)]
    message: Option<String>,
}

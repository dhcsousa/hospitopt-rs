use anyhow::Result;
use entity::{patient, prelude::Patient};
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, PaginatorTrait,
    TransactionTrait,
};
use std::{fmt, str::FromStr};

pub const TRIAGE_CONDITIONS: &[&str] = &[
    "Emergência (Vermelho)",
    "Muito Urgente (Laranja)",
    "Urgente (Amarelo)",
    "Pouco Urgente (Verde)",
    "Não Urgente (Azul)",
];

const LAT_MIN: f64 = 38.70;
const LAT_MAX: f64 = 38.80;
const LON_MIN: f64 = -9.28;
const LON_MAX: f64 = -9.12;

pub struct PatientSeedReport {
    pub inserted: usize,
    pub total: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatientSeedMode {
    Reset,
    Append,
}

impl PatientSeedMode {
    pub fn label(self) -> &'static str {
        match self {
            PatientSeedMode::Reset => "reset",
            PatientSeedMode::Append => "append",
        }
    }
}

impl fmt::Display for PatientSeedMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl FromStr for PatientSeedMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "reset" | "replace" | "clear" => Ok(PatientSeedMode::Reset),
            "append" | "add" | "keep" => Ok(PatientSeedMode::Append),
            other => Err(format!(
                "Unsupported patient seed mode '{other}'. Expected 'reset' or 'append'."
            )),
        }
    }
}

pub async fn seed(db: &DatabaseConnection, mode: PatientSeedMode) -> Result<PatientSeedReport> {
    let mut txn = db.begin().await?;

    if matches!(mode, PatientSeedMode::Reset) {
        Patient::delete_many().exec(&mut txn).await?;
    }

    let mut rng = rand::rng();
    let patient_count = rng.random_range(5..=10);

    for _ in 0..patient_count {
        let condition_idx = rng.random_range(0..TRIAGE_CONDITIONS.len());
        let latitude = rng.random_range(LAT_MIN..LAT_MAX);
        let longitude = rng.random_range(LON_MIN..LON_MAX);

        patient::ActiveModel {
            condition: Set(TRIAGE_CONDITIONS[condition_idx].to_string()),
            latitude: Set(latitude),
            longitude: Set(longitude),
            ..Default::default()
        }
        .insert(&mut txn)
        .await?;
    }

    txn.commit().await?;

    let total = Patient::find().count(db).await? as usize;

    Ok(PatientSeedReport {
        inserted: patient_count,
        total,
    })
}

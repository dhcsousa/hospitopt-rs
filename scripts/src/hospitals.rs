use anyhow::Result;
use entity::{
    hospital, hospital_speciality,
    prelude::{Hospital, HospitalSpeciality},
};
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait,
};

pub struct HospitalSeedReport {
    pub hospitals: usize,
    pub triage_levels: usize,
}

struct HospitalSeed {
    name: &'static str,
    latitude: f64,
    longitude: f64,
    bed_capacity: i32,
}

struct TriageLevelSeed {
    name: &'static str,
    waiting_time_seconds: i64,
    beds_capacity: i32,
}

struct TriageLevelConfig {
    name: &'static str,
    wait_min_seconds: i64,
    wait_max_seconds: i64,
    share_min: f32,
    share_max: f32,
}

pub async fn seed(db: &DatabaseConnection) -> Result<HospitalSeedReport> {
    let hospitals = hospital_blueprints();
    let mut txn = db.begin().await?;

    HospitalSpeciality::delete_many().exec(&mut txn).await?;
    Hospital::delete_many().exec(&mut txn).await?;

    let mut rng = rand::rng();
    let mut triage_total = 0usize;

    for hospital_seed in &hospitals {
        let inserted = hospital::ActiveModel {
            location_name: Set(hospital_seed.name.to_string()),
            latitude: Set(hospital_seed.latitude),
            longitude: Set(hospital_seed.longitude),
            bed_capacity: Set(hospital_seed.bed_capacity),
            ..Default::default()
        }
        .insert(&mut txn)
        .await?;

        for triage in build_manchester_levels(hospital_seed.bed_capacity, &mut rng) {
            hospital_speciality::ActiveModel {
                hospital_id: Set(inserted.id),
                triage_level: Set(triage.name.to_string()),
                waiting_time_seconds: Set(triage.waiting_time_seconds),
                beds_capacity: Set(triage.beds_capacity),
                ..Default::default()
            }
            .insert(&mut txn)
            .await?;

            triage_total += 1;
        }
    }

    txn.commit().await?;

    Ok(HospitalSeedReport {
        hospitals: hospitals.len(),
        triage_levels: triage_total,
    })
}

fn hospital_blueprints() -> Vec<HospitalSeed> {
    vec![
        HospitalSeed {
            name: "Hospital Egas Moniz",
            latitude: 38.699920286486,
            longitude: -9.18819546474445,
            bed_capacity: 420,
        },
        HospitalSeed {
            name: "Hospital São Francisco Xavier",
            latitude: 38.707161256896,
            longitude: -9.21788492844298,
            bed_capacity: 360,
        },
        HospitalSeed {
            name: "Hospital São José",
            latitude: 38.7177291826238,
            longitude: -9.13744886489834,
            bed_capacity: 510,
        },
        HospitalSeed {
            name: "Hospital Santa Marta",
            latitude: 38.7237874362237,
            longitude: -9.14485431710445,
            bed_capacity: 280,
        },
        HospitalSeed {
            name: "Hospital Dona Estefânia",
            latitude: 38.7288310990263,
            longitude: -9.13891561875593,
            bed_capacity: 330,
        },
        HospitalSeed {
            name: "Instituto Português Oncologia de Lisboa",
            latitude: 38.7397869315826,
            longitude: -9.161252182332,
            bed_capacity: 400,
        },
        HospitalSeed {
            name: "Hospital Curry Cabral",
            latitude: 38.7413193138079,
            longitude: -9.15205064090183,
            bed_capacity: 350,
        },
        HospitalSeed {
            name: "Hospital Santa Maria",
            latitude: 38.7480419586009,
            longitude: -9.16054501791882,
            bed_capacity: 520,
        },
    ]
}

fn build_manchester_levels(total_beds: i32, rng: &mut impl Rng) -> Vec<TriageLevelSeed> {
    const LEVEL_CONFIG: [TriageLevelConfig; 5] = [
        TriageLevelConfig {
            name: "Emergência (Vermelho)",
            wait_min_seconds: 0,
            wait_max_seconds: 120,
            share_min: 0.02,
            share_max: 0.05,
        },
        TriageLevelConfig {
            name: "Muito Urgente (Laranja)",
            wait_min_seconds: 5 * 60,
            wait_max_seconds: 12 * 60,
            share_min: 0.05,
            share_max: 0.12,
        },
        TriageLevelConfig {
            name: "Urgente (Amarelo)",
            wait_min_seconds: 45 * 60,
            wait_max_seconds: 75 * 60,
            share_min: 0.20,
            share_max: 0.32,
        },
        TriageLevelConfig {
            name: "Pouco Urgente (Verde)",
            wait_min_seconds: 90 * 60,
            wait_max_seconds: 150 * 60,
            share_min: 0.20,
            share_max: 0.30,
        },
        TriageLevelConfig {
            name: "Não Urgente (Azul)",
            wait_min_seconds: 180 * 60,
            wait_max_seconds: 300 * 60,
            share_min: 0.20,
            share_max: 0.35,
        },
    ];

    let sampled_weights: Vec<f32> = LEVEL_CONFIG
        .iter()
        .map(|cfg| rng.random_range(cfg.share_min..=cfg.share_max))
        .collect();

    let total_weight: f32 = sampled_weights.iter().sum();

    let mut allocations: Vec<i32> = sampled_weights
        .iter()
        .map(|weight| ((*weight / total_weight) * total_beds as f32).round() as i32)
        .collect();

    for beds in allocations.iter_mut() {
        if *beds <= 0 {
            *beds = 1;
        }
    }

    let mut assigned: i32 = allocations.iter().sum();

    while assigned < total_beds {
        for idx in (0..allocations.len()).rev() {
            allocations[idx] += 1;
            assigned += 1;
            if assigned == total_beds {
                break;
            }
        }
    }

    while assigned > total_beds {
        for idx in (0..allocations.len()).rev() {
            if allocations[idx] > 1 {
                allocations[idx] -= 1;
                assigned -= 1;
            }
            if assigned == total_beds {
                break;
            }
        }
    }

    LEVEL_CONFIG
        .iter()
        .zip(allocations.into_iter())
        .map(|(cfg, beds)| TriageLevelSeed {
            name: cfg.name,
            waiting_time_seconds: rng.random_range(cfg.wait_min_seconds..=cfg.wait_max_seconds),
            beds_capacity: beds,
        })
        .collect()
}

use anyhow::{Context, Result};
use entity::prelude::*;
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL environment variable is not set")?;

    let db = Database::connect(&database_url)
        .await
        .context("Failed to connect to the database")?;

    print_hospitals(&db).await?;
    print_hospital_specialities(&db).await?;
    print_patients(&db).await?;

    Ok(())
}

async fn print_hospitals(db: &DatabaseConnection) -> Result<(), DbErr> {
    let hospitals = Hospital::find().all(db).await?;
    if hospitals.is_empty() {
        println!("(no hospitals found)");
    } else {
        println!("\n== Hospitals ({} records) ==", hospitals.len());
        for hospital in hospitals {
            println!("{:#?}", hospital);
        }
    }
    Ok(())
}

async fn print_hospital_specialities(db: &DatabaseConnection) -> Result<(), DbErr> {
    let specialities = HospitalSpeciality::find().all(db).await?;
    if specialities.is_empty() {
        println!("(no hospital specialities found)");
    } else {
        println!(
            "\n== Hospital Specialities ({} records) ==",
            specialities.len(),
        );
        for speciality in specialities {
            println!("{:#?}", speciality);
        }
    }
    Ok(())
}

async fn print_patients(db: &DatabaseConnection) -> Result<(), DbErr> {
    let patients = Patient::find().all(db).await?;
    if patients.is_empty() {
        println!("(no patients found)");
    } else {
        println!("\n== Patients ({} records) ==", patients.len());
        for patient in patients {
            println!("{:#?}", patient);
        }
    }
    Ok(())
}

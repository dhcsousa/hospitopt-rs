pub use sea_orm_migration::prelude::*;

mod m20251005_195352_hospitals;
mod m20251006_210502_patients;
mod m20251006_211530_beds_per_speciality;
mod m20251007_182626_rename_speciality_to_triage;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251005_195352_hospitals::Migration),
            Box::new(m20251006_210502_patients::Migration),
            Box::new(m20251006_211530_beds_per_speciality::Migration),
            Box::new(m20251007_182626_rename_speciality_to_triage::Migration),
        ]
    }
}

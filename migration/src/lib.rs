pub use sea_orm_migration::prelude::*;

mod m20251005_195352_hospitals;
mod m20251006_210502_patients;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251005_195352_hospitals::Migration),
            Box::new(m20251006_210502_patients::Migration),
        ]
    }
}

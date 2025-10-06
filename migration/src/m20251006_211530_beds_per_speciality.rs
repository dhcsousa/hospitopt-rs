use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(HospitalSpecialities::Table)
                    .add_column(
                        ColumnDef::new(HospitalSpecialities::BedsCapacity)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(HospitalSpecialities::Table)
                    .drop_column(HospitalSpecialities::BedsCapacity)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum HospitalSpecialities {
    #[sea_orm(iden = "hospital_specialities")]
    Table,
    BedsCapacity,
}

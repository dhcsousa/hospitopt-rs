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
                    .rename_column(
                        HospitalSpecialities::SpecialityName,
                        HospitalSpecialities::TriageLevel,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("uq-hospital_specialities-hospital-speciality")
                    .table(HospitalSpecialities::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("uq-hospital_specialities-hospital-triage")
                    .table(HospitalSpecialities::Table)
                    .col(HospitalSpecialities::HospitalId)
                    .col(HospitalSpecialities::TriageLevel)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("uq-hospital_specialities-hospital-triage")
                    .table(HospitalSpecialities::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(HospitalSpecialities::Table)
                    .rename_column(
                        HospitalSpecialities::TriageLevel,
                        HospitalSpecialities::SpecialityName,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("uq-hospital_specialities-hospital-speciality")
                    .table(HospitalSpecialities::Table)
                    .col(HospitalSpecialities::HospitalId)
                    .col(HospitalSpecialities::SpecialityName)
                    .unique()
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
    HospitalId,
    #[sea_orm(iden = "speciality_name")]
    SpecialityName,
    #[sea_orm(iden = "triage_level")]
    TriageLevel,
}

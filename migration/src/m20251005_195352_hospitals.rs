use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(create_hospitals_table()).await?;

        manager
            .create_table(create_hospital_specialities_table())
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-hospital_specialities-hospital")
                    .from(
                        HospitalSpecialities::Table,
                        HospitalSpecialities::HospitalId,
                    )
                    .to(Hospitals::Table, Hospitals::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-hospital_specialities-hospital_id")
                    .table(HospitalSpecialities::Table)
                    .col(HospitalSpecialities::HospitalId)
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("uq-hospital_specialities-hospital-speciality")
                    .table(HospitalSpecialities::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-hospital_specialities-hospital_id")
                    .table(HospitalSpecialities::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-hospital_specialities-hospital")
                    .table(HospitalSpecialities::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(HospitalSpecialities::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Hospitals::Table).to_owned())
            .await?;

        Ok(())
    }
}

fn create_hospitals_table() -> TableCreateStatement {
    Table::create()
        .table(Hospitals::Table)
        .if_not_exists()
        .col(pk_auto(Hospitals::Id))
        .col(string(Hospitals::LocationName).not_null())
        .col(ColumnDef::new(Hospitals::Latitude).double().not_null())
        .col(ColumnDef::new(Hospitals::Longitude).double().not_null())
        .col(ColumnDef::new(Hospitals::BedCapacity).integer().not_null())
        .to_owned()
}

fn create_hospital_specialities_table() -> TableCreateStatement {
    Table::create()
        .table(HospitalSpecialities::Table)
        .if_not_exists()
        .col(pk_auto(HospitalSpecialities::Id))
        .col(
            ColumnDef::new(HospitalSpecialities::HospitalId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(HospitalSpecialities::SpecialityName)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(HospitalSpecialities::WaitingTimeSeconds)
                .big_integer()
                .not_null(),
        )
        .to_owned()
}

#[derive(DeriveIden)]
enum Hospitals {
    #[sea_orm(iden = "hospitals")]
    Table,
    Id,
    LocationName,
    Latitude,
    Longitude,
    BedCapacity,
}

#[derive(DeriveIden)]
enum HospitalSpecialities {
    #[sea_orm(iden = "hospital_specialities")]
    Table,
    Id,
    HospitalId,
    SpecialityName,
    WaitingTimeSeconds,
}

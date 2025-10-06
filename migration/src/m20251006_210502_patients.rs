use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Patients::Table)
                    .if_not_exists()
                    .col(pk_auto(Patients::Id))
                    .col(string(Patients::Condition).not_null())
                    .col(ColumnDef::new(Patients::Latitude).double().not_null())
                    .col(ColumnDef::new(Patients::Longitude).double().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Patients::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Patients {
    #[sea_orm(iden = "patients")]
    Table,
    Id,
    Condition,
    Latitude,
    Longitude,
}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "patients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub condition: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hospitals")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub location_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub bed_capacity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::hospital_specialty::Entity")]
    HospitalSpecialty,
}

impl Related<super::hospital_specialty::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HospitalSpecialty.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

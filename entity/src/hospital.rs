use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hospitals")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub bed_capacity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::hospital_speciality::Entity")]
    HospitalSpeciality,
}

impl Related<super::hospital_speciality::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HospitalSpeciality.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hospital_specialities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hospital_id: i32,
    pub speciality_name: String,
    pub waiting_time_seconds: i64,
    pub beds_capacity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::hospital::Entity",
        from = "Column::HospitalId",
        to = "super::hospital::Column::Id",
        on_delete = "Cascade"
    )]
    Hospital,
}

impl Related<super::hospital::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hospital.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

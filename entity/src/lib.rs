pub mod hospital;
pub mod hospital_speciality;
pub mod patient;

pub mod prelude {
    pub use super::hospital::Entity as Hospital;
    pub use super::hospital_speciality::Entity as HospitalSpeciality;
    pub use super::patient::Entity as Patient;
}

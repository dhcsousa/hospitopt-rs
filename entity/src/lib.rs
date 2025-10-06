pub mod hospital;
pub mod hospital_specialty;
pub mod patient;

pub mod prelude {
    pub use super::hospital::Entity as Hospital;
    pub use super::hospital_specialty::Entity as HospitalSpecialty;
    pub use super::patient::Entity as Patient;
}

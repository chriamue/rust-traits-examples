pub mod airplane;
pub mod amphibious;
pub mod car;
pub mod helicopter;
pub mod motorcycle;
pub mod ship;
pub mod vehicle;

pub use airplane::Airplane;
pub use amphibious::AmphibiousVehicle;
pub use car::Car;
pub use helicopter::Helicopter;
pub use motorcycle::Motorcycle;
pub use ship::Ship;
pub use vehicle::Vehicle;

/// Demonstrate polymorphism with trait objects
pub fn describe_vehicle(vehicle: &dyn Vehicle) -> String {
    vehicle.description()
}

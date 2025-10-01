/// Core trait that all vehicles must implement
pub trait Vehicle {
    /// Get the vehicle's identifier/name
    fn name(&self) -> String;

    /// Get the vehicle's type
    fn vehicle_type(&self) -> &'static str;

    /// Get the vehicle's manufacturer
    fn manufacturer(&self) -> &str;

    /// Get the vehicle's year
    fn year(&self) -> u32;

    /// Get a description of the vehicle
    fn description(&self) -> String {
        format!(
            "{} {} {} ({})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type()
        )
    }
}

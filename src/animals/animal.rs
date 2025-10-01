pub trait Animal {
    fn name(&self) -> String;

    fn species(&self) -> &'static str;

    /// Get a description of the animal
    fn description(&self) -> String {
        format!("{} is a {}", self.name(), self.species())
    }
}

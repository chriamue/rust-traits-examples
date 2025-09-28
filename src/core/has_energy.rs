use super::energy_level::EnergyLevel;

/// Core trait for entities that have energy
pub trait HasEnergy {
    /// Get current energy level
    fn energy(&self) -> EnergyLevel;

    /// Set energy level
    fn set_energy(&mut self, level: EnergyLevel);

    /// Check if entity has sufficient energy for an activity
    fn can_perform(&self, required_level: EnergyLevel) -> bool {
        self.energy() >= required_level
    }

    /// Consume energy (decrease by one level)
    fn consume_energy(&mut self) {
        let mut current = self.energy();
        current.decrease();
        self.set_energy(current);
    }

    /// Consume multiple energy levels
    fn consume_energy_levels(&mut self, levels: u8) {
        let mut current = self.energy();
        for _ in 0..levels {
            current.decrease();
        }
        self.set_energy(current);
    }

    /// Rest and recover energy
    fn rest(&mut self) {
        let mut current = self.energy();
        current.increase();
        self.set_energy(current);
    }
}

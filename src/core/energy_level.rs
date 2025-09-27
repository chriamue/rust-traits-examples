#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnergyLevel {
    Collapsed = 0,   // Cannot perform any actions
    Exhausted = 1,   // Can only rest
    Tired = 2,       // Limited actions only
    Normal = 3,      // Normal activity level
    Energetic = 4,   // High activity level
    Hyperactive = 5, // Maximum energy
}

impl EnergyLevel {
    /// Create energy level from numeric value (0-100)
    pub fn from_points(points: u8) -> Self {
        match points {
            0..=10 => EnergyLevel::Collapsed,
            11..=25 => EnergyLevel::Exhausted,
            26..=50 => EnergyLevel::Tired,
            51..=75 => EnergyLevel::Normal,
            76..=90 => EnergyLevel::Energetic,
            91..=100 => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal, // Fallback for values > 100
        }
    }

    /// Convert to numeric points (0-100)
    pub fn to_points(&self) -> u8 {
        match self {
            EnergyLevel::Collapsed => 5,
            EnergyLevel::Exhausted => 20,
            EnergyLevel::Tired => 40,
            EnergyLevel::Normal => 65,
            EnergyLevel::Energetic => 85,
            EnergyLevel::Hyperactive => 100,
        }
    }

    /// Check if energy level allows for basic movement
    pub fn can_move(&self) -> bool {
        *self > EnergyLevel::Collapsed
    }

    /// Check if energy level allows for intensive activities
    pub fn can_run(&self) -> bool {
        *self >= EnergyLevel::Normal
    }

    /// Get the display name for this energy level
    pub fn name(&self) -> &'static str {
        match self {
            EnergyLevel::Collapsed => "Collapsed",
            EnergyLevel::Exhausted => "Exhausted",
            EnergyLevel::Tired => "Tired",
            EnergyLevel::Normal => "Normal",
            EnergyLevel::Energetic => "Energetic",
            EnergyLevel::Hyperactive => "Hyperactive",
        }
    }

    /// Reduce energy level by one step
    pub fn decrease(&mut self) {
        *self = match self {
            EnergyLevel::Hyperactive => EnergyLevel::Energetic,
            EnergyLevel::Energetic => EnergyLevel::Normal,
            EnergyLevel::Normal => EnergyLevel::Tired,
            EnergyLevel::Tired => EnergyLevel::Exhausted,
            EnergyLevel::Exhausted => EnergyLevel::Collapsed,
            EnergyLevel::Collapsed => EnergyLevel::Collapsed, // Can't go lower
        };
    }

    /// Increase energy level by one step
    pub fn increase(&mut self) {
        *self = match self {
            EnergyLevel::Collapsed => EnergyLevel::Exhausted,
            EnergyLevel::Exhausted => EnergyLevel::Tired,
            EnergyLevel::Tired => EnergyLevel::Normal,
            EnergyLevel::Normal => EnergyLevel::Energetic,
            EnergyLevel::Energetic => EnergyLevel::Hyperactive,
            EnergyLevel::Hyperactive => EnergyLevel::Hyperactive, // Can't go higher
        };
    }
}

impl std::fmt::Display for EnergyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_level_from_points() {
        assert_eq!(EnergyLevel::from_points(0), EnergyLevel::Collapsed);
        assert_eq!(EnergyLevel::from_points(15), EnergyLevel::Exhausted);
        assert_eq!(EnergyLevel::from_points(35), EnergyLevel::Tired);
        assert_eq!(EnergyLevel::from_points(65), EnergyLevel::Normal);
        assert_eq!(EnergyLevel::from_points(85), EnergyLevel::Energetic);
        assert_eq!(EnergyLevel::from_points(95), EnergyLevel::Hyperactive);
    }

    #[test]
    fn test_energy_level_capabilities() {
        assert!(!EnergyLevel::Collapsed.can_move());
        assert!(EnergyLevel::Exhausted.can_move());

        assert!(!EnergyLevel::Tired.can_run());
        assert!(EnergyLevel::Normal.can_run());
    }

    #[test]
    fn test_energy_level_modification() {
        let mut energy = EnergyLevel::Normal;
        energy.decrease();
        assert_eq!(energy, EnergyLevel::Tired);

        energy.increase();
        assert_eq!(energy, EnergyLevel::Normal);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", EnergyLevel::Collapsed), "Collapsed");
        assert_eq!(format!("{}", EnergyLevel::Normal), "Normal");
    }
}

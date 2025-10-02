use crate::core::EnergyLevel;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intensity {
    Gentle,
    Moderate,
    Vigorous,
    Intense,
    Maximum,
}

impl Intensity {
    /// Get the difficulty level of this intensity (1-5)
    pub fn difficulty_level(&self) -> u8 {
        match self {
            Intensity::Gentle => 1,
            Intensity::Moderate => 2,
            Intensity::Vigorous => 3,
            Intensity::Intense => 4,
            Intensity::Maximum => 5,
        }
    }

    /// Get the energy cost for this intensity level
    pub fn energy_cost(&self) -> u8 {
        match self {
            Intensity::Gentle => 0,   // No extra cost
            Intensity::Moderate => 0, // No extra cost, just base movement
            Intensity::Vigorous => 1, // Small extra cost
            Intensity::Intense => 2,  // Moderate extra cost
            Intensity::Maximum => 3,  // High extra cost
        }
    }

    /// Get the minimum energy level required for this intensity
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self {
            Intensity::Gentle => EnergyLevel::Exhausted,
            Intensity::Moderate => EnergyLevel::Tired,
            Intensity::Vigorous => EnergyLevel::Normal,
            Intensity::Intense => EnergyLevel::Energetic,
            Intensity::Maximum => EnergyLevel::Hyperactive,
        }
    }

    /// Get a description of this intensity level
    pub fn description(&self) -> &'static str {
        match self {
            Intensity::Gentle => "gentle, low-impact movement",
            Intensity::Moderate => "moderate, steady movement",
            Intensity::Vigorous => "vigorous, energetic movement",
            Intensity::Intense => "intense, high-energy movement",
            Intensity::Maximum => "maximum effort, all-out movement",
        }
    }

    /// Get the speed multiplier for this intensity (relative to base speed)
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            Intensity::Gentle => 0.5,   // Half speed
            Intensity::Moderate => 1.0, // Normal speed
            Intensity::Vigorous => 1.5, // 1.5x speed
            Intensity::Intense => 2.0,  // Double speed
            Intensity::Maximum => 3.0,  // Triple speed
        }
    }

    /// Check if this intensity is sustainable for long periods
    pub fn is_sustainable(&self) -> bool {
        match self {
            Intensity::Gentle | Intensity::Moderate => true,
            Intensity::Vigorous => false, // Can be maintained for moderate periods
            Intensity::Intense | Intensity::Maximum => false, // Short bursts only
        }
    }

    /// Get all intensity levels
    pub fn all_intensities() -> Vec<Intensity> {
        vec![
            Intensity::Gentle,
            Intensity::Moderate,
            Intensity::Vigorous,
            Intensity::Intense,
            Intensity::Maximum,
        ]
    }

    /// Get intensities that can be sustained with the given energy level
    pub fn available_for_energy(energy: EnergyLevel) -> Vec<Intensity> {
        Self::all_intensities()
            .into_iter()
            .filter(|intensity| energy >= intensity.required_energy_level())
            .collect()
    }

    /// Get intensities up to a certain difficulty level
    pub fn by_max_difficulty(max_difficulty: u8) -> Vec<Intensity> {
        Self::all_intensities()
            .into_iter()
            .filter(|intensity| intensity.difficulty_level() <= max_difficulty)
            .collect()
    }
}

impl fmt::Display for Intensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Intensity::Gentle => "Gentle",
            Intensity::Moderate => "Moderate",
            Intensity::Vigorous => "Vigorous",
            Intensity::Intense => "Intense",
            Intensity::Maximum => "Maximum",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for Intensity {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gentle" | "easy" | "slow" => Intensity::Gentle,
            "moderate" | "normal" | "medium" => Intensity::Moderate,
            "vigorous" | "energetic" | "active" => Intensity::Vigorous,
            "intense" | "hard" | "high" => Intensity::Intense,
            "maximum" | "max" | "extreme" | "all_out" => Intensity::Maximum,
            _ => Intensity::Moderate, // Default fallback
        }
    }
}

impl From<u8> for Intensity {
    fn from(level: u8) -> Self {
        match level {
            1 => Intensity::Gentle,
            2 => Intensity::Moderate,
            3 => Intensity::Vigorous,
            4 => Intensity::Intense,
            5..=u8::MAX => Intensity::Maximum,
            0 => Intensity::Gentle, // Fallback for 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_levels() {
        assert_eq!(Intensity::Gentle.difficulty_level(), 1);
        assert_eq!(Intensity::Moderate.difficulty_level(), 2);
        assert_eq!(Intensity::Vigorous.difficulty_level(), 3);
        assert_eq!(Intensity::Intense.difficulty_level(), 4);
        assert_eq!(Intensity::Maximum.difficulty_level(), 5);
    }

    #[test]
    fn test_energy_requirements() {
        assert_eq!(
            Intensity::Gentle.required_energy_level(),
            EnergyLevel::Exhausted
        );
        assert_eq!(
            Intensity::Moderate.required_energy_level(),
            EnergyLevel::Tired
        );
        assert_eq!(
            Intensity::Vigorous.required_energy_level(),
            EnergyLevel::Normal
        );
        assert_eq!(
            Intensity::Intense.required_energy_level(),
            EnergyLevel::Energetic
        );
        assert_eq!(
            Intensity::Maximum.required_energy_level(),
            EnergyLevel::Hyperactive
        );
    }

    #[test]
    fn test_energy_costs() {
        assert_eq!(Intensity::Gentle.energy_cost(), 0);
        assert_eq!(Intensity::Moderate.energy_cost(), 0);
        assert_eq!(Intensity::Vigorous.energy_cost(), 1);
        assert_eq!(Intensity::Intense.energy_cost(), 2);
        assert_eq!(Intensity::Maximum.energy_cost(), 3);
    }

    #[test]
    fn test_speed_multipliers() {
        assert_eq!(Intensity::Gentle.speed_multiplier(), 0.5);
        assert_eq!(Intensity::Moderate.speed_multiplier(), 1.0);
        assert_eq!(Intensity::Vigorous.speed_multiplier(), 1.5);
        assert_eq!(Intensity::Intense.speed_multiplier(), 2.0);
        assert_eq!(Intensity::Maximum.speed_multiplier(), 3.0);
    }

    #[test]
    fn test_sustainability() {
        assert!(Intensity::Gentle.is_sustainable());
        assert!(Intensity::Moderate.is_sustainable());
        assert!(!Intensity::Vigorous.is_sustainable());
        assert!(!Intensity::Intense.is_sustainable());
        assert!(!Intensity::Maximum.is_sustainable());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Intensity::from("gentle"), Intensity::Gentle);
        assert_eq!(Intensity::from("vigorous"), Intensity::Vigorous);
        assert_eq!(Intensity::from("maximum"), Intensity::Maximum);
        assert_eq!(Intensity::from("unknown"), Intensity::Moderate); // fallback
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Intensity::from(1), Intensity::Gentle);
        assert_eq!(Intensity::from(3), Intensity::Vigorous);
        assert_eq!(Intensity::from(5), Intensity::Maximum);
        assert_eq!(Intensity::from(10), Intensity::Maximum); // clamped to max
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Intensity::Gentle), "Gentle");
        assert_eq!(format!("{}", Intensity::Maximum), "Maximum");
    }

    #[test]
    fn test_available_for_energy() {
        let available = Intensity::available_for_energy(EnergyLevel::Normal);
        assert!(available.contains(&Intensity::Gentle));
        assert!(available.contains(&Intensity::Moderate));
        assert!(available.contains(&Intensity::Vigorous));
        assert!(!available.contains(&Intensity::Intense));
        assert!(!available.contains(&Intensity::Maximum));
    }

    #[test]
    fn test_by_max_difficulty() {
        let low_intensities = Intensity::by_max_difficulty(2);
        assert_eq!(low_intensities.len(), 2);
        assert!(low_intensities.contains(&Intensity::Gentle));
        assert!(low_intensities.contains(&Intensity::Moderate));
        assert!(!low_intensities.contains(&Intensity::Vigorous));
    }
}

use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwimmingError {
    #[error("Cannot swim: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for swimming: need at least {required}, have {current}")]
    InsufficientEnergyForSwimming {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Cannot dive to {requested_depth}m: maximum depth is {max_depth}m")]
    DepthLimitExceeded {
        requested_depth: u32,
        max_depth: u32,
    },
}

pub type SwimmingResult = Result<String, SwimmingError>;

/// Swimming capability - depends on Moving trait and energy
pub trait Swimming: Moving + HasEnergy {
    /// Maximum diving depth in meters - varies by implementation
    fn max_depth(&self) -> u32;

    /// Basic swimming
    fn swim(&mut self) -> SwimmingResult {
        let current_energy = self.energy();

        // Swimming requires at least Tired energy level
        let required_energy = EnergyLevel::Tired;
        if current_energy < required_energy {
            return Err(SwimmingError::InsufficientEnergyForSwimming {
                required: required_energy,
                current: current_energy,
            });
        }

        // Swimming consumes energy
        self.consume_energy();

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => Ok("Entity swims".to_string()),
            Err(movement_error) => Err(SwimmingError::MovementError(movement_error)),
        }
    }

    /// Diving to specific depth
    fn dive(&mut self, target_depth: u32) -> SwimmingResult {
        let current_energy = self.energy();

        // Check depth limits
        if target_depth > self.max_depth() {
            return Err(SwimmingError::DepthLimitExceeded {
                requested_depth: target_depth,
                max_depth: self.max_depth(),
            });
        }

        // Diving requires Normal energy level
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(SwimmingError::InsufficientEnergyForSwimming {
                required: required_energy,
                current: current_energy,
            });
        }

        // Diving consumes more energy
        self.consume_energy_levels(2);

        // Use basic movement for diving
        match self.do_move() {
            Ok(_) => Ok(format!("Entity dives to {}m depth", target_depth)),
            Err(movement_error) => Err(SwimmingError::MovementError(movement_error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestSwimmer {
        energy: EnergyLevel,
        max_depth: u32,
    }

    impl TestSwimmer {
        fn new(energy: EnergyLevel, max_depth: u32) -> Self {
            Self { energy, max_depth }
        }
    }

    impl HasEnergy for TestSwimmer {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }

        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestSwimmer {}

    impl Swimming for TestSwimmer {
        fn max_depth(&self) -> u32 {
            self.max_depth
        }
    }

    #[test]
    fn test_basic_swimming_success() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Normal, 100);

        let result = swimmer.swim();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity swims");

        // Energy should be consumed (Normal -> Exhausted after swim + move)
        assert_eq!(swimmer.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_swimming_insufficient_energy() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Exhausted, 100);

        let result = swimmer.swim();
        assert!(result.is_err());

        if let Err(SwimmingError::InsufficientEnergyForSwimming { required, current }) = result {
            assert_eq!(required, EnergyLevel::Tired);
            assert_eq!(current, EnergyLevel::Exhausted);
        } else {
            panic!("Expected InsufficientEnergyForSwimming error");
        }
    }

    #[test]
    fn test_diving_success() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Hyperactive, 100);

        let result = swimmer.dive(50);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity dives to 50m depth");

        // Energy should be consumed (dive consumes 2 + move consumes 1 = 3 total)
        assert_eq!(swimmer.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_diving_depth_limit_exceeded() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Hyperactive, 100);

        let result = swimmer.dive(150); // Exceeds max_depth of 100
        assert!(result.is_err());

        if let Err(SwimmingError::DepthLimitExceeded {
            requested_depth,
            max_depth,
        }) = result
        {
            assert_eq!(requested_depth, 150);
            assert_eq!(max_depth, 100);
        } else {
            panic!("Expected DepthLimitExceeded error");
        }
    }

    #[test]
    fn test_diving_insufficient_energy() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Tired, 100);

        let result = swimmer.dive(50);
        assert!(result.is_err());

        if let Err(SwimmingError::InsufficientEnergyForSwimming { required, current }) = result {
            assert_eq!(required, EnergyLevel::Normal);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForSwimming error");
        }
    }

    #[test]
    fn test_max_depth_values() {
        let shallow_swimmer = TestSwimmer::new(EnergyLevel::Normal, 10);
        let deep_swimmer = TestSwimmer::new(EnergyLevel::Normal, 1000);

        assert_eq!(shallow_swimmer.max_depth(), 10);
        assert_eq!(deep_swimmer.max_depth(), 1000);
        assert!(deep_swimmer.max_depth() > shallow_swimmer.max_depth());
    }

    #[test]
    fn test_diving_at_max_depth() {
        let mut swimmer = TestSwimmer::new(EnergyLevel::Hyperactive, 50);

        // Should succeed at exactly max depth
        let result = swimmer.dive(50);
        assert!(result.is_ok());

        // Reset energy for next test
        swimmer.set_energy(EnergyLevel::Hyperactive);

        // Should fail at max depth + 1
        let result = swimmer.dive(51);
        assert!(result.is_err());
    }

    #[test]
    fn test_energy_consumption_differences() {
        let mut swimmer1 = TestSwimmer::new(EnergyLevel::Energetic, 100);
        let mut swimmer2 = TestSwimmer::new(EnergyLevel::Energetic, 100);

        // Swimming consumes less energy than diving
        swimmer1.swim().unwrap();
        swimmer2.dive(50).unwrap();

        // swimmer1 should have more energy left than swimmer2
        assert!(swimmer1.energy() > swimmer2.energy());
    }
}

use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlyingError {
    #[error("Cannot fly: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for flying: need at least {required}, have {current}")]
    InsufficientEnergyForFlying {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Cannot fly to {requested_altitude}m: maximum altitude is {max_altitude}m")]
    AltitudeLimitExceeded {
        requested_altitude: u32,
        max_altitude: u32,
    },
}

pub type FlyingResult = Result<String, FlyingError>;

/// Flying capability - depends on Moving trait and energy
pub trait Flying: Moving + HasEnergy {
    /// Maximum flying altitude in meters - varies by implementation
    fn max_altitude(&self) -> u32;

    /// Basic flying
    fn fly(&mut self) -> FlyingResult {
        let current_energy = self.energy();

        // Flying requires at least Normal energy level
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Flying consumes energy
        self.consume_energy_levels(2);

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => Ok("Entity flies".to_string()),
            Err(movement_error) => Err(FlyingError::MovementError(movement_error)),
        }
    }

    /// Flying to specific altitude
    fn fly_to_altitude(&mut self, target_altitude: u32) -> FlyingResult {
        let current_energy = self.energy();

        // Check altitude limits
        if target_altitude > self.max_altitude() {
            return Err(FlyingError::AltitudeLimitExceeded {
                requested_altitude: target_altitude,
                max_altitude: self.max_altitude(),
            });
        }

        // Flying to altitude requires Energetic energy level
        let required_energy = EnergyLevel::Energetic;
        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Flying to altitude consumes more energy
        self.consume_energy_levels(3);

        // Use basic movement for the flying motion
        match self.do_move() {
            Ok(_) => Ok(format!("Entity flies to {}m altitude", target_altitude)),
            Err(movement_error) => Err(FlyingError::MovementError(movement_error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestFlyer {
        energy: EnergyLevel,
        max_altitude: u32,
    }

    impl TestFlyer {
        fn new(energy: EnergyLevel, max_altitude: u32) -> Self {
            Self {
                energy,
                max_altitude,
            }
        }
    }

    impl HasEnergy for TestFlyer {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }

        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestFlyer {}

    impl Flying for TestFlyer {
        fn max_altitude(&self) -> u32 {
            self.max_altitude
        }
    }

    #[test]
    fn test_basic_flying_success() {
        let mut flyer = TestFlyer::new(EnergyLevel::Energetic, 1000);

        let result = flyer.fly();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity flies");

        // Energy should be consumed (fly consumes 2 + move consumes 1 = 3 total)
        assert_eq!(flyer.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_flying_insufficient_energy() {
        let mut flyer = TestFlyer::new(EnergyLevel::Tired, 1000);

        let result = flyer.fly();
        assert!(result.is_err());

        if let Err(FlyingError::InsufficientEnergyForFlying { required, current }) = result {
            assert_eq!(required, EnergyLevel::Normal);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForFlying error");
        }
    }

    #[test]
    fn test_fly_to_altitude_success() {
        let mut flyer = TestFlyer::new(EnergyLevel::Hyperactive, 1000);

        let result = flyer.fly_to_altitude(500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity flies to 500m altitude");

        // Energy should be consumed (fly_to_altitude consumes 3 + move consumes 1 = 4 total)
        assert_eq!(flyer.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_altitude_limit_exceeded() {
        let mut flyer = TestFlyer::new(EnergyLevel::Hyperactive, 500);

        let result = flyer.fly_to_altitude(1000); // Exceeds max_altitude of 500
        assert!(result.is_err());

        if let Err(FlyingError::AltitudeLimitExceeded {
            requested_altitude,
            max_altitude,
        }) = result
        {
            assert_eq!(requested_altitude, 1000);
            assert_eq!(max_altitude, 500);
        } else {
            panic!("Expected AltitudeLimitExceeded error");
        }
    }

    #[test]
    fn test_fly_to_altitude_insufficient_energy() {
        let mut flyer = TestFlyer::new(EnergyLevel::Normal, 1000);

        let result = flyer.fly_to_altitude(500);
        assert!(result.is_err());

        if let Err(FlyingError::InsufficientEnergyForFlying { required, current }) = result {
            assert_eq!(required, EnergyLevel::Energetic);
            assert_eq!(current, EnergyLevel::Normal);
        } else {
            panic!("Expected InsufficientEnergyForFlying error");
        }
    }

    #[test]
    fn test_max_altitude_values() {
        let low_flyer = TestFlyer::new(EnergyLevel::Energetic, 100);
        let high_flyer = TestFlyer::new(EnergyLevel::Energetic, 5000);

        assert_eq!(low_flyer.max_altitude(), 100);
        assert_eq!(high_flyer.max_altitude(), 5000);
        assert!(high_flyer.max_altitude() > low_flyer.max_altitude());
    }

    #[test]
    fn test_fly_to_max_altitude() {
        let mut flyer = TestFlyer::new(EnergyLevel::Hyperactive, 300);

        // Should succeed at exactly max altitude
        let result = flyer.fly_to_altitude(300);
        assert!(result.is_ok());

        // Reset energy for next test
        flyer.set_energy(EnergyLevel::Hyperactive);

        // Should fail at max altitude + 1
        let result = flyer.fly_to_altitude(301);
        assert!(result.is_err());
    }

    #[test]
    fn test_energy_consumption_differences() {
        let mut flyer1 = TestFlyer::new(EnergyLevel::Hyperactive, 1000);
        let mut flyer2 = TestFlyer::new(EnergyLevel::Hyperactive, 1000);

        // Basic flying consumes less energy than flying to altitude
        flyer1.fly().unwrap();
        flyer2.fly_to_altitude(500).unwrap();

        // flyer1 should have more energy left than flyer2
        assert!(flyer1.energy() > flyer2.energy());
    }
}

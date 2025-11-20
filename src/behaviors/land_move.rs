//! Land Movement Trait
//!
//! This module defines the LandMove trait which serves as the base
//! for both walking and driving capabilities.

use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LandMoveError {
    #[error("Cannot move on land: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for land movement: need at least {required}, have {current}")]
    InsufficientEnergyForLandMove {
        required: EnergyLevel,
        current: EnergyLevel,
    },
}

pub type LandMoveResult = Result<String, LandMoveError>;

/// Trait for anything that can move on land
/// This is the base trait for both biological movement (walking) and mechanical movement (driving)
pub trait LandMove: Moving + HasEnergy {
    /// Basic land movement
    fn land_move(&mut self) -> LandMoveResult {
        let current_energy = self.energy();

        // Land movement requires at least Exhausted energy level
        let required_energy = EnergyLevel::Exhausted;
        if current_energy < required_energy {
            return Err(LandMoveError::InsufficientEnergyForLandMove {
                required: required_energy,
                current: current_energy,
            });
        }

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => Ok("Entity moves on land".to_string()),
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }

    /// Fast land movement
    fn land_move_fast(&mut self) -> LandMoveResult {
        let current_energy = self.energy();

        // Fast movement requires at least Normal energy level
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(LandMoveError::InsufficientEnergyForLandMove {
                required: required_energy,
                current: current_energy,
            });
        }

        // Use basic movement as foundation first
        match self.do_move() {
            Ok(_) => {
                // Fast movement consumes extra energy after movement
                self.consume_energy();
                Ok("Entity moves fast on land".to_string())
            }
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestLandMover {
        energy: EnergyLevel,
    }

    impl TestLandMover {
        fn new(energy: EnergyLevel) -> Self {
            Self { energy }
        }
    }

    impl HasEnergy for TestLandMover {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }
        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestLandMover {}
    impl LandMove for TestLandMover {}

    #[test]
    fn test_basic_land_movement() {
        let mut mover = TestLandMover::new(EnergyLevel::Normal);

        let result = mover.land_move();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity moves on land");

        // Energy consumed by do_move (1 level)
        assert_eq!(mover.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_land_move_insufficient_energy() {
        let mut mover = TestLandMover::new(EnergyLevel::Collapsed);

        let result = mover.land_move();
        assert!(result.is_err());

        if let Err(LandMoveError::InsufficientEnergyForLandMove { required, current }) = result {
            assert_eq!(required, EnergyLevel::Exhausted);
            assert_eq!(current, EnergyLevel::Collapsed);
        } else {
            panic!("Expected InsufficientEnergyForLandMove error");
        }
    }

    #[test]
    fn test_fast_land_movement() {
        let mut mover = TestLandMover::new(EnergyLevel::Energetic);

        let result = mover.land_move_fast();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity moves fast on land");

        // Energy consumed by do_move (1) + land_move_fast (1) = 2 total
        assert_eq!(mover.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_fast_land_move_insufficient_energy() {
        let mut mover = TestLandMover::new(EnergyLevel::Tired);

        let result = mover.land_move_fast();
        assert!(result.is_err());

        if let Err(LandMoveError::InsufficientEnergyForLandMove { required, current }) = result {
            assert_eq!(required, EnergyLevel::Normal);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForLandMove error");
        }
    }

    #[test]
    fn test_energy_consumption_differences() {
        let mut mover1 = TestLandMover::new(EnergyLevel::Energetic);
        let mut mover2 = TestLandMover::new(EnergyLevel::Energetic);

        // Basic land movement consumes less energy than fast movement
        mover1.land_move().unwrap();
        mover2.land_move_fast().unwrap();

        // mover1 should have more energy left than mover2
        assert!(mover1.energy() > mover2.energy());
    }

    #[test]
    fn test_land_move_from_minimum_energy() {
        let mut mover = TestLandMover::new(EnergyLevel::Exhausted);

        let result = mover.land_move();
        // This will succeed because Exhausted meets the minimum requirement
        assert!(result.is_ok());
    }

    #[test]
    fn test_land_move_from_tired() {
        let mut mover = TestLandMover::new(EnergyLevel::Tired);

        let result = mover.land_move();
        assert!(result.is_ok());

        // Should be at Exhausted after movement
        assert_eq!(mover.energy(), EnergyLevel::Exhausted);
    }
}

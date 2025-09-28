use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovingError {
    #[error("Cannot move: entity is collapsed (energy: {current})")]
    Collapsed { current: EnergyLevel },

    #[error("Insufficient energy to move: need at least {required}, have {current}")]
    InsufficientEnergy {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Movement blocked: {reason}")]
    MovementBlocked { reason: String },
}

pub type MovingResult = Result<String, MovingError>;

/// Basic movement capability that depends on energy
pub trait Moving: HasEnergy {
    /// Perform basic movement - requires at least Exhausted energy level
    fn do_move(&mut self) -> MovingResult {
        let current_energy = self.energy();

        // Check if collapsed - cannot move at all
        if current_energy == EnergyLevel::Collapsed {
            return Err(MovingError::Collapsed {
                current: current_energy,
            });
        }

        // Check minimum energy requirement for movement
        let required_energy = EnergyLevel::Exhausted;
        if current_energy < required_energy {
            return Err(MovingError::InsufficientEnergy {
                required: required_energy,
                current: current_energy,
            });
        }

        // Movement is possible - consume energy
        self.consume_energy();

        // Return success message based on energy level
        let movement_description = match current_energy {
            EnergyLevel::Collapsed => unreachable!(), // Already handled above
            EnergyLevel::Exhausted => "moves very slowly",
            EnergyLevel::Tired => "moves cautiously",
            EnergyLevel::Normal => "moves steadily",
            EnergyLevel::Energetic => "moves with vigor",
            EnergyLevel::Hyperactive => "moves with explosive energy",
        };

        Ok(format!("Entity {}", movement_description))
    }

    /// Check if movement is currently possible
    fn can_move(&self) -> bool {
        self.energy() > EnergyLevel::Collapsed
    }

    /// Get the energy cost for basic movement
    fn movement_energy_cost(&self) -> u8 {
        1 // Basic movement costs 1 energy level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::EnergyLevel;

    // Mock entity for testing
    #[derive(Debug)]
    struct TestEntity {
        energy: EnergyLevel,
    }

    impl HasEnergy for TestEntity {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }

        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestEntity {}

    #[test]
    fn test_basic_movement_success() {
        let mut entity = TestEntity {
            energy: EnergyLevel::Normal,
        };

        let result = entity.do_move();
        assert!(result.is_ok());
        assert_eq!(entity.energy(), EnergyLevel::Tired); // Energy consumed
    }

    #[test]
    fn test_movement_when_collapsed() {
        let mut entity = TestEntity {
            energy: EnergyLevel::Collapsed,
        };

        let result = entity.do_move();
        assert!(result.is_err());

        if let Err(MovingError::Collapsed { current }) = result {
            assert_eq!(current, EnergyLevel::Collapsed);
        } else {
            panic!("Expected Collapsed error");
        }
    }

    #[test]
    fn test_can_move() {
        let entity_can_move = TestEntity {
            energy: EnergyLevel::Exhausted,
        };
        let entity_cannot_move = TestEntity {
            energy: EnergyLevel::Collapsed,
        };

        assert!(entity_can_move.can_move());
        assert!(!entity_cannot_move.can_move());
    }
}

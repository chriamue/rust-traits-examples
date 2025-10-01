use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalkingError {
    #[error("Cannot walk: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for walking: need at least {required}, have {current}")]
    InsufficientEnergyForWalking {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Terrain too difficult for current energy level: {terrain}")]
    TerrainToodifficult { terrain: String },
}

pub type WalkingResult = Result<String, WalkingError>;

/// Walking capability - depends on Moving trait and energy
pub trait Walking: Moving + HasEnergy {
    /// Basic walking - more controlled than general movement
    fn walk(&mut self) -> WalkingResult {
        let current_energy = self.energy();

        // Walking requires at least Exhausted energy level
        let required_energy = EnergyLevel::Exhausted;
        if current_energy < required_energy {
            return Err(WalkingError::InsufficientEnergyForWalking {
                required: required_energy,
                current: current_energy,
            });
        }

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let walk_description = match current_energy {
                    EnergyLevel::Collapsed => unreachable!(), // Already checked
                    EnergyLevel::Exhausted => "shuffles slowly",
                    EnergyLevel::Tired => "walks carefully",
                    EnergyLevel::Normal => "walks steadily",
                    EnergyLevel::Energetic => "walks briskly",
                    EnergyLevel::Hyperactive => "walks with spring in step",
                };

                Ok(format!("Entity {}", walk_description))
            }
            Err(movement_error) => Err(WalkingError::MovementError(movement_error)),
        }
    }

    /// Running - faster but more energy-intensive
    fn run(&mut self) -> WalkingResult {
        let current_energy = self.energy();

        // Running requires at least Normal energy level
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(WalkingError::InsufficientEnergyForWalking {
                required: required_energy,
                current: current_energy,
            });
        }

        // Running consumes extra energy beyond basic movement
        self.consume_energy(); // Extra energy cost for running

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let run_description = match current_energy {
                    EnergyLevel::Normal => "runs at moderate pace",
                    EnergyLevel::Energetic => "runs swiftly",
                    EnergyLevel::Hyperactive => "sprints at full speed",
                    _ => unreachable!(), // Already checked energy requirements
                };

                Ok(format!("Entity {}", run_description))
            }
            Err(movement_error) => Err(WalkingError::MovementError(movement_error)),
        }
    }

    /// Walk on difficult terrain - requires more energy
    fn walk_on_terrain(&mut self, terrain: &str) -> WalkingResult {
        let current_energy = self.energy();

        // Difficult terrain requires higher energy
        let required_energy = match terrain {
            "sand" | "grass" => EnergyLevel::Exhausted,
            "rocks" | "mud" => EnergyLevel::Tired,
            "steep_hill" | "rough_terrain" => EnergyLevel::Normal,
            "mountain" => EnergyLevel::Energetic,
            _ => EnergyLevel::Exhausted, // Default
        };

        if current_energy < required_energy {
            return Err(WalkingError::TerrainToodifficult {
                terrain: terrain.to_string(),
            });
        }

        // Terrain walking may consume extra energy
        let extra_energy_cost = match terrain {
            "rocks" | "mud" | "steep_hill" => 1,
            "mountain" => 2,
            _ => 0,
        };

        if extra_energy_cost > 0 {
            self.consume_energy_levels(extra_energy_cost);
        }

        // Use basic walking as foundation
        match self.walk() {
            Ok(_) => Ok(format!("Entity walks across {}", terrain)),
            Err(walking_error) => Err(walking_error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;

    #[derive(Debug)]
    struct TestWalker {
        energy: EnergyLevel,
    }

    impl HasEnergy for TestWalker {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }
        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestWalker {}
    impl Walking for TestWalker {}

    #[test]
    fn test_basic_walking() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Normal,
        };

        let result = walker.walk();
        assert!(result.is_ok());
        assert_eq!(walker.energy(), EnergyLevel::Tired); // Energy consumed by basic movement
    }

    #[test]
    fn test_running_requires_more_energy() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Normal,
        };

        let result = walker.run();
        assert!(result.is_ok());
        // Should consume 2 energy levels (1 for run + 1 for basic movement)
        assert_eq!(walker.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_terrain_walking() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Energetic,
        };

        let result = walker.walk_on_terrain("mountain");
        assert!(result.is_ok());
        assert_eq!(walker.energy(), EnergyLevel::Exhausted);
    }
}

//! Land Movement Trait
//!
//! This module defines the LandMove trait which serves as an abstraction
//! over both walking and driving capabilities.

use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy, Intensity, Terrain};
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

    #[error("Terrain too challenging: {terrain}")]
    TerrainToodifficult { terrain: String },
}

pub type LandMoveResult = Result<String, LandMoveError>;

/// Trait for anything that can move on land
/// This includes both biological movement (walking) and mechanical movement (driving)
pub trait LandMove: Moving + HasEnergy {
    /// Basic land movement - mirrors walk()
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
            Ok(_) => {
                let move_description = match current_energy {
                    EnergyLevel::Collapsed => unreachable!(),
                    EnergyLevel::Exhausted => "moves slowly on land",
                    EnergyLevel::Tired => "moves carefully on land",
                    EnergyLevel::Normal => "moves steadily on land",
                    EnergyLevel::Energetic => "moves confidently on land",
                    EnergyLevel::Hyperactive => "moves with great vigor on land",
                };

                Ok(format!("Entity {}", move_description))
            }
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }

    /// Fast land movement - mirrors run()
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

        // Fast movement consumes extra energy beyond basic movement
        self.consume_energy(); // Extra energy cost for speed

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let fast_description = match current_energy {
                    EnergyLevel::Normal => "moves fast at moderate pace on land",
                    EnergyLevel::Energetic => "moves swiftly on land",
                    EnergyLevel::Hyperactive => "moves at maximum speed on land",
                    _ => unreachable!(), // Already checked energy requirements
                };

                Ok(format!("Entity {}", fast_description))
            }
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }

    /// Navigate different terrain types - now uses Terrain enum
    fn navigate_terrain(&mut self, terrain: Terrain) -> LandMoveResult {
        let current_energy = self.energy();
        let required_energy = terrain.required_energy_level();

        if current_energy < required_energy {
            return Err(LandMoveError::TerrainToodifficult {
                terrain: format!(
                    "Terrain '{}' too difficult for current energy: {}",
                    terrain, current_energy
                ),
            });
        }

        let terrain_energy_cost = terrain.energy_cost();
        if terrain_energy_cost > 0 {
            self.consume_energy_levels(terrain_energy_cost);
        }

        match self.land_move() {
            Ok(_) => Ok(format!("Entity successfully navigates {} terrain", terrain)),
            Err(land_move_error) => Err(land_move_error),
        }
    }

    /// Land movement at specific intensity level - now uses Intensity enum
    fn land_move_at_intensity(&mut self, intensity: Intensity) -> LandMoveResult {
        let current_energy = self.energy();
        let required_energy = intensity.required_energy_level();

        if current_energy < required_energy {
            return Err(LandMoveError::InsufficientEnergyForLandMove {
                required: required_energy,
                current: current_energy,
            });
        }

        let intensity_energy_cost = intensity.energy_cost();
        if intensity_energy_cost > 0 {
            self.consume_energy_levels(intensity_energy_cost);
        }

        // Use basic movement for the motion
        match self.do_move() {
            Ok(_) => Ok(format!(
                "Entity moves on land at {} intensity ({})",
                intensity,
                intensity.description()
            )),
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }
}

/// Helper function for unified land movement
pub fn land_move_any<T: LandMove>(mut mover: T) -> LandMoveResult {
    mover.land_move()
}

use crate::animals::{Dog, Duck, Eagle, Penguin};
use crate::vehicles::{Airplane, Car, Motorcycle};

// LandMove implementations for animals (via walking)
impl LandMove for Dog {}
impl LandMove for Duck {}
impl LandMove for Eagle {}
impl LandMove for Penguin {}

// LandMove implementations for vehicles (via driving)
impl LandMove for Car {}
impl LandMove for Motorcycle {}
impl LandMove for Airplane {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;

    #[derive(Debug)]
    struct TestLandMover {
        energy: EnergyLevel,
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
        let mut mover = TestLandMover {
            energy: EnergyLevel::Normal,
        };

        let result = mover.land_move();
        assert!(result.is_ok());
        assert_eq!(mover.energy(), EnergyLevel::Tired); // Energy consumed by basic movement
    }

    #[test]
    fn test_fast_land_movement() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Normal,
        };

        let result = mover.land_move_fast();
        assert!(result.is_ok());
        // Should consume 2 energy levels (1 for fast + 1 for basic movement)
        assert_eq!(mover.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_terrain_navigation() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Hyperactive, // Start with maximum energy
        };

        let result = mover.navigate_terrain(Terrain::Extreme);
        assert!(result.is_ok());
        // Extreme terrain: 3 energy for terrain + 1 for land_move = 4 total
        // Hyperactive (5) -> 4 consumed = Exhausted (1)
        assert_eq!(mover.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_intensity_movement() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Hyperactive, // Start with maximum energy
        };

        let result = mover.land_move_at_intensity(Intensity::Intense);
        assert!(result.is_ok());
        // intense: 2 energy for intensity + 1 for do_move = 3 total
        // Hyperactive (5) -> 3 consumed = Tired (2)
        assert_eq!(mover.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_terrain_navigation_insufficient_energy() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Normal, // Not enough for extreme terrain
        };

        let result = mover.navigate_terrain(Terrain::Extreme);
        assert!(result.is_err());
        // Should match the TerrainToodifficult error
        if let Err(LandMoveError::TerrainToodifficult { terrain }) = result {
            assert!(terrain.contains("Extreme")); // Updated to match Display format
            assert!(terrain.contains("too difficult"));
        } else {
            panic!("Expected TerrainToodifficult error");
        }
    }

    #[test]
    fn test_intensity_insufficient_energy() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Tired, // Not enough for intense movement
        };

        let result = mover.land_move_at_intensity(Intensity::Intense);
        assert!(result.is_err());
        // Should match the InsufficientEnergyForLandMove error
        if let Err(LandMoveError::InsufficientEnergyForLandMove { required, current }) = result {
            assert_eq!(required, EnergyLevel::Energetic);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForLandMove error");
        }
    }

    #[test]
    fn test_various_terrains() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Hyperactive,
        };

        // Test easy terrain
        let result = mover.navigate_terrain(Terrain::Road);
        assert!(result.is_ok());

        // Reset energy
        mover.set_energy(EnergyLevel::Hyperactive);

        // Test moderate terrain
        let result = mover.navigate_terrain(Terrain::Rocky);
        assert!(result.is_ok());

        // Reset energy
        mover.set_energy(EnergyLevel::Hyperactive);

        // Test difficult terrain
        let result = mover.navigate_terrain(Terrain::Mountain);
        assert!(result.is_ok());
    }

    #[test]
    fn test_various_intensities() {
        let mut mover = TestLandMover {
            energy: EnergyLevel::Hyperactive,
        };

        // Test gentle intensity
        let result = mover.land_move_at_intensity(Intensity::Gentle);
        assert!(result.is_ok());

        // Reset energy
        mover.set_energy(EnergyLevel::Hyperactive);

        // Test vigorous intensity
        let result = mover.land_move_at_intensity(Intensity::Vigorous);
        assert!(result.is_ok());

        // Reset energy
        mover.set_energy(EnergyLevel::Hyperactive);

        // Test maximum intensity
        let result = mover.land_move_at_intensity(Intensity::Maximum);
        assert!(result.is_ok());
    }

    #[test]
    fn test_intensity_energy_calculation() {
        // Test that different intensities consume the expected energy
        let mut mover = TestLandMover {
            energy: EnergyLevel::Hyperactive,
        };

        // Gentle should only consume base movement energy (1)
        let result = mover.land_move_at_intensity(Intensity::Gentle);
        assert!(result.is_ok());
        assert_eq!(mover.energy(), EnergyLevel::Energetic); // 5 -> 4

        // Reset and test vigorous (1 extra + 1 base = 2 total)
        mover.set_energy(EnergyLevel::Hyperactive);
        let result = mover.land_move_at_intensity(Intensity::Vigorous);
        assert!(result.is_ok());
        assert_eq!(mover.energy(), EnergyLevel::Normal); // 5 -> 3
    }
}

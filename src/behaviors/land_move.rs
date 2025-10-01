//! Land Movement Trait
//!
//! This module defines the LandMove trait which serves as an abstraction
//! over both walking and driving capabilities using sealed traits.

use crate::animals::Animal;
use crate::behaviors::driving::Driving;
use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;
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

// Sealed trait pattern to prevent overlapping implementations
mod sealed {
    pub trait Sealed {}
}

/// Trait for anything that can move on land
/// This includes both biological movement (walking) and mechanical movement (driving)
pub trait LandMove: Moving + HasEnergy + sealed::Sealed {
    /// Maximum speed on land (km/h for vehicles, relative speed for animals)
    fn max_land_speed(&self) -> u32;

    /// Efficiency of land movement (distance per energy unit)
    fn land_efficiency(&self) -> u32 {
        50 // Default efficiency
    }

    /// Get the name of this land mover
    fn land_mover_name(&self) -> String;

    /// Get the type/category of this land mover
    fn land_mover_type(&self) -> String;

    /// Perform basic land movement
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

                Ok(format!("{} {}", self.land_mover_name(), move_description))
            }
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }

    /// Move at specific speed on land
    fn land_move_at_speed(&mut self, target_speed: u32) -> LandMoveResult {
        let current_energy = self.energy();
        let max_speed = self.max_land_speed();

        if target_speed > max_speed {
            return Err(LandMoveError::TerrainToodifficult {
                terrain: format!(
                    "Speed {} km/h exceeds maximum {} km/h",
                    target_speed, max_speed
                ),
            });
        }

        // Speed affects energy requirements
        let required_energy = match target_speed {
            0..=20 => EnergyLevel::Exhausted,
            21..=50 => EnergyLevel::Tired,
            51..=100 => EnergyLevel::Normal,
            101..=150 => EnergyLevel::Energetic,
            _ => EnergyLevel::Hyperactive,
        };

        if current_energy < required_energy {
            return Err(LandMoveError::InsufficientEnergyForLandMove {
                required: required_energy,
                current: current_energy,
            });
        }

        // Calculate energy cost based on speed
        let speed_energy_cost = match target_speed {
            0..=20 => 1,
            21..=50 => 1,
            51..=100 => 2,
            101..=150 => 3,
            _ => 4,
        };

        self.consume_energy_levels(speed_energy_cost);

        match self.do_move() {
            Ok(_) => Ok(format!(
                "{} moves on land at {} km/h",
                self.land_mover_name(),
                target_speed
            )),
            Err(movement_error) => Err(LandMoveError::MovementError(movement_error)),
        }
    }

    /// Navigate different terrain types
    fn navigate_terrain(&mut self, terrain: &str) -> LandMoveResult {
        let current_energy = self.energy();

        let required_energy = match terrain {
            "road" | "pavement" => EnergyLevel::Exhausted,
            "grass" | "dirt" => EnergyLevel::Tired,
            "rocky" | "sandy" => EnergyLevel::Normal,
            "steep" | "muddy" => EnergyLevel::Energetic,
            "extreme" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };

        if current_energy < required_energy {
            return Err(LandMoveError::TerrainToodifficult {
                terrain: format!(
                    "Terrain '{}' too difficult for current energy: {}",
                    terrain, current_energy
                ),
            });
        }

        let terrain_energy_cost = match terrain {
            "road" | "pavement" => 0,
            "grass" | "dirt" => 1,
            "rocky" | "sandy" => 2,
            "steep" | "muddy" => 3,
            "extreme" => 4,
            _ => 1,
        };

        if terrain_energy_cost > 0 {
            self.consume_energy_levels(terrain_energy_cost);
        }

        match self.land_move() {
            Ok(_) => Ok(format!(
                "{} successfully navigates {} terrain",
                self.land_mover_name(),
                terrain
            )),
            Err(land_move_error) => Err(land_move_error),
        }
    }
}

// Import concrete types to implement sealed trait for them
use crate::animals::{Dog, Duck, Eagle, Penguin};
use crate::vehicles::{
    airplane::Airplane, amphibious::AmphibiousVehicle, car::Car, motorcycle::Motorcycle,
};

// Seal specific walking animals
impl sealed::Sealed for Dog {}
impl sealed::Sealed for Duck {}
impl sealed::Sealed for Eagle {}
impl sealed::Sealed for Penguin {}

// Seal specific driving vehicles
impl sealed::Sealed for Car {}
impl sealed::Sealed for Motorcycle {}
impl sealed::Sealed for Airplane {}
impl sealed::Sealed for AmphibiousVehicle {}

// Note: For types that can both walk AND drive (like AmphibiousVehicle),
// we need to decide which behavior to prioritize for LandMove.
// Here we'll prioritize driving for vehicles.

// LandMove implementation for walking animals
impl LandMove for Dog {
    fn max_land_speed(&self) -> u32 {
        match self.energy() {
            EnergyLevel::Collapsed => 0,
            EnergyLevel::Exhausted => 3,
            EnergyLevel::Tired => 5,
            EnergyLevel::Normal => 8,
            EnergyLevel::Energetic => 12,
            EnergyLevel::Hyperactive => 15,
        }
    }

    fn land_efficiency(&self) -> u32 {
        100
    }

    fn land_mover_name(&self) -> String {
        Animal::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (walking)", Animal::species(self))
    }
}

impl LandMove for Duck {
    fn max_land_speed(&self) -> u32 {
        match self.energy() {
            EnergyLevel::Collapsed => 0,
            EnergyLevel::Exhausted => 2,
            EnergyLevel::Tired => 4,
            EnergyLevel::Normal => 6,
            EnergyLevel::Energetic => 8,
            EnergyLevel::Hyperactive => 10,
        }
    }

    fn land_efficiency(&self) -> u32 {
        100
    }

    fn land_mover_name(&self) -> String {
        Animal::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (walking)", Animal::species(self))
    }
}

impl LandMove for Eagle {
    fn max_land_speed(&self) -> u32 {
        match self.energy() {
            EnergyLevel::Collapsed => 0,
            EnergyLevel::Exhausted => 2,
            EnergyLevel::Tired => 3,
            EnergyLevel::Normal => 5,
            EnergyLevel::Energetic => 7,
            EnergyLevel::Hyperactive => 10,
        }
    }

    fn land_efficiency(&self) -> u32 {
        100
    }

    fn land_mover_name(&self) -> String {
        Animal::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (walking)", Animal::species(self))
    }
}

impl LandMove for Penguin {
    fn max_land_speed(&self) -> u32 {
        match self.energy() {
            EnergyLevel::Collapsed => 0,
            EnergyLevel::Exhausted => 2,
            EnergyLevel::Tired => 3,
            EnergyLevel::Normal => 5,
            EnergyLevel::Energetic => 8,
            EnergyLevel::Hyperactive => 12,
        }
    }

    fn land_efficiency(&self) -> u32 {
        100
    }

    fn land_mover_name(&self) -> String {
        Animal::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (walking)", Animal::species(self))
    }
}

// LandMove implementation for driving vehicles
impl LandMove for Car {
    fn max_land_speed(&self) -> u32 {
        Driving::max_speed(self)
    }

    fn land_efficiency(&self) -> u32 {
        Driving::fuel_efficiency(self)
    }

    fn land_mover_name(&self) -> String {
        Vehicle::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (driving)", Vehicle::vehicle_type(self))
    }
}

impl LandMove for Motorcycle {
    fn max_land_speed(&self) -> u32 {
        Driving::max_speed(self)
    }

    fn land_efficiency(&self) -> u32 {
        Driving::fuel_efficiency(self)
    }

    fn land_mover_name(&self) -> String {
        Vehicle::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (driving)", Vehicle::vehicle_type(self))
    }
}

impl LandMove for Airplane {
    fn max_land_speed(&self) -> u32 {
        Driving::max_speed(self) // Taxiing speed
    }

    fn land_efficiency(&self) -> u32 {
        Driving::fuel_efficiency(self)
    }

    fn land_mover_name(&self) -> String {
        Vehicle::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (taxiing)", Vehicle::vehicle_type(self))
    }
}

impl LandMove for AmphibiousVehicle {
    fn max_land_speed(&self) -> u32 {
        Driving::max_speed(self) // Use driving speed on land
    }

    fn land_efficiency(&self) -> u32 {
        Driving::fuel_efficiency(self)
    }

    fn land_mover_name(&self) -> String {
        Vehicle::name(self)
    }

    fn land_mover_type(&self) -> String {
        format!("{} (driving)", Vehicle::vehicle_type(self))
    }
}

/// Simplified enum for heterogeneous collections of land movers
pub enum LandMover {
    Any(Box<dyn LandMove>),
}

impl LandMover {
    /// Create from any walking entity
    pub fn from_walker<T>(walker: T) -> Self
    where
        T: LandMove + 'static,
    {
        LandMover::Any(Box::new(walker))
    }

    /// Create from any driving entity
    pub fn from_driver<T>(driver: T) -> Self
    where
        T: LandMove + 'static,
    {
        LandMover::Any(Box::new(driver))
    }

    /// Create from any land moving entity (unified constructor)
    pub fn new<T>(land_mover: T) -> Self
    where
        T: LandMove + 'static,
    {
        LandMover::Any(Box::new(land_mover))
    }

    pub fn name(&self) -> String {
        match self {
            LandMover::Any(m) => m.land_mover_name(),
        }
    }

    pub fn entity_type(&self) -> String {
        match self {
            LandMover::Any(m) => m.land_mover_type(),
        }
    }
}

// Seal and implement for LandMover enum itself
impl sealed::Sealed for LandMover {}

impl HasEnergy for LandMover {
    fn energy(&self) -> EnergyLevel {
        match self {
            LandMover::Any(m) => m.energy(),
        }
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        match self {
            LandMover::Any(m) => m.set_energy(level),
        }
    }
}

impl Moving for LandMover {}

impl LandMove for LandMover {
    fn max_land_speed(&self) -> u32 {
        match self {
            LandMover::Any(m) => m.max_land_speed(),
        }
    }

    fn land_efficiency(&self) -> u32 {
        match self {
            LandMover::Any(m) => m.land_efficiency(),
        }
    }

    fn land_mover_name(&self) -> String {
        match self {
            LandMover::Any(m) => m.land_mover_name(),
        }
    }

    fn land_mover_type(&self) -> String {
        match self {
            LandMover::Any(m) => m.land_mover_type(),
        }
    }

    fn land_move(&mut self) -> LandMoveResult {
        match self {
            LandMover::Any(m) => m.land_move(),
        }
    }
}

/// Helper function for unified land movement
pub fn land_move_any<T: LandMove>(mut mover: T) -> LandMoveResult {
    mover.land_move()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicles::car::EngineType;

    #[test]
    fn test_land_move_with_animals() {
        let mut dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
        dog.set_energy(EnergyLevel::Normal);

        let result = dog.land_move();
        assert!(result.is_ok());
        assert!(dog.energy() < EnergyLevel::Normal);
        assert_eq!(dog.max_land_speed(), 5); // After energy consumption
    }

    #[test]
    fn test_land_move_with_vehicles() {
        let mut car = Car::new(
            "Test Car".to_string(),
            "Test Motors".to_string(),
            2023,
            EngineType::Gasoline {
                cylinders: 4,
                displacement: 2.0,
            },
        );
        car.set_energy(EnergyLevel::Normal);

        let result = car.land_move();
        assert!(result.is_ok());
        assert!(car.max_land_speed() > 100);
        assert_eq!(car.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_unified_function() {
        let mut dog = Dog::new("Rex".to_string(), "German Shepherd".to_string());
        dog.set_energy(EnergyLevel::Energetic);

        let mut car = Car::new(
            "Speedster".to_string(),
            "FastCars Inc".to_string(),
            2023,
            EngineType::Electric {
                battery_capacity: 75,
            },
        );
        car.set_energy(EnergyLevel::Energetic);

        // Both can be passed to the same function!
        let dog_result = land_move_any(dog);
        let car_result = land_move_any(car);

        assert!(dog_result.is_ok());
        assert!(car_result.is_ok());
    }

    #[test]
    fn test_land_mover_enum() {
        let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
        let car = Car::new(
            "Fast Car".to_string(),
            "Speed Motors".to_string(),
            2023,
            EngineType::Gasoline {
                cylinders: 6,
                displacement: 3.0,
            },
        );

        let mut walker = LandMover::from_walker(dog);
        let mut driver = LandMover::from_driver(car);

        walker.set_energy(EnergyLevel::Energetic);
        driver.set_energy(EnergyLevel::Energetic);

        assert_eq!(walker.name(), "Buddy");
        assert_eq!(driver.name(), "Fast Car");
        assert!(walker.entity_type().contains("walking"));
        assert!(driver.entity_type().contains("driving"));

        let walk_result = walker.land_move();
        let drive_result = driver.land_move();

        assert!(walk_result.is_ok());
        assert!(drive_result.is_ok());
    }
}

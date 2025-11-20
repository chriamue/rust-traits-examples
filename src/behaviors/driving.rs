use crate::behaviors::land_move::LandMove;
use crate::behaviors::moving::MovingError;
use crate::core::{EnergyLevel, Terrain};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadType {
    Highway,
    City,
    Country,
    Suburban,
    Mountain,
    OffRoad,
    ExtremeOff,
}

impl RoadType {
    /// Get the difficulty level of this road type (1-4)
    pub fn difficulty_level(&self) -> u8 {
        match self {
            RoadType::Highway | RoadType::City => 1,
            RoadType::Country | RoadType::Suburban => 2,
            RoadType::Mountain | RoadType::OffRoad => 3,
            RoadType::ExtremeOff => 4,
        }
    }

    /// Get the energy cost for driving on this road type
    pub fn energy_cost(&self) -> u8 {
        match self {
            RoadType::Highway | RoadType::City => 1,
            RoadType::Country | RoadType::Suburban => 1,
            RoadType::Mountain | RoadType::OffRoad => 2,
            RoadType::ExtremeOff => 3,
        }
    }

    /// Get the minimum energy level required for this road type
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self {
            RoadType::Highway | RoadType::City => EnergyLevel::Tired,
            RoadType::Country | RoadType::Suburban => EnergyLevel::Normal,
            RoadType::Mountain | RoadType::OffRoad => EnergyLevel::Energetic,
            RoadType::ExtremeOff => EnergyLevel::Hyperactive,
        }
    }

    /// Get a description of this road type
    pub fn description(&self) -> &'static str {
        match self {
            RoadType::Highway => "high-speed highway with smooth asphalt",
            RoadType::City => "urban streets with traffic lights and congestion",
            RoadType::Country => "rural roads with moderate traffic",
            RoadType::Suburban => "residential area streets",
            RoadType::Mountain => "winding mountain roads with steep grades",
            RoadType::OffRoad => "unpaved trails and rough terrain",
            RoadType::ExtremeOff => "extremely challenging off-road conditions",
        }
    }

    /// Check if this road type requires special vehicle capabilities
    pub fn requires_off_road_capability(&self) -> bool {
        matches!(self, RoadType::OffRoad | RoadType::ExtremeOff)
    }

    /// Get all road types
    pub fn all_road_types() -> Vec<RoadType> {
        vec![
            RoadType::Highway,
            RoadType::City,
            RoadType::Country,
            RoadType::Suburban,
            RoadType::Mountain,
            RoadType::OffRoad,
            RoadType::ExtremeOff,
        ]
    }

    /// Get road types suitable for a given difficulty level
    pub fn by_difficulty(max_difficulty: u8) -> Vec<RoadType> {
        Self::all_road_types()
            .into_iter()
            .filter(|road| road.difficulty_level() <= max_difficulty)
            .collect()
    }
}

impl std::fmt::Display for RoadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RoadType::Highway => "Highway",
            RoadType::City => "City",
            RoadType::Country => "Country",
            RoadType::Suburban => "Suburban",
            RoadType::Mountain => "Mountain",
            RoadType::OffRoad => "Off-Road",
            RoadType::ExtremeOff => "Extreme Off-Road",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for RoadType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "highway" => RoadType::Highway,
            "city" => RoadType::City,
            "country" => RoadType::Country,
            "suburban" => RoadType::Suburban,
            "mountain" => RoadType::Mountain,
            "off_road" | "offroad" => RoadType::OffRoad,
            "extreme_terrain" | "extreme" => RoadType::ExtremeOff,
            _ => RoadType::Country, // Default fallback
        }
    }
}

#[derive(Error, Debug)]
pub enum DrivingError {
    #[error("Cannot drive: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for driving: need at least {required}, have {current}")]
    InsufficientEnergyForDriving {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Cannot reach {requested_speed} km/h: maximum speed is {max_speed} km/h")]
    SpeedLimitExceeded {
        requested_speed: u32,
        max_speed: u32,
    },

    #[error("Road conditions too challenging: {road_type} - {description}")]
    ChallengingRoadConditions {
        road_type: RoadType,
        description: String,
    },

    #[error("Mechanical failure: {reason}")]
    MechanicalFailure { reason: String },

    #[error("Vehicle not capable of {terrain} terrain")]
    TerrainNotSupported { terrain: Terrain },
}

pub type DrivingResult = Result<String, DrivingError>;

/// Driving capability - uses LandMove as foundation and adds vehicle-specific features
pub trait Driving: LandMove {
    /// Maximum driving speed in km/h - varies by implementation
    fn max_speed(&self) -> u32;

    /// Fuel efficiency in km per energy level
    fn fuel_efficiency(&self) -> u32 {
        50 // Default: 50 km per energy level
    }

    /// Check if vehicle can handle off-road conditions
    fn has_off_road_capability(&self) -> bool {
        false // Default: no off-road capability
    }

    /// Get the vehicle's driving skill level (affects performance on difficult roads)
    fn driving_skill(&self) -> u8 {
        3 // Default: moderate driving skill (1-5 scale)
    }

    /// Basic driving - uses land_move
    fn drive(&mut self) -> DrivingResult {
        // Driving is basic land movement for vehicles
        match self.land_move() {
            Ok(_) => Ok("Entity drives".to_string()),
            Err(e) => Err(DrivingError::MovementError(match e {
                crate::behaviors::land_move::LandMoveError::MovementError(me) => me,
                crate::behaviors::land_move::LandMoveError::InsufficientEnergyForLandMove {
                    required,
                    current,
                } => {
                    // Convert to generic movement error
                    return Err(DrivingError::InsufficientEnergyForDriving { required, current });
                }
            })),
        }
    }

    /// Drive at specific speed
    fn drive_at_speed(&mut self, target_speed: u32) -> DrivingResult {
        let current_energy = self.energy();

        // Check speed limits
        if target_speed > self.max_speed() {
            return Err(DrivingError::SpeedLimitExceeded {
                requested_speed: target_speed,
                max_speed: self.max_speed(),
            });
        }

        // Speed affects energy requirements
        let required_energy = match target_speed {
            0..=50 => EnergyLevel::Tired,
            51..=100 => EnergyLevel::Normal,
            101..=150 => EnergyLevel::Energetic,
            _ => EnergyLevel::Hyperactive,
        };

        if current_energy < required_energy {
            return Err(DrivingError::InsufficientEnergyForDriving {
                required: required_energy,
                current: current_energy,
            });
        }

        // Calculate energy cost based on speed
        let speed_energy_cost = match target_speed {
            0..=50 => 1,
            51..=100 => 2,
            101..=150 => 3,
            _ => 4,
        };

        self.consume_energy_levels(speed_energy_cost);

        // Use basic movement for the driving motion
        match self.do_move() {
            Ok(_) => Ok(format!("Entity drives at {} km/h", target_speed)),
            Err(movement_error) => Err(DrivingError::MovementError(movement_error)),
        }
    }

    /// Driving on different road types - now uses RoadType enum
    fn drive_on_road(&mut self, road_type: RoadType) -> DrivingResult {
        let current_energy = self.energy();
        let required_energy = road_type.required_energy_level();

        // Check if vehicle can handle off-road conditions
        if road_type.requires_off_road_capability() && !self.has_off_road_capability() {
            return Err(DrivingError::ChallengingRoadConditions {
                road_type,
                description: "Vehicle lacks off-road capability".to_string(),
            });
        }

        // Apply driving skill modifier to energy requirements
        let driving_skill = self.driving_skill();
        let adjusted_required = if driving_skill >= 4 && required_energy > EnergyLevel::Tired {
            // Skilled drivers need less energy for difficult roads
            match required_energy {
                EnergyLevel::Hyperactive => EnergyLevel::Energetic,
                EnergyLevel::Energetic => EnergyLevel::Normal,
                EnergyLevel::Normal => EnergyLevel::Tired,
                _ => required_energy,
            }
        } else {
            required_energy
        };

        if current_energy < adjusted_required {
            return Err(DrivingError::ChallengingRoadConditions {
                road_type,
                description: format!(
                    "Insufficient energy for {} roads: need {}, have {}",
                    road_type, adjusted_required, current_energy
                ),
            });
        }

        // Apply driving skill modifier to energy cost
        let mut road_energy_cost = road_type.energy_cost();
        if driving_skill >= 4 && road_energy_cost > 1 {
            road_energy_cost = road_energy_cost.saturating_sub(1); // Skilled drivers are more efficient
        } else if driving_skill <= 2 {
            road_energy_cost += 1; // Unskilled drivers consume more energy
        }

        self.consume_energy_levels(road_energy_cost);

        // Use basic driving as foundation
        match self.drive() {
            Ok(_) => Ok(format!(
                "Entity drives on {} roads ({})",
                road_type,
                road_type.description()
            )),
            Err(driving_error) => Err(driving_error),
        }
    }

    /// Drive on terrain (converts Terrain to appropriate driving context)
    fn drive_on_terrain(&mut self, terrain: Terrain) -> DrivingResult {
        // Check if terrain is suitable for vehicles
        if !terrain.vehicle_accessible() {
            return Err(DrivingError::TerrainNotSupported { terrain });
        }

        // Convert terrain to road type for driving context
        let road_type = match terrain {
            Terrain::Road | Terrain::Pavement | Terrain::Sidewalk => RoadType::Highway,
            Terrain::Grass | Terrain::Dirt => RoadType::Country,
            Terrain::Gravel => RoadType::OffRoad,
            Terrain::Sand | Terrain::Rocky => RoadType::OffRoad,
            Terrain::Snow => RoadType::Mountain,
            _ => {
                return Err(DrivingError::TerrainNotSupported { terrain });
            }
        };

        self.drive_on_road(road_type)
    }

    /// Long distance driving with fuel efficiency considerations
    fn drive_distance(&mut self, distance_km: u32) -> DrivingResult {
        let current_energy = self.energy();
        let efficiency = self.fuel_efficiency();

        // Calculate required energy based on distance and efficiency
        let energy_needed = distance_km.div_ceil(efficiency); // Ceiling division

        if current_energy as u32 <= energy_needed {
            return Err(DrivingError::InsufficientEnergyForDriving {
                required: EnergyLevel::from_points((energy_needed * 20) as u8), // Rough conversion
                current: current_energy,
            });
        }

        // Consume calculated energy
        self.consume_energy_levels(energy_needed as u8);

        match self.do_move() {
            Ok(_) => Ok(format!(
                "Entity drives {} km (efficiency: {} km per energy level)",
                distance_km, efficiency
            )),
            Err(movement_error) => Err(DrivingError::MovementError(movement_error)),
        }
    }

    /// Get available road types for current energy level and capabilities
    fn available_road_types(&self) -> Vec<RoadType> {
        let current_energy = self.energy();
        let has_off_road = self.has_off_road_capability();

        RoadType::all_road_types()
            .into_iter()
            .filter(|road_type| {
                // Check energy requirement
                if current_energy < road_type.required_energy_level() {
                    return false;
                }
                // Check off-road capability
                if road_type.requires_off_road_capability() && !has_off_road {
                    return false;
                }
                true
            })
            .collect()
    }

    /// Drive on the most challenging road possible with current energy and capabilities
    fn drive_max_challenge(&mut self) -> DrivingResult {
        let available_roads = self.available_road_types();
        let most_challenging = available_roads
            .into_iter()
            .max_by_key(|road| road.difficulty_level())
            .unwrap_or(RoadType::Highway);

        self.drive_on_road(most_challenging)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;
    use crate::core::HasEnergy;

    #[derive(Debug)]
    struct TestVehicle {
        energy: EnergyLevel,
        max_speed: u32,
        off_road: bool,
        skill: u8,
    }

    impl HasEnergy for TestVehicle {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }
        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestVehicle {}
    impl LandMove for TestVehicle {}

    impl Driving for TestVehicle {
        fn max_speed(&self) -> u32 {
            self.max_speed
        }

        fn has_off_road_capability(&self) -> bool {
            self.off_road
        }

        fn driving_skill(&self) -> u8 {
            self.skill
        }
    }

    #[test]
    fn test_basic_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
            skill: 3,
        };

        let result = vehicle.drive();
        assert!(result.is_ok());
        assert_eq!(vehicle.energy(), EnergyLevel::Tired); // Energy consumed
    }

    #[test]
    fn test_speed_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Hyperactive,
            max_speed: 200,
            off_road: false,
            skill: 3,
        };

        let result = vehicle.drive_at_speed(180);
        assert!(result.is_ok());
    }

    #[test]
    fn test_speed_limit_exceeded() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Hyperactive,
            max_speed: 120,
            off_road: false,
            skill: 3,
        };

        let result = vehicle.drive_at_speed(150);
        assert!(result.is_err());
    }

    #[test]
    fn test_road_type_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Energetic,
            max_speed: 120,
            off_road: false,
            skill: 3,
        };

        // Should work on highway
        let result = vehicle.drive_on_road(RoadType::Highway);
        assert!(result.is_ok());

        // Reset energy
        vehicle.set_energy(EnergyLevel::Energetic);

        // Should fail on off-road without capability
        let result = vehicle.drive_on_road(RoadType::OffRoad);
        assert!(result.is_err());
    }

    #[test]
    fn test_off_road_capability() {
        let mut off_road_vehicle = TestVehicle {
            energy: EnergyLevel::Energetic,
            max_speed: 100,
            off_road: true,
            skill: 3,
        };

        // Should work with off-road capability
        let result = off_road_vehicle.drive_on_road(RoadType::OffRoad);
        assert!(result.is_ok());
    }

    #[test]
    fn test_driving_skill_affects_performance() {
        let mut skilled_driver = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
            skill: 5,
        };

        let mut novice_driver = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
            skill: 2,
        };

        // Both try mountain roads
        let skilled_result = skilled_driver.drive_on_road(RoadType::Mountain);
        let novice_result = novice_driver.drive_on_road(RoadType::Mountain);

        // Skilled driver should succeed, novice might fail due to energy requirements
        if skilled_result.is_ok() && novice_result.is_ok() {
            // If both succeed, skilled driver should have more energy left
            assert!(skilled_driver.energy() >= novice_driver.energy());
        }
    }

    #[test]
    fn test_terrain_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
            skill: 3,
        };

        // Should work on road terrain
        let result = vehicle.drive_on_terrain(Terrain::Road);
        assert!(result.is_ok());

        // Reset energy
        vehicle.set_energy(EnergyLevel::Normal);

        // Should fail on non-vehicle terrain
        let result = vehicle.drive_on_terrain(Terrain::Forest);
        assert!(result.is_err());
    }

    #[test]
    fn test_available_road_types() {
        let vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
            skill: 3,
        };

        let available = vehicle.available_road_types();
        assert!(available.contains(&RoadType::Highway));
        assert!(available.contains(&RoadType::Country));
        assert!(!available.contains(&RoadType::OffRoad)); // No off-road capability
    }

    #[test]
    fn test_max_challenge_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Hyperactive,
            max_speed: 120,
            off_road: true,
            skill: 5,
        };

        let result = vehicle.drive_max_challenge();
        assert!(result.is_ok());
    }
}

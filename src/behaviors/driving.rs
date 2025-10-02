use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy, Terrain};
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
    /// Get the difficulty level of this road type (1-5)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergencyManeuver {
    EmergencyBrake,
    EvasiveManeuver,
    EmergencyLaneChange,
    CollisionAvoidance,
    SpinRecovery,
    SkidControl,
}

impl EmergencyManeuver {
    /// Get the difficulty level of this maneuver (1-5)
    pub fn difficulty_level(&self) -> u8 {
        match self {
            EmergencyManeuver::EmergencyBrake => 2,
            EmergencyManeuver::EmergencyLaneChange => 2,
            EmergencyManeuver::EvasiveManeuver => 3,
            EmergencyManeuver::SkidControl => 4,
            EmergencyManeuver::CollisionAvoidance => 4,
            EmergencyManeuver::SpinRecovery => 5,
        }
    }

    /// Get the energy cost for this maneuver
    pub fn energy_cost(&self) -> u8 {
        match self {
            EmergencyManeuver::EmergencyBrake => 2,
            EmergencyManeuver::EmergencyLaneChange => 2,
            EmergencyManeuver::EvasiveManeuver => 3,
            EmergencyManeuver::SkidControl => 3,
            EmergencyManeuver::CollisionAvoidance => 4,
            EmergencyManeuver::SpinRecovery => 4,
        }
    }

    /// Get the minimum energy level required for this maneuver
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self.difficulty_level() {
            1..=2 => EnergyLevel::Normal,
            3..=4 => EnergyLevel::Energetic,
            5 => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        }
    }

    /// Get a description of this maneuver
    pub fn description(&self) -> &'static str {
        match self {
            EmergencyManeuver::EmergencyBrake => "sudden hard braking to avoid collision",
            EmergencyManeuver::EmergencyLaneChange => "quick lane change to avoid obstacle",
            EmergencyManeuver::EvasiveManeuver => "complex steering to avoid multiple hazards",
            EmergencyManeuver::CollisionAvoidance => "last-second maneuver to prevent crash",
            EmergencyManeuver::SpinRecovery => "recovering from vehicle spin or slide",
            EmergencyManeuver::SkidControl => "regaining control during skid conditions",
        }
    }

    /// Check if this is a high-risk maneuver
    pub fn is_high_risk(&self) -> bool {
        self.difficulty_level() >= 4
    }
}

impl std::fmt::Display for EmergencyManeuver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EmergencyManeuver::EmergencyBrake => "Emergency Brake",
            EmergencyManeuver::EvasiveManeuver => "Evasive Maneuver",
            EmergencyManeuver::EmergencyLaneChange => "Emergency Lane Change",
            EmergencyManeuver::CollisionAvoidance => "Collision Avoidance",
            EmergencyManeuver::SpinRecovery => "Spin Recovery",
            EmergencyManeuver::SkidControl => "Skid Control",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for EmergencyManeuver {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "emergency_brake" | "brake" => EmergencyManeuver::EmergencyBrake,
            "evasive_maneuver" | "evasive" => EmergencyManeuver::EvasiveManeuver,
            "emergency_lane_change" | "lane_change" => EmergencyManeuver::EmergencyLaneChange,
            "collision_avoidance" | "avoid_collision" => EmergencyManeuver::CollisionAvoidance,
            "spin_recovery" | "spin" => EmergencyManeuver::SpinRecovery,
            "skid_control" | "skid" => EmergencyManeuver::SkidControl,
            _ => EmergencyManeuver::EmergencyBrake, // Default fallback
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

    #[error("Emergency maneuver failed: {maneuver} - {reason}")]
    EmergencyManeuverFailed {
        maneuver: EmergencyManeuver,
        reason: String,
    },

    #[error("Mechanical failure: {reason}")]
    MechanicalFailure { reason: String },

    #[error("Vehicle not capable of {terrain} terrain")]
    TerrainNotSupported { terrain: Terrain },
}

pub type DrivingResult = Result<String, DrivingError>;

/// Driving capability - depends on Moving trait and energy
pub trait Driving: Moving + HasEnergy {
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

    /// Basic driving
    fn drive(&mut self) -> DrivingResult {
        let current_energy = self.energy();

        // Driving requires at least Tired energy level
        let required_energy = EnergyLevel::Tired;
        if current_energy < required_energy {
            return Err(DrivingError::InsufficientEnergyForDriving {
                required: required_energy,
                current: current_energy,
            });
        }

        // Driving consumes energy beyond basic movement
        self.consume_energy(); // Extra cost for driving

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let drive_description = match current_energy {
                    EnergyLevel::Tired => "drives slowly and carefully",
                    EnergyLevel::Normal => "drives at steady pace",
                    EnergyLevel::Energetic => "drives smoothly with confidence",
                    EnergyLevel::Hyperactive => "drives with impressive performance",
                    _ => unreachable!(),
                };

                Ok(format!("Entity {}", drive_description))
            }
            Err(movement_error) => Err(DrivingError::MovementError(movement_error)),
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

        if current_energy < required_energy {
            return Err(DrivingError::ChallengingRoadConditions {
                road_type,
                description: format!(
                    "Insufficient energy for {} roads: need {}, have {}",
                    road_type, required_energy, current_energy
                ),
            });
        }

        let road_energy_cost = road_type.energy_cost();
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

    /// Emergency maneuver - now uses EmergencyManeuver enum
    fn emergency_maneuver(&mut self, maneuver: EmergencyManeuver) -> DrivingResult {
        let current_energy = self.energy();
        let required_energy = maneuver.required_energy_level();

        if current_energy < required_energy {
            return Err(DrivingError::InsufficientEnergyForDriving {
                required: required_energy,
                current: current_energy,
            });
        }

        let maneuver_cost = maneuver.energy_cost();
        self.consume_energy_levels(maneuver_cost);

        // Check if maneuver was successful based on remaining energy and risk
        let success = if maneuver.is_high_risk() {
            self.energy() >= EnergyLevel::Tired // High-risk maneuvers need more remaining energy
        } else {
            self.energy() >= EnergyLevel::Exhausted
        };

        if success {
            Ok(format!(
                "Entity successfully performed {} ({})",
                maneuver,
                maneuver.description()
            ))
        } else {
            Err(DrivingError::EmergencyManeuverFailed {
                maneuver,
                reason: "Vehicle exhausted during maneuver".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;

    #[derive(Debug)]
    struct TestVehicle {
        energy: EnergyLevel,
        max_speed: u32,
        off_road: bool,
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

    impl Driving for TestVehicle {
        fn max_speed(&self) -> u32 {
            self.max_speed
        }

        fn has_off_road_capability(&self) -> bool {
            self.off_road
        }
    }

    #[test]
    fn test_basic_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
        };

        let result = vehicle.drive();
        assert!(result.is_ok());
        assert_eq!(vehicle.energy(), EnergyLevel::Exhausted); // Energy consumed
    }

    #[test]
    fn test_speed_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Hyperactive,
            max_speed: 200,
            off_road: false,
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
        };

        // Should work with off-road capability
        let result = off_road_vehicle.drive_on_road(RoadType::OffRoad);
        assert!(result.is_ok());
    }

    #[test]
    fn test_terrain_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
            off_road: false,
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
}

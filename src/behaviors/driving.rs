use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

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

    #[error("Road conditions too challenging: {conditions}")]
    ChallengingRoadConditions { conditions: String },

    #[error("Mechanical failure: {reason}")]
    MechanicalFailure { reason: String },
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

    /// Driving on different road conditions
    fn drive_on_road(&mut self, road_type: &str) -> DrivingResult {
        let current_energy = self.energy();

        // Different road conditions require different energy levels
        let required_energy = match road_type {
            "highway" | "city" => EnergyLevel::Tired,
            "country" | "suburban" => EnergyLevel::Normal,
            "mountain" | "off_road" => EnergyLevel::Energetic,
            "extreme_terrain" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };

        if current_energy < required_energy {
            return Err(DrivingError::ChallengingRoadConditions {
                conditions: format!(
                    "Road type '{}' too challenging for current energy: {}",
                    road_type, current_energy
                ),
            });
        }

        // Challenging roads consume extra energy
        let road_energy_cost = match road_type {
            "mountain" | "off_road" => 2,
            "extreme_terrain" => 3,
            _ => 1,
        };

        self.consume_energy_levels(road_energy_cost);

        // Use basic driving as foundation
        match self.drive() {
            Ok(_) => Ok(format!("Entity drives on {} roads", road_type)),
            Err(driving_error) => Err(driving_error),
        }
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

    /// Emergency maneuver - high energy cost but potentially life-saving
    fn emergency_maneuver(&mut self, maneuver_type: &str) -> DrivingResult {
        let current_energy = self.energy();

        // Emergency maneuvers require significant energy
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(DrivingError::InsufficientEnergyForDriving {
                required: required_energy,
                current: current_energy,
            });
        }

        // High energy cost for emergency maneuvers
        let maneuver_cost = match maneuver_type {
            "emergency_brake" => 2,
            "evasive_maneuver" => 3,
            "emergency_lane_change" => 2,
            "collision_avoidance" => 4,
            _ => 2,
        };

        self.consume_energy_levels(maneuver_cost);

        // Check if maneuver was successful based on remaining energy
        if self.energy() >= EnergyLevel::Exhausted {
            Ok(format!("Entity successfully performed {}", maneuver_type))
        } else {
            Err(DrivingError::MechanicalFailure {
                reason: format!("Vehicle exhausted during {}", maneuver_type),
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
    }

    #[test]
    fn test_basic_driving() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Normal,
            max_speed: 120,
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
        };

        let result = vehicle.drive_at_speed(180);
        assert!(result.is_ok());
    }

    #[test]
    fn test_speed_limit_exceeded() {
        let mut vehicle = TestVehicle {
            energy: EnergyLevel::Hyperactive,
            max_speed: 120,
        };

        let result = vehicle.drive_at_speed(150);
        assert!(result.is_err());
    }
}

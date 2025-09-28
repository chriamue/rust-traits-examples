use crate::behaviors::moving::{Moving, MovingError, MovingResult};
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

    #[error("Weather conditions prevent flying: {weather}")]
    WeatherConstraint { weather: String },
}

pub type FlyingResult = Result<String, FlyingError>;

/// Flying capability - depends on Moving trait and energy (most energy-intensive)
pub trait Flying: Moving + HasEnergy {
    /// Maximum flying altitude in meters - varies by implementation
    fn max_altitude(&self) -> u32;

    /// Basic flying
    fn fly(&mut self) -> FlyingResult {
        let current_energy = self.energy();

        // Flying requires at least Normal energy level (highest requirement)
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Flying consumes significant energy beyond basic movement
        self.consume_energy_levels(2); // High energy cost for flight

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let fly_description = match current_energy {
                    EnergyLevel::Normal => "flies steadily",
                    EnergyLevel::Energetic => "soars gracefully",
                    EnergyLevel::Hyperactive => "flies with incredible agility",
                    _ => unreachable!(),
                };

                Ok(format!("Entity {}", fly_description))
            }
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

        // High altitude flying energy requirements
        let required_energy = match target_altitude {
            0..=100 => EnergyLevel::Normal,
            101..=500 => EnergyLevel::Energetic,
            _ => EnergyLevel::Hyperactive,
        };

        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Calculate energy cost based on altitude
        let altitude_energy_cost = match target_altitude {
            0..=100 => 2,
            101..=500 => 3,
            501..=1000 => 4,
            _ => 5,
        };

        self.consume_energy_levels(altitude_energy_cost);

        // Use basic movement for the flying motion
        match self.do_move() {
            Ok(_) => Ok(format!(
                "Entity flies to {} meters altitude",
                target_altitude
            )),
            Err(movement_error) => Err(FlyingError::MovementError(movement_error)),
        }
    }

    /// Gliding - less energy intensive than active flight
    fn glide(&mut self) -> FlyingResult {
        let current_energy = self.energy();

        // Gliding requires less energy than powered flight
        let required_energy = EnergyLevel::Tired;
        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Gliding consumes minimal energy
        self.consume_energy(); // Only 1 level for gliding

        match self.do_move() {
            Ok(_) => Ok("Entity glides effortlessly".to_string()),
            Err(movement_error) => Err(FlyingError::MovementError(movement_error)),
        }
    }

    /// Flying in weather conditions
    fn fly_in_weather(&mut self, weather: &str) -> FlyingResult {
        let current_energy = self.energy();

        // Weather affects energy requirements
        let required_energy = match weather {
            "clear" | "sunny" => EnergyLevel::Normal,
            "windy" | "cloudy" => EnergyLevel::Energetic,
            "storm" | "heavy_rain" => EnergyLevel::Hyperactive,
            "hurricane" | "tornado" => {
                return Err(FlyingError::WeatherConstraint {
                    weather: weather.to_string(),
                });
            }
            _ => EnergyLevel::Normal,
        };

        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        // Weather flying consumes extra energy
        let weather_energy_cost = match weather {
            "windy" | "cloudy" => 1,
            "storm" | "heavy_rain" => 2,
            _ => 0,
        };

        if weather_energy_cost > 0 {
            self.consume_energy_levels(weather_energy_cost);
        }

        // Use basic flying as foundation
        match self.fly() {
            Ok(_) => Ok(format!("Entity flies through {} weather", weather)),
            Err(flying_error) => Err(flying_error),
        }
    }
}

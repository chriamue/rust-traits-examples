use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy, Weather};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightMode {
    Powered,  // Active flapping/engine powered flight
    Gliding,  // Passive gliding flight
    Soaring,  // Using thermals and updrafts
    Hovering, // Stationary flight (helicopters, hummingbirds)
}

impl FlightMode {
    /// Get the energy cost for this flight mode
    pub fn energy_cost(&self) -> u8 {
        match self {
            FlightMode::Powered => 2,  // High energy for active flight
            FlightMode::Gliding => 1,  // Low energy for gliding
            FlightMode::Soaring => 1,  // Low energy using thermals
            FlightMode::Hovering => 3, // Very high energy for hovering
        }
    }

    /// Get the minimum energy level required for this flight mode
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self {
            FlightMode::Powered => EnergyLevel::Normal,
            FlightMode::Gliding => EnergyLevel::Tired,
            FlightMode::Soaring => EnergyLevel::Tired,
            FlightMode::Hovering => EnergyLevel::Energetic,
        }
    }

    /// Get a description of this flight mode
    pub fn description(&self) -> &'static str {
        match self {
            FlightMode::Powered => "active powered flight with continuous wing movement",
            FlightMode::Gliding => "passive gliding using existing momentum",
            FlightMode::Soaring => "energy-efficient flight using thermals and air currents",
            FlightMode::Hovering => "stationary flight maintaining fixed position",
        }
    }
}

impl std::fmt::Display for FlightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            FlightMode::Powered => "Powered Flight",
            FlightMode::Gliding => "Gliding",
            FlightMode::Soaring => "Soaring",
            FlightMode::Hovering => "Hovering",
        };
        write!(f, "{}", name)
    }
}

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

    #[error("Weather conditions prevent flying: {weather} - {description}")]
    WeatherConstraint {
        weather: Weather,
        description: String,
    },

    #[error("Flight mode not supported: {mode}")]
    FlightModeNotSupported { mode: FlightMode },
}

pub type FlyingResult = Result<String, FlyingError>;

/// Flying capability - depends on Moving trait and energy (most energy-intensive)
pub trait Flying: Moving + HasEnergy {
    /// Maximum flying altitude in meters - varies by implementation
    fn max_altitude(&self) -> u32;

    /// Get the entity's flying skill level (affects performance in difficult weather)
    fn flying_skill(&self) -> u8 {
        3 // Default: moderate flying skill (1-5 scale)
    }

    /// Check if this flyer supports a specific flight mode
    fn supports_flight_mode(&self, mode: FlightMode) -> bool {
        match mode {
            FlightMode::Powered => true,   // All flyers can do powered flight
            FlightMode::Gliding => true,   // All flyers can glide
            FlightMode::Soaring => true,   // Most flyers can soar
            FlightMode::Hovering => false, // Only some flyers can hover
        }
    }

    /// Basic flying - now uses powered flight mode
    fn fly(&mut self) -> FlyingResult {
        self.fly_with_mode(FlightMode::Powered)
    }

    /// Fly with specific flight mode
    fn fly_with_mode(&mut self, mode: FlightMode) -> FlyingResult {
        let current_energy = self.energy();

        // Check if flight mode is supported
        if !self.supports_flight_mode(mode) {
            return Err(FlyingError::FlightModeNotSupported { mode });
        }

        let required_energy = mode.required_energy_level();
        if current_energy < required_energy {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: required_energy,
                current: current_energy,
            });
        }

        let mode_energy_cost = mode.energy_cost();
        self.consume_energy_levels(mode_energy_cost);

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let fly_description = match (current_energy, mode) {
                    (EnergyLevel::Normal, FlightMode::Powered) => {
                        "flies steadily with powered flight"
                    }
                    (EnergyLevel::Energetic, FlightMode::Powered) => {
                        "soars gracefully with strong wing beats"
                    }
                    (EnergyLevel::Hyperactive, FlightMode::Powered) => {
                        "flies with incredible agility and speed"
                    }
                    (_, FlightMode::Gliding) => "glides effortlessly through the air",
                    (_, FlightMode::Soaring) => "soars using thermal currents",
                    (_, FlightMode::Hovering) => "hovers in place with precise control",
                    _ => "flies through the air",
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
        self.fly_with_mode(FlightMode::Gliding)
    }

    /// Soaring using thermals
    fn soar(&mut self) -> FlyingResult {
        self.fly_with_mode(FlightMode::Soaring)
    }

    /// Hovering (if supported)
    fn hover(&mut self) -> FlyingResult {
        self.fly_with_mode(FlightMode::Hovering)
    }

    /// Flying in weather conditions - now uses Weather enum
    fn fly_in_weather(&mut self, weather: Weather) -> FlyingResult {
        let current_energy = self.energy();

        // Check if weather is safe for flying
        if !weather.is_safe_for_flying() {
            return Err(FlyingError::WeatherConstraint {
                weather,
                description: format!("Flying prohibited in {} conditions", weather),
            });
        }

        let required_energy = weather.required_energy_level();

        // Apply flying skill modifier to energy requirements
        let flying_skill = self.flying_skill();
        let adjusted_required = if flying_skill >= 4 && required_energy > EnergyLevel::Normal {
            // Skilled flyers need less energy for difficult weather
            match required_energy {
                EnergyLevel::Hyperactive => EnergyLevel::Energetic,
                EnergyLevel::Energetic => EnergyLevel::Normal,
                _ => required_energy,
            }
        } else {
            required_energy
        };

        if current_energy < adjusted_required {
            return Err(FlyingError::InsufficientEnergyForFlying {
                required: adjusted_required,
                current: current_energy,
            });
        }

        // Apply flying skill modifier to energy cost
        let mut weather_energy_cost = weather.energy_cost();
        if flying_skill >= 4 && weather_energy_cost > 0 {
            weather_energy_cost = weather_energy_cost.saturating_sub(1); // Skilled flyers are more efficient
        } else if flying_skill <= 2 {
            weather_energy_cost += 1; // Unskilled flyers struggle more
        }

        if weather_energy_cost > 0 {
            self.consume_energy_levels(weather_energy_cost);
        }

        // Use basic flying as foundation
        match self.fly() {
            Ok(_) => Ok(format!(
                "Entity flies through {} weather ({})",
                weather,
                weather.description()
            )),
            Err(flying_error) => Err(flying_error),
        }
    }

    /// Get available flight modes for current energy level
    fn available_flight_modes(&self) -> Vec<FlightMode> {
        let current_energy = self.energy();
        vec![
            FlightMode::Powered,
            FlightMode::Gliding,
            FlightMode::Soaring,
            FlightMode::Hovering,
        ]
        .into_iter()
        .filter(|mode| {
            current_energy >= mode.required_energy_level() && self.supports_flight_mode(*mode)
        })
        .collect()
    }

    /// Get weather conditions suitable for current skill and energy
    fn suitable_weather_conditions(&self) -> Vec<Weather> {
        let skill_level = self.flying_skill();
        let current_energy = self.energy();

        Weather::all_weather_conditions()
            .into_iter()
            .filter(|weather| {
                weather.is_safe_for_flying() && current_energy >= weather.required_energy_level()
            })
            .filter(|weather| weather.difficulty_level() <= skill_level + 1) // Allow slightly above skill
            .collect()
    }

    /// Fly using the most efficient mode for current conditions
    fn fly_efficiently(&mut self) -> FlyingResult {
        let available_modes = self.available_flight_modes();
        let most_efficient = available_modes
            .into_iter()
            .min_by_key(|mode| mode.energy_cost())
            .unwrap_or(FlightMode::Powered);

        self.fly_with_mode(most_efficient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;

    #[derive(Debug)]
    struct TestFlyer {
        energy: EnergyLevel,
        max_altitude: u32,
        skill: u8,
        can_hover: bool,
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

        fn flying_skill(&self) -> u8 {
            self.skill
        }

        fn supports_flight_mode(&self, mode: FlightMode) -> bool {
            match mode {
                FlightMode::Hovering => self.can_hover,
                _ => true,
            }
        }
    }

    #[test]
    fn test_basic_flying() {
        let mut flyer = TestFlyer {
            energy: EnergyLevel::Normal,
            max_altitude: 1000,
            skill: 3,
            can_hover: false,
        };

        let result = flyer.fly();
        assert!(result.is_ok());
    }

    #[test]
    fn test_flight_modes() {
        let mut flyer = TestFlyer {
            energy: EnergyLevel::Energetic,
            max_altitude: 1000,
            skill: 3,
            can_hover: true,
        };

        let result = flyer.glide();
        assert!(result.is_ok());

        flyer.set_energy(EnergyLevel::Energetic);
        let result = flyer.hover();
        assert!(result.is_ok());
    }

    #[test]
    fn test_weather_flying() {
        let mut flyer = TestFlyer {
            energy: EnergyLevel::Hyperactive,
            max_altitude: 1000,
            skill: 5,
            can_hover: false,
        };

        let result = flyer.fly_in_weather(Weather::Clear);
        assert!(result.is_ok());

        flyer.set_energy(EnergyLevel::Hyperactive);
        let result = flyer.fly_in_weather(Weather::Hurricane);
        assert!(result.is_err());
    }

    #[test]
    fn test_skill_affects_weather_performance() {
        let mut skilled_flyer = TestFlyer {
            energy: EnergyLevel::Normal,
            max_altitude: 1000,
            skill: 5,
            can_hover: false,
        };

        let mut novice_flyer = TestFlyer {
            energy: EnergyLevel::Normal,
            max_altitude: 1000,
            skill: 2,
            can_hover: false,
        };

        // Both try windy weather
        let skilled_result = skilled_flyer.fly_in_weather(Weather::Windy);
        let novice_result = novice_flyer.fly_in_weather(Weather::Windy);

        // Skilled flyer should have an easier time
        if skilled_result.is_ok() && novice_result.is_ok() {
            assert!(skilled_flyer.energy() >= novice_flyer.energy());
        }
    }
}

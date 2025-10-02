use crate::core::EnergyLevel;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weather {
    // Clear conditions
    Clear,
    Sunny,

    // Light conditions
    PartlyCloudy,
    Overcast,
    LightWind,

    // Moderate conditions
    Cloudy,
    Windy,
    LightRain,
    Fog,

    // Challenging conditions
    HeavyWind,
    Rain,
    Snow,
    Hail,

    // Severe conditions
    Storm,
    HeavyRain,
    Blizzard,

    // Extreme conditions (dangerous/impossible to fly)
    Hurricane,
    Tornado,
    Thunderstorm,
}

impl Weather {
    /// Get the difficulty level of this weather condition (1-6)
    pub fn difficulty_level(&self) -> u8 {
        match self {
            // Clear (1)
            Weather::Clear | Weather::Sunny => 1,

            // Light (2)
            Weather::PartlyCloudy | Weather::Overcast | Weather::LightWind => 2,

            // Moderate (3)
            Weather::Cloudy | Weather::Windy | Weather::LightRain | Weather::Fog => 3,

            // Challenging (4)
            Weather::HeavyWind | Weather::Rain | Weather::Snow | Weather::Hail => 4,

            // Severe (5)
            Weather::Storm | Weather::HeavyRain | Weather::Blizzard => 5,

            // Extreme (6)
            Weather::Hurricane | Weather::Tornado | Weather::Thunderstorm => 6,
        }
    }

    /// Get the energy cost for flying in this weather
    pub fn energy_cost(&self) -> u8 {
        match self {
            // Clear conditions - no extra cost
            Weather::Clear | Weather::Sunny => 0,

            // Light conditions - minimal cost
            Weather::PartlyCloudy | Weather::Overcast | Weather::LightWind => 0,

            // Moderate conditions - some extra cost
            Weather::Cloudy | Weather::Windy | Weather::LightRain | Weather::Fog => 1,

            // Challenging conditions - significant cost
            Weather::HeavyWind | Weather::Rain | Weather::Snow | Weather::Hail => 2,

            // Severe conditions - high cost
            Weather::Storm | Weather::HeavyRain | Weather::Blizzard => 3,

            // Extreme conditions - flying not recommended
            Weather::Hurricane | Weather::Tornado | Weather::Thunderstorm => 5,
        }
    }

    /// Get the minimum energy level required to fly in this weather
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self.difficulty_level() {
            1..=2 => EnergyLevel::Normal,
            3..=4 => EnergyLevel::Energetic,
            5..=6 => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        }
    }

    /// Get a description of this weather condition
    pub fn description(&self) -> &'static str {
        match self {
            Weather::Clear => "clear skies with excellent visibility",
            Weather::Sunny => "bright sunny conditions with calm air",
            Weather::PartlyCloudy => "partly cloudy with good visibility",
            Weather::Overcast => "overcast skies with adequate visibility",
            Weather::LightWind => "light winds with minor turbulence",
            Weather::Cloudy => "cloudy conditions with reduced visibility",
            Weather::Windy => "windy conditions with moderate turbulence",
            Weather::LightRain => "light rain with decreased visibility",
            Weather::Fog => "foggy conditions with poor visibility",
            Weather::HeavyWind => "heavy winds with significant turbulence",
            Weather::Rain => "steady rain with limited visibility",
            Weather::Snow => "snowy conditions with ice formation risk",
            Weather::Hail => "hailstorm with impact damage risk",
            Weather::Storm => "stormy weather with severe turbulence",
            Weather::HeavyRain => "heavy rain with very poor visibility",
            Weather::Blizzard => "blizzard conditions with zero visibility",
            Weather::Hurricane => "hurricane-force winds - extremely dangerous",
            Weather::Tornado => "tornado activity - flying prohibited",
            Weather::Thunderstorm => "thunderstorm with lightning risk",
        }
    }

    /// Check if flying is safe in this weather
    pub fn is_safe_for_flying(&self) -> bool {
        match self {
            Weather::Hurricane | Weather::Tornado | Weather::Thunderstorm => false,
            _ => true,
        }
    }

    /// Check if this weather condition affects visibility
    pub fn affects_visibility(&self) -> bool {
        match self {
            Weather::Clear | Weather::Sunny | Weather::PartlyCloudy | Weather::LightWind => false,
            _ => true,
        }
    }

    /// Get the visibility level (1-5, where 5 is perfect visibility)
    pub fn visibility_level(&self) -> u8 {
        match self {
            Weather::Clear | Weather::Sunny => 5,
            Weather::PartlyCloudy | Weather::LightWind => 4,
            Weather::Overcast | Weather::Cloudy | Weather::Windy => 3,
            Weather::LightRain | Weather::HeavyWind | Weather::Snow => 2,
            Weather::Fog | Weather::Rain | Weather::Hail | Weather::Storm => 1,
            Weather::HeavyRain
            | Weather::Blizzard
            | Weather::Hurricane
            | Weather::Tornado
            | Weather::Thunderstorm => 0,
        }
    }

    /// Get weather conditions suitable for a given skill level
    pub fn suitable_for_skill_level(skill_level: u8) -> Vec<Weather> {
        Self::all_weather_conditions()
            .into_iter()
            .filter(|weather| weather.difficulty_level() <= skill_level)
            .collect()
    }

    /// Get all weather conditions
    pub fn all_weather_conditions() -> Vec<Weather> {
        vec![
            Weather::Clear,
            Weather::Sunny,
            Weather::PartlyCloudy,
            Weather::Overcast,
            Weather::LightWind,
            Weather::Cloudy,
            Weather::Windy,
            Weather::LightRain,
            Weather::Fog,
            Weather::HeavyWind,
            Weather::Rain,
            Weather::Snow,
            Weather::Hail,
            Weather::Storm,
            Weather::HeavyRain,
            Weather::Blizzard,
            Weather::Hurricane,
            Weather::Tornado,
            Weather::Thunderstorm,
        ]
    }

    /// Get weather conditions by difficulty level
    pub fn by_difficulty(max_difficulty: u8) -> Vec<Weather> {
        Self::all_weather_conditions()
            .into_iter()
            .filter(|weather| weather.difficulty_level() <= max_difficulty)
            .collect()
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Weather::Clear => "Clear",
            Weather::Sunny => "Sunny",
            Weather::PartlyCloudy => "Partly Cloudy",
            Weather::Overcast => "Overcast",
            Weather::LightWind => "Light Wind",
            Weather::Cloudy => "Cloudy",
            Weather::Windy => "Windy",
            Weather::LightRain => "Light Rain",
            Weather::Fog => "Fog",
            Weather::HeavyWind => "Heavy Wind",
            Weather::Rain => "Rain",
            Weather::Snow => "Snow",
            Weather::Hail => "Hail",
            Weather::Storm => "Storm",
            Weather::HeavyRain => "Heavy Rain",
            Weather::Blizzard => "Blizzard",
            Weather::Hurricane => "Hurricane",
            Weather::Tornado => "Tornado",
            Weather::Thunderstorm => "Thunderstorm",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for Weather {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "clear" => Weather::Clear,
            "sunny" => Weather::Sunny,
            "partly_cloudy" | "partly cloudy" => Weather::PartlyCloudy,
            "overcast" => Weather::Overcast,
            "light_wind" | "light wind" => Weather::LightWind,
            "cloudy" => Weather::Cloudy,
            "windy" => Weather::Windy,
            "light_rain" | "light rain" => Weather::LightRain,
            "fog" | "foggy" => Weather::Fog,
            "heavy_wind" | "heavy wind" => Weather::HeavyWind,
            "rain" | "rainy" => Weather::Rain,
            "snow" | "snowy" => Weather::Snow,
            "hail" => Weather::Hail,
            "storm" | "stormy" => Weather::Storm,
            "heavy_rain" | "heavy rain" => Weather::HeavyRain,
            "blizzard" => Weather::Blizzard,
            "hurricane" => Weather::Hurricane,
            "tornado" => Weather::Tornado,
            "thunderstorm" | "thunder" => Weather::Thunderstorm,
            _ => Weather::Clear, // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_difficulty() {
        assert_eq!(Weather::Clear.difficulty_level(), 1);
        assert_eq!(Weather::Windy.difficulty_level(), 3);
        assert_eq!(Weather::Storm.difficulty_level(), 5);
        assert_eq!(Weather::Hurricane.difficulty_level(), 6);
    }

    #[test]
    fn test_energy_requirements() {
        assert_eq!(Weather::Clear.required_energy_level(), EnergyLevel::Normal);
        assert_eq!(
            Weather::Windy.required_energy_level(),
            EnergyLevel::Energetic
        );
        assert_eq!(
            Weather::Hurricane.required_energy_level(),
            EnergyLevel::Hyperactive
        );
    }

    #[test]
    fn test_safety() {
        assert!(Weather::Clear.is_safe_for_flying());
        assert!(Weather::Storm.is_safe_for_flying());
        assert!(!Weather::Hurricane.is_safe_for_flying());
        assert!(!Weather::Tornado.is_safe_for_flying());
    }

    #[test]
    fn test_visibility() {
        assert_eq!(Weather::Clear.visibility_level(), 5);
        assert_eq!(Weather::Fog.visibility_level(), 1);
        assert_eq!(Weather::Blizzard.visibility_level(), 0);
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Weather::from("clear"), Weather::Clear);
        assert_eq!(Weather::from("heavy_rain"), Weather::HeavyRain);
        assert_eq!(Weather::from("unknown"), Weather::Clear); // fallback
    }
}

use crate::behaviors::{driving::Driving, land_move::LandMove, moving::Moving, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct AmphibiousVehicle {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel,
    amphibious_type: AmphibiousType,
    propulsion: AmphibiousPropulsion,
    hull_type: HullType,
}

#[derive(Debug, Clone)]
pub enum AmphibiousType {
    Duck,             // WWII-style amphibious truck
    Hovercraft,       // Air cushion vehicle
    AmphibiousCar,    // Modern amphibious car
    LandingCraft,     // Military assault vehicle
    AmphibiousRV,     // Recreational vehicle
    EmergencyVehicle, // Rescue operations
}

#[derive(Debug, Clone)]
pub enum AmphibiousPropulsion {
    WheelsAndPropeller {
        wheels: u8,
        propeller_power: u32,
    },
    TracksAndWaterJet {
        tracks: bool,
        waterjet_power: u32,
    },
    HovercraftFan {
        fan_power: u32,
        cushion_pressure: u32,
    },
    HybridSystem {
        land_system: String,
        water_system: String,
    },
}

#[derive(Debug, Clone)]
pub enum HullType {
    BoatHull,     // Traditional boat-like hull
    PlanningHull, // High-speed water operations
    Catamaran,    // Twin hull design
    AirCushion,   // For hovercraft
    Sealed,       // Waterproof car body
}

impl AmphibiousVehicle {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        amphibious_type: AmphibiousType,
        propulsion: AmphibiousPropulsion,
        hull_type: HullType,
    ) -> Self {
        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            amphibious_type,
            propulsion,
            hull_type,
        }
    }

    pub fn amphibious_type(&self) -> &AmphibiousType {
        &self.amphibious_type
    }

    pub fn propulsion(&self) -> &AmphibiousPropulsion {
        &self.propulsion
    }

    pub fn hull_type(&self) -> &HullType {
        &self.hull_type
    }

    /// Switch between land and water mode
    pub fn switch_mode(&mut self, to_water: bool) -> Result<String, String> {
        let mode = if to_water { "water" } else { "land" };

        // Different amphibious types have different transition capabilities
        let transition_energy = match self.amphibious_type {
            AmphibiousType::Duck => 1,             // Simple transition
            AmphibiousType::Hovercraft => 0,       // No transition needed
            AmphibiousType::AmphibiousCar => 1,    // Simple
            AmphibiousType::LandingCraft => 1,     // Military efficiency
            AmphibiousType::AmphibiousRV => 2,     // More complex
            AmphibiousType::EmergencyVehicle => 1, // Quick response needed
        };

        if self.energy() < EnergyLevel::Tired {
            return Err(format!("Insufficient energy to switch to {} mode", mode));
        }

        self.consume_energy_levels(transition_energy);

        Ok(format!("Successfully switched to {} mode", mode))
    }

    /// Refuel the vehicle
    pub fn refuel(&mut self) {
        self.energy = EnergyLevel::Hyperactive;
    }
}

impl Vehicle for AmphibiousVehicle {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Amphibious Vehicle"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        format!(
            "{} {} {} ({}, {:?}, {:?}, {:?})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.amphibious_type,
            self.hull_type,
            self.propulsion
        )
    }
}

impl HasEnergy for AmphibiousVehicle {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for AmphibiousVehicle {}
impl LandMove for AmphibiousVehicle {}

impl Driving for AmphibiousVehicle {
    fn max_speed(&self) -> u32 {
        let base_speed = match self.amphibious_type {
            AmphibiousType::Duck => 80,              // Military utility
            AmphibiousType::Hovercraft => 100,       // High speed capability
            AmphibiousType::AmphibiousCar => 120,    // Car-like performance
            AmphibiousType::LandingCraft => 60,      // Heavy and armored
            AmphibiousType::AmphibiousRV => 90,      // Recreational
            AmphibiousType::EmergencyVehicle => 110, // Emergency response
        };

        // Propulsion affects land speed
        let propulsion_modifier = match &self.propulsion {
            AmphibiousPropulsion::WheelsAndPropeller { wheels, .. } => {
                (*wheels as u32) * 5 // More wheels = better land performance
            }
            AmphibiousPropulsion::TracksAndWaterJet { tracks, .. } => {
                if *tracks {
                    10
                } else {
                    0
                }
            } // Tracks help on rough terrain
            AmphibiousPropulsion::HovercraftFan { .. } => 20, // Hovercraft can go fast on any surface
            AmphibiousPropulsion::HybridSystem { .. } => 15,  // Optimized for both
        };

        base_speed + propulsion_modifier
    }

    fn fuel_efficiency(&self) -> u32 {
        let base_efficiency = match self.amphibious_type {
            AmphibiousType::Duck => 25u32,             // Military efficiency
            AmphibiousType::Hovercraft => 15u32,       // High energy consumption
            AmphibiousType::AmphibiousCar => 40u32,    // Car-like efficiency
            AmphibiousType::LandingCraft => 20u32,     // Heavy vehicle
            AmphibiousType::AmphibiousRV => 30u32,     // Recreational efficiency
            AmphibiousType::EmergencyVehicle => 25u32, // Utility focus
        };

        // Hull design affects efficiency on land
        let hull_penalty = match self.hull_type {
            HullType::BoatHull => 5u32,     // Not optimized for land
            HullType::PlanningHull => 3u32, // Better than boat hull
            HullType::Catamaran => 7u32,    // Wide and draggy on land
            HullType::AirCushion => 0u32,   // No penalty for hovercraft
            HullType::Sealed => 1u32,       // Minimal penalty
        };

        base_efficiency.saturating_sub(hull_penalty)
    }
}

impl Swimming for AmphibiousVehicle {
    fn max_depth(&self) -> u32 {
        match self.amphibious_type {
            AmphibiousType::Duck => 2,             // Shallow water operations
            AmphibiousType::Hovercraft => 0,       // Surface only (air cushion)
            AmphibiousType::AmphibiousCar => 1,    // Very shallow
            AmphibiousType::LandingCraft => 3,     // Beach assault capability
            AmphibiousType::AmphibiousRV => 1,     // Recreational shallow water
            AmphibiousType::EmergencyVehicle => 2, // Flood rescue operations
        }
    }
}

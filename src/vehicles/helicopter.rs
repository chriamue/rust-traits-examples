use crate::behaviors::{flying::Flying, moving::Moving};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Helicopter {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel, // Represents fuel level
    helicopter_type: HelicopterType,
    rotor_diameter: u32, // in meters
    engine_type: HelicopterEngine,
}

#[derive(Debug, Clone)]
pub enum HelicopterType {
    Emergency, // Ambulance, rescue
    Military,  // Attack, transport
    Civilian,  // Private, taxi
    Cargo,     // Heavy lift
    Police,    // Law enforcement
    News,      // Media, surveillance
}

#[derive(Debug, Clone)]
pub enum HelicopterEngine {
    Turboshaft { engines: u8, power_each: u32 }, // power in kW
    Piston { engines: u8, displacement: f32 },   // displacement in liters
    Electric { motors: u8, battery_capacity: u32 }, // battery in kWh
}

impl Helicopter {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        helicopter_type: HelicopterType,
        rotor_diameter: u32,
        engine_type: HelicopterEngine,
    ) -> Self {
        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            helicopter_type,
            rotor_diameter,
            engine_type,
        }
    }

    pub fn helicopter_type(&self) -> &HelicopterType {
        &self.helicopter_type
    }

    pub fn rotor_diameter(&self) -> u32 {
        self.rotor_diameter
    }

    pub fn engine_type(&self) -> &HelicopterEngine {
        &self.engine_type
    }

    /// Refuel the helicopter
    pub fn refuel(&mut self) {
        self.energy = EnergyLevel::Hyperactive;
    }
}

impl Vehicle for Helicopter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Helicopter"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        let engine_desc = match &self.engine_type {
            HelicopterEngine::Turboshaft {
                engines,
                power_each,
            } => format!("{}x{}kW Turboshaft", engines, power_each),
            HelicopterEngine::Piston {
                engines,
                displacement,
            } => format!("{}x{:.1}L Piston", engines, displacement),
            HelicopterEngine::Electric {
                motors,
                battery_capacity,
            } => format!("{}x Electric ({}kWh)", motors, battery_capacity),
        };

        format!(
            "{} {} {} ({}, {:?}, {}m rotor, {})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.helicopter_type,
            self.rotor_diameter,
            engine_desc
        )
    }
}

impl HasEnergy for Helicopter {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Helicopter {}

impl Flying for Helicopter {
    fn max_altitude(&self) -> u32 {
        let base_altitude = match self.helicopter_type {
            HelicopterType::Emergency => 3000, // Need to reach mountain rescues
            HelicopterType::Military => 6000,  // High altitude operations
            HelicopterType::Civilian => 2000,  // City flights
            HelicopterType::Cargo => 2500,     // Heavy lifting capability
            HelicopterType::Police => 1500,    // Urban surveillance
            HelicopterType::News => 1000,      // City reporting
        };

        // Larger rotors and more powerful engines can go higher
        let rotor_bonus = (self.rotor_diameter / 2) * 100;
        let engine_bonus = match &self.engine_type {
            HelicopterEngine::Turboshaft {
                engines,
                power_each,
            } => (*engines as u32) * (*power_each / 100),
            HelicopterEngine::Piston { engines, .. } => (*engines as u32) * 50,
            HelicopterEngine::Electric { motors, .. } => (*motors as u32) * 100,
        };

        base_altitude + rotor_bonus + engine_bonus
    }
}

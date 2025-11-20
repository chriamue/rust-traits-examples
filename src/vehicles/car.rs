use crate::behaviors::{driving::Driving, land_move::LandMove, moving::Moving};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Car {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel, // Represents fuel level
    max_speed: u32,
    engine_type: EngineType,
}

#[derive(Debug, Clone)]
pub enum EngineType {
    Gasoline {
        cylinders: u8,
        displacement: f32,
    }, // displacement in liters
    Electric {
        battery_capacity: u32,
    }, // kWh
    Hybrid {
        gas_engine: Box<EngineType>,
        electric_motor: u32,
    }, // electric motor in kW
    Diesel {
        cylinders: u8,
        displacement: f32,
    },
}

impl Car {
    pub fn new(name: String, manufacturer: String, year: u32, engine_type: EngineType) -> Self {
        let max_speed = match &engine_type {
            EngineType::Gasoline { cylinders, .. } => 120 + (cylinders * 10) as u32,
            EngineType::Electric { battery_capacity } => 100 + (battery_capacity / 10),
            EngineType::Hybrid { .. } => 140,
            EngineType::Diesel { cylinders, .. } => 110 + (cylinders * 8) as u32,
        };

        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal, // Start with half tank
            max_speed,
            engine_type,
        }
    }

    pub fn engine_type(&self) -> &EngineType {
        &self.engine_type
    }

    /// Refuel the car
    pub fn refuel(&mut self) {
        self.energy = EnergyLevel::Hyperactive; // Full tank
    }

    /// Get fuel level as percentage
    pub fn fuel_percentage(&self) -> u8 {
        match self.energy {
            EnergyLevel::Collapsed => 0,
            EnergyLevel::Exhausted => 15,
            EnergyLevel::Tired => 30,
            EnergyLevel::Normal => 50,
            EnergyLevel::Energetic => 75,
            EnergyLevel::Hyperactive => 100,
        }
    }
}

impl Vehicle for Car {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Car"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        let engine_desc = match &self.engine_type {
            EngineType::Gasoline {
                cylinders,
                displacement,
            } => format!("{:.1}L V{} Gasoline", displacement, cylinders),
            EngineType::Electric { battery_capacity } => {
                format!("{}kWh Electric", battery_capacity)
            }
            EngineType::Hybrid { .. } => "Hybrid".to_string(),
            EngineType::Diesel {
                cylinders,
                displacement,
            } => format!("{:.1}L V{} Diesel", displacement, cylinders),
        };

        format!(
            "{} {} {} ({}, {})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            engine_desc
        )
    }
}

impl HasEnergy for Car {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Car {}
impl LandMove for Car {}

impl Driving for Car {
    fn max_speed(&self) -> u32 {
        self.max_speed
    }

    fn fuel_efficiency(&self) -> u32 {
        match &self.engine_type {
            EngineType::Gasoline {
                cylinders,
                displacement,
            } => {
                // Larger engines are less efficient
                let base_efficiency = 60;
                base_efficiency - (cylinders * 2) as u32 - (displacement * 5.0) as u32
            }
            EngineType::Electric { .. } => 120, // Electric cars are very efficient
            EngineType::Hybrid { .. } => 90,    // Hybrids are quite efficient
            EngineType::Diesel { .. } => 70,    // Diesel is more efficient than gasoline
        }
    }

    // Add this method to explicitly indicate no off-road capability
    fn has_off_road_capability(&self) -> bool {
        false // Regular cars cannot handle off-road terrain
    }
}

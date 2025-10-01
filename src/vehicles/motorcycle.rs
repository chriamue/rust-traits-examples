use crate::behaviors::{driving::Driving, moving::Moving};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Motorcycle {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel,
    engine_size: u32, // in cc
    motorcycle_type: MotorcycleType,
}

#[derive(Debug, Clone)]
pub enum MotorcycleType {
    Sport,    // High speed, low comfort
    Cruiser,  // Moderate speed, high comfort
    Touring,  // Moderate speed, long distance
    Dirt,     // Moderate speed, off-road capable
    Electric, // Quiet, efficient
}

impl Motorcycle {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        engine_size: u32,
        moto_type: MotorcycleType,
    ) -> Self {
        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            engine_size,
            motorcycle_type: moto_type,
        }
    }

    pub fn engine_size(&self) -> u32 {
        self.engine_size
    }

    pub fn motorcycle_type(&self) -> &MotorcycleType {
        &self.motorcycle_type
    }
}

impl Vehicle for Motorcycle {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Motorcycle"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        format!(
            "{} {} {} ({}, {}cc {:?})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.engine_size,
            self.motorcycle_type
        )
    }
}

impl HasEnergy for Motorcycle {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Motorcycle {}

impl Driving for Motorcycle {
    fn max_speed(&self) -> u32 {
        let base_speed = match self.motorcycle_type {
            MotorcycleType::Sport => 200,
            MotorcycleType::Cruiser => 140,
            MotorcycleType::Touring => 160,
            MotorcycleType::Dirt => 120,
            MotorcycleType::Electric => 130,
        };

        // Engine size affects top speed
        base_speed + (self.engine_size / 50)
    }

    fn fuel_efficiency(&self) -> u32 {
        let base_efficiency = match self.motorcycle_type {
            MotorcycleType::Sport => 40,     // Performance over efficiency
            MotorcycleType::Cruiser => 60,   // Balanced
            MotorcycleType::Touring => 70,   // Built for long distances
            MotorcycleType::Dirt => 50,      // Off-road focused
            MotorcycleType::Electric => 150, // Very efficient
        };

        // Larger engines are generally less efficient
        base_efficiency - (self.engine_size / 100)
    }
}

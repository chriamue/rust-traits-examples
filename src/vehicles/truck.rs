use crate::behaviors::{driving::Driving, moving::Moving};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Truck {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel,
    payload_capacity: u32, // in kg
    truck_type: TruckType,
    current_load: u32, // in kg
}

#[derive(Debug, Clone)]
pub enum TruckType {
    Pickup,    // Light duty
    Delivery,  // Medium duty, city use
    Semi,      // Heavy duty, long haul
    Dump,      // Heavy duty, construction
    Emergency, // Fire truck, ambulance, etc.
}

impl Truck {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        payload_capacity: u32,
        truck_type: TruckType,
    ) -> Self {
        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            payload_capacity,
            truck_type,
            current_load: 0,
        }
    }

    pub fn load_cargo(&mut self, weight: u32) -> Result<(), String> {
        if self.current_load + weight > self.payload_capacity {
            Err(format!(
                "Cannot load {} kg: would exceed capacity of {} kg",
                weight, self.payload_capacity
            ))
        } else {
            self.current_load += weight;
            Ok(())
        }
    }

    pub fn unload_cargo(&mut self, weight: u32) -> Result<(), String> {
        if weight > self.current_load {
            Err(format!(
                "Cannot unload {} kg: only {} kg loaded",
                weight, self.current_load
            ))
        } else {
            self.current_load -= weight;
            Ok(())
        }
    }

    pub fn current_load(&self) -> u32 {
        self.current_load
    }

    pub fn payload_capacity(&self) -> u32 {
        self.payload_capacity
    }

    pub fn load_percentage(&self) -> f32 {
        (self.current_load as f32 / self.payload_capacity as f32) * 100.0
    }
}

impl Vehicle for Truck {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Truck"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        format!(
            "{} {} {} ({}, {:?}, {}kg capacity)",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.truck_type,
            self.payload_capacity
        )
    }
}

impl HasEnergy for Truck {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Truck {}

impl Driving for Truck {
    fn max_speed(&self) -> u32 {
        let base_speed = match self.truck_type {
            TruckType::Pickup => 160,
            TruckType::Delivery => 120,
            TruckType::Semi => 110,
            TruckType::Dump => 90,
            TruckType::Emergency => 140,
        };

        // Load affects maximum speed
        let load_factor = self.current_load as f32 / self.payload_capacity as f32;
        let speed_reduction = (load_factor * 20.0) as u32; // Up to 20 km/h reduction when fully loaded

        base_speed.saturating_sub(speed_reduction)
    }

    fn fuel_efficiency(&self) -> u32 {
        let base_efficiency = match self.truck_type {
            TruckType::Pickup => 35,
            TruckType::Delivery => 25,
            TruckType::Semi => 15,
            TruckType::Dump => 20,
            TruckType::Emergency => 30,
        };

        // Load affects fuel efficiency
        let load_factor = self.current_load as f32 / self.payload_capacity as f32;
        let efficiency_reduction = (load_factor * 10.0) as u32; // Up to 10 km per energy level reduction

        base_efficiency.saturating_sub(efficiency_reduction)
    }
}

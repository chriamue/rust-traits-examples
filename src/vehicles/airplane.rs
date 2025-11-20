use crate::behaviors::{driving::Driving, flying::Flying, land_move::LandMove, moving::Moving};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Airplane {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel, // Represents fuel level
    airplane_type: AirplaneType,
    wingspan: u32, // in meters
    engine_type: AirplaneEngine,
    can_taxi: bool, // Can it drive on runways?
}

#[derive(Debug, Clone)]
pub enum AirplaneType {
    Commercial, // Passenger airliner
    Military,   // Fighter, bomber, transport
    Private,    // Small personal aircraft
    Cargo,      // Freight hauler
    Aerobatic,  // Stunt flying
    Seaplane,   // Can land on water
}

#[derive(Debug, Clone)]
pub enum AirplaneEngine {
    Jet { engines: u8, thrust_each: u32 },          // thrust in kN
    Turboprop { engines: u8, power_each: u32 },     // power in kW
    Piston { engines: u8, power_each: u32 },        // power in kW
    Electric { motors: u8, battery_capacity: u32 }, // battery in kWh
}

impl Airplane {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        airplane_type: AirplaneType,
        wingspan: u32,
        engine_type: AirplaneEngine,
    ) -> Self {
        // Most airplanes can taxi on runways (drive slowly on ground)
        let can_taxi = true;

        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            airplane_type,
            wingspan,
            engine_type,
            can_taxi,
        }
    }

    pub fn airplane_type(&self) -> &AirplaneType {
        &self.airplane_type
    }

    pub fn wingspan(&self) -> u32 {
        self.wingspan
    }

    pub fn engine_type(&self) -> &AirplaneEngine {
        &self.engine_type
    }

    pub fn can_taxi(&self) -> bool {
        self.can_taxi
    }

    /// Refuel the airplane
    pub fn refuel(&mut self) {
        self.energy = EnergyLevel::Hyperactive;
    }
}

impl Vehicle for Airplane {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Airplane"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        let engine_desc = match &self.engine_type {
            AirplaneEngine::Jet {
                engines,
                thrust_each,
            } => format!("{}x{}kN Jet", engines, thrust_each),
            AirplaneEngine::Turboprop {
                engines,
                power_each,
            } => format!("{}x{}kW Turboprop", engines, power_each),
            AirplaneEngine::Piston {
                engines,
                power_each,
            } => format!("{}x{}kW Piston", engines, power_each),
            AirplaneEngine::Electric {
                motors,
                battery_capacity,
            } => format!("{}x Electric ({}kWh)", motors, battery_capacity),
        };

        format!(
            "{} {} {} ({}, {:?}, {}m wingspan, {})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.airplane_type,
            self.wingspan,
            engine_desc
        )
    }
}

impl HasEnergy for Airplane {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Airplane {}
impl LandMove for Airplane {}

impl Flying for Airplane {
    fn max_altitude(&self) -> u32 {
        let base_altitude = match self.airplane_type {
            AirplaneType::Commercial => 12000, // Cruise altitude
            AirplaneType::Military => 15000,   // High altitude capability
            AirplaneType::Private => 4000,     // General aviation
            AirplaneType::Cargo => 11000,      // Similar to commercial
            AirplaneType::Aerobatic => 3000,   // Lower altitude stunts
            AirplaneType::Seaplane => 2000,    // Lower altitude operations
        };

        // Engine type affects maximum altitude
        let engine_bonus = match &self.engine_type {
            AirplaneEngine::Jet {
                engines,
                thrust_each,
            } => (*engines as u32) * (*thrust_each / 10), // Jets can go very high
            AirplaneEngine::Turboprop {
                engines,
                power_each,
            } => (*engines as u32) * (*power_each / 100),
            AirplaneEngine::Piston {
                engines,
                power_each,
            } => (*engines as u32) * (*power_each / 200), // Lower altitude
            AirplaneEngine::Electric { motors, .. } => (*motors as u32) * 200, // Limited by battery
        };

        base_altitude + engine_bonus
    }
}

impl Driving for Airplane {
    fn max_speed(&self) -> u32 {
        // Taxi speed on ground - much slower than flight speed
        match self.airplane_type {
            AirplaneType::Commercial => 30, // Large planes taxi slowly
            AirplaneType::Military => 40,   // Military can taxi faster
            AirplaneType::Private => 25,    // Small planes
            AirplaneType::Cargo => 25,      // Heavy and slow
            AirplaneType::Aerobatic => 35,  // Agile on ground too
            AirplaneType::Seaplane => 20,   // Careful on water/land
        }
    }

    fn fuel_efficiency(&self) -> u32 {
        // Very low efficiency when driving/taxiing (engines not optimized for ground)
        match &self.engine_type {
            AirplaneEngine::Jet { .. } => 5, // Jets are terrible for ground
            AirplaneEngine::Turboprop { .. } => 8, // Slightly better
            AirplaneEngine::Piston { .. } => 12, // Better for low speed
            AirplaneEngine::Electric { .. } => 20, // Most efficient
        }
    }
}

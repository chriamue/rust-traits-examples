use crate::behaviors::{moving::Moving, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;

#[derive(Debug)]
pub struct Ship {
    name: String,
    manufacturer: String,
    year: u32,
    energy: EnergyLevel, // Represents fuel level
    ship_type: ShipType,
    displacement: u32, // in tons
    propulsion: PropulsionType,
}

#[derive(Debug, Clone)]
pub enum ShipType {
    CargoShip,
    CruiseShip,
    Warship,
    Yacht,
    Ferry,
    Submarine,
    Speedboat,
}

#[derive(Debug, Clone)]
pub enum PropulsionType {
    Diesel {
        engines: u8,
        power_each: u32,
    }, // power in kW
    Nuclear {
        reactors: u8,
    },
    Wind {
        sails: u8,
    },
    Electric {
        motors: u8,
        power_each: u32,
    },
    Hybrid {
        primary: Box<PropulsionType>,
        secondary: Box<PropulsionType>,
    },
}

impl Ship {
    pub fn new(
        name: String,
        manufacturer: String,
        year: u32,
        ship_type: ShipType,
        displacement: u32,
        propulsion: PropulsionType,
    ) -> Self {
        Self {
            name,
            manufacturer,
            year,
            energy: EnergyLevel::Normal,
            ship_type,
            displacement,
            propulsion,
        }
    }

    pub fn ship_type(&self) -> &ShipType {
        &self.ship_type
    }

    pub fn displacement(&self) -> u32 {
        self.displacement
    }

    pub fn propulsion(&self) -> &PropulsionType {
        &self.propulsion
    }

    /// Refuel the ship
    pub fn refuel(&mut self) {
        self.energy = EnergyLevel::Hyperactive;
    }
}

impl Vehicle for Ship {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn vehicle_type(&self) -> &'static str {
        "Ship"
    }

    fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    fn year(&self) -> u32 {
        self.year
    }

    fn description(&self) -> String {
        let propulsion_desc = match &self.propulsion {
            PropulsionType::Diesel {
                engines,
                power_each,
            } => format!("{}x{}kW Diesel", engines, power_each),
            PropulsionType::Nuclear { reactors } => format!("{}x Nuclear", reactors),
            PropulsionType::Wind { sails } => format!("{}x Sails", sails),
            PropulsionType::Electric { motors, power_each } => {
                format!("{}x{}kW Electric", motors, power_each)
            }
            PropulsionType::Hybrid { .. } => "Hybrid".to_string(),
        };

        format!(
            "{} {} {} ({}, {:?}, {}t, {})",
            self.year(),
            self.manufacturer(),
            self.name(),
            self.vehicle_type(),
            self.ship_type,
            self.displacement,
            propulsion_desc
        )
    }
}

impl HasEnergy for Ship {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Ship {}

impl Swimming for Ship {
    fn max_depth(&self) -> u32 {
        match self.ship_type {
            ShipType::Submarine => 300, // Can dive deep
            ShipType::Speedboat => 0,   // Surface only
            ShipType::Yacht => 0,       // Surface only
            _ => 0,                     // Most ships stay on surface
        }
    }
}

use crate::animals::Animal;
use crate::behaviors::{moving::Moving, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug)]
pub struct Snake {
    name: String,
    energy: EnergyLevel,
    is_aquatic: bool,
}

impl Snake {
    pub fn new(name: String, is_aquatic: bool) -> Self {
        Self {
            name,
            energy: EnergyLevel::Normal,
            is_aquatic,
        }
    }
}

impl Animal for Snake {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Snake"
    }
}

impl HasEnergy for Snake {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Snake {}

// Only aquatic snakes can swim
impl Swimming for Snake {
    fn max_depth(&self) -> u32 {
        if self.is_aquatic {
            50
        } else {
            0 // Non-aquatic snakes can't really swim
        }
    }
}

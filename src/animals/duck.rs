use crate::animals::Animal;
use crate::behaviors::{flying::Flying, moving::Moving, swimming::Swimming, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug)]
pub struct Duck {
    name: String,
    energy: EnergyLevel,
}

impl Duck {
    pub fn new(name: String) -> Self {
        Self {
            name,
            energy: EnergyLevel::Normal,
        }
    }
}

impl Animal for Duck {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Duck"
    }
}

impl HasEnergy for Duck {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Duck {}
impl Walking for Duck {}
impl Swimming for Duck {
    fn max_depth(&self) -> u32 {
        5 // Ducks don't dive too deep
    }
}

impl Flying for Duck {
    fn max_altitude(&self) -> u32 {
        1000 // Moderate flying height
    }
}

use crate::animals::Animal;
use crate::behaviors::{moving::Moving, swimming::Swimming, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug)]
pub struct Penguin {
    name: String,
    energy: EnergyLevel,
}

impl Penguin {
    pub fn new(name: String) -> Self {
        Self {
            name,
            energy: EnergyLevel::Normal,
        }
    }
}

impl Animal for Penguin {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Penguin"
    }
}

impl HasEnergy for Penguin {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Penguin {}
impl Walking for Penguin {}

impl Swimming for Penguin {
    fn max_depth(&self) -> u32 {
        500 // Penguins are excellent divers
    }
}

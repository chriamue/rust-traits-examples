use crate::animals::Animal;
use crate::behaviors::LandMove;
use crate::behaviors::{flying::Flying, moving::Moving, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug)]
pub struct Eagle {
    name: String,
    energy: EnergyLevel,
}

impl Eagle {
    pub fn new(name: String) -> Self {
        Self {
            name,
            energy: EnergyLevel::Energetic, // Eagles start with high energy
        }
    }
}

impl Animal for Eagle {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Eagle"
    }
}

impl HasEnergy for Eagle {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Eagle {}
impl LandMove for Eagle {}
impl Walking for Eagle {}
impl Flying for Eagle {
    fn max_altitude(&self) -> u32 {
        3000 // Eagles can fly very high
    }
}

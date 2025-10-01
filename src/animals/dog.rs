use crate::animals::Animal;
use crate::behaviors::{moving::Moving, swimming::Swimming, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug)]
pub struct Dog {
    name: String,
    energy: EnergyLevel,
    breed: String,
}

impl Dog {
    pub fn new(name: String, breed: String) -> Self {
        Self {
            name,
            breed,
            energy: EnergyLevel::Energetic,
        }
    }

    pub fn breed(&self) -> &str {
        &self.breed
    }
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Dog"
    }
}

impl HasEnergy for Dog {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Dog {}
impl Walking for Dog {}

impl Swimming for Dog {
    fn max_depth(&self) -> u32 {
        match self.breed.as_str() {
            "Golden Retriever" | "Labrador" => 10, // Water dogs swim better
            "Bulldog" => 2,                        // Poor swimmers
            _ => 5,                                // Average
        }
    }
}

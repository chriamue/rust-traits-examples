use crate::animals::Animal;
use crate::behaviors::{moving::Moving, swimming::Swimming, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug, Clone, Copy)]
pub enum DogBreed {
    BorderCollie,
    GoldenRetriever,
    Labrador,
    Bulldog,
    Greyhound,
    Husky,
    SaintBernard,
    Other,
}

impl std::fmt::Display for DogBreed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DogBreed::BorderCollie => "Border Collie",
            DogBreed::GoldenRetriever => "Golden Retriever",
            DogBreed::Labrador => "Labrador",
            DogBreed::Bulldog => "Bulldog",
            DogBreed::Greyhound => "Greyhound",
            DogBreed::Husky => "Husky",
            DogBreed::SaintBernard => "Saint Bernard",
            DogBreed::Other => "Other",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub struct Dog {
    pub name: String,
    pub breed: DogBreed,
    pub energy: EnergyLevel,
}

impl Dog {
    pub fn new(name: String, breed: DogBreed) -> Self {
        Self {
            name,
            breed,
            energy: EnergyLevel::Energetic,
        }
    }
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.to_string()
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
        match self.breed {
            DogBreed::BorderCollie
            | DogBreed::GoldenRetriever
            | DogBreed::Labrador
            | DogBreed::Husky
            | DogBreed::SaintBernard => 10,
            DogBreed::Bulldog => 2,
            DogBreed::Greyhound => 3,
            DogBreed::Other => 5,
        }
    }
}

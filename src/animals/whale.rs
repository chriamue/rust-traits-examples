use crate::animals::Animal;
use crate::behaviors::moving::Moving;
use crate::behaviors::swimming::Swimming;
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug, Clone, Copy)]
pub enum WhaleSpecies {
    BlueWhale,
    Orca,
    Dolphin,
    Humpback,
}

impl WhaleSpecies {
    pub fn max_diving_depth(&self) -> u32 {
        match self {
            WhaleSpecies::Humpback => 200,  // Moderate diver
            WhaleSpecies::BlueWhale => 500, // Surface feeder
            WhaleSpecies::Orca => 300,      // Medium depth
            WhaleSpecies::Dolphin => 150,   // Shallow water
        }
    }
}

#[derive(Debug)]
pub struct Whale {
    pub name: String,
    pub species: WhaleSpecies,
    pub energy: EnergyLevel,
}

impl Whale {
    pub fn new(name: String, species: WhaleSpecies) -> Self {
        Self {
            name,
            species,
            energy: EnergyLevel::Normal,
        }
    }
}

impl Animal for Whale {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn species(&self) -> &'static str {
        "Whale"
    }
}

impl HasEnergy for Whale {
    fn energy(&self) -> EnergyLevel {
        self.energy
    }

    fn set_energy(&mut self, level: EnergyLevel) {
        self.energy = level;
    }
}

impl Moving for Whale {}

impl Swimming for Whale {
    fn max_depth(&self) -> u32 {
        self.species.max_diving_depth()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whale_diving_depths() {
        let blue_whale = Whale::new("Blue".to_string(), WhaleSpecies::BlueWhale);
        let humpback = Whale::new("Humpy".to_string(), WhaleSpecies::Humpback);
        let dolphin = Whale::new("Flipper".to_string(), WhaleSpecies::Dolphin);

        assert!(blue_whale.max_depth() > humpback.max_depth());
        assert!(humpback.max_depth() > dolphin.max_depth());
    }
}

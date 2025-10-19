use crate::animals::Animal;
use crate::behaviors::{moving::Moving, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};

#[derive(Debug, Clone, Copy)]
pub enum SnakeSpecies {
    Python,
    Anaconda,
    Viper,
    Cobra,
}

impl SnakeSpecies {
    pub fn swimming_depth(&self) -> u32 {
        match self {
            SnakeSpecies::Anaconda => 80, // Excellent swimmer, semi-aquatic
            SnakeSpecies::Python => 25,   // Good swimmer when needed
            SnakeSpecies::Cobra => 8,     // Basic swimming ability
            SnakeSpecies::Viper => 0,     // Cannot swim effectively
        }
    }
}

impl std::fmt::Display for SnakeSpecies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            SnakeSpecies::Python => "Python",
            SnakeSpecies::Anaconda => "Anaconda",
            SnakeSpecies::Viper => "Viper",
            SnakeSpecies::Cobra => "Cobra",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub struct Snake {
    pub name: String,
    pub species: SnakeSpecies,
    pub energy: EnergyLevel,
}

impl Snake {
    pub fn new(name: String, species: SnakeSpecies) -> Self {
        Self {
            name,
            species,
            energy: EnergyLevel::Normal,
        }
    }
}

impl Animal for Snake {
    fn name(&self) -> String {
        self.name.to_string()
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

impl Swimming for Snake {
    fn max_depth(&self) -> u32 {
        self.species.swimming_depth()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anaconda_swims_better_than_viper() {
        let anaconda = Snake::new("Ana".to_string(), SnakeSpecies::Anaconda);
        let viper = Snake::new("Venom".to_string(), SnakeSpecies::Viper);

        assert!(anaconda.max_depth() > viper.max_depth());
    }
}

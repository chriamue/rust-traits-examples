use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    // Easy terrains
    Road,
    Pavement,
    Sidewalk,

    // Moderate terrains
    Grass,
    Dirt,
    Gravel,
    Sand,

    // Challenging terrains
    Rocky,
    Muddy,
    Snow,
    Forest,

    // Difficult terrains
    Steep,
    Mountain,
    Swamp,
    Desert,

    // Extreme terrains
    Extreme,
    Cliff,
    Glacier,
    Volcano,
}

impl Terrain {
    /// Get the difficulty level of this terrain
    pub fn difficulty_level(&self) -> u8 {
        match self {
            // Easy (1-2)
            Terrain::Road | Terrain::Pavement | Terrain::Sidewalk => 1,

            // Moderate (3-4)
            Terrain::Grass | Terrain::Dirt => 2,
            Terrain::Gravel | Terrain::Sand => 3,

            // Challenging (5-6)
            Terrain::Rocky | Terrain::Muddy => 4,
            Terrain::Snow | Terrain::Forest => 5,

            // Difficult (7-8)
            Terrain::Steep | Terrain::Mountain => 6,
            Terrain::Swamp | Terrain::Desert => 7,

            // Extreme (9-10)
            Terrain::Extreme | Terrain::Cliff => 8,
            Terrain::Glacier | Terrain::Volcano => 9,
        }
    }

    /// Get the energy cost for navigating this terrain
    pub fn energy_cost(&self) -> u8 {
        match self {
            // Easy terrains - no extra cost
            Terrain::Road | Terrain::Pavement | Terrain::Sidewalk => 0,

            // Moderate terrains - low cost
            Terrain::Grass | Terrain::Dirt => 0,
            Terrain::Gravel | Terrain::Sand => 1,

            // Challenging terrains - moderate cost
            Terrain::Rocky | Terrain::Muddy => 1,
            Terrain::Snow | Terrain::Forest => 2,

            // Difficult terrains - high cost
            Terrain::Steep | Terrain::Mountain => 2,
            Terrain::Swamp | Terrain::Desert => 3,

            // Extreme terrains - very high cost
            Terrain::Extreme | Terrain::Cliff => 3,
            Terrain::Glacier | Terrain::Volcano => 4,
        }
    }

    /// Get the minimum energy level required to navigate this terrain
    pub fn required_energy_level(&self) -> crate::core::EnergyLevel {
        use crate::core::EnergyLevel;

        match self.difficulty_level() {
            1..=2 => EnergyLevel::Exhausted,
            3..=4 => EnergyLevel::Tired,
            5..=6 => EnergyLevel::Normal,
            7..=8 => EnergyLevel::Energetic,
            9..=10 => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal, // fallback
        }
    }

    /// Get a description of this terrain
    pub fn description(&self) -> &'static str {
        match self {
            Terrain::Road => "smooth asphalt road",
            Terrain::Pavement => "concrete pavement",
            Terrain::Sidewalk => "pedestrian sidewalk",
            Terrain::Grass => "grassy field",
            Terrain::Dirt => "packed dirt path",
            Terrain::Gravel => "loose gravel surface",
            Terrain::Sand => "sandy ground",
            Terrain::Rocky => "rocky, uneven surface",
            Terrain::Muddy => "muddy, slippery ground",
            Terrain::Snow => "snow-covered terrain",
            Terrain::Forest => "dense forest floor",
            Terrain::Steep => "steep inclined surface",
            Terrain::Mountain => "mountainous terrain",
            Terrain::Swamp => "marshy swampland",
            Terrain::Desert => "arid desert landscape",
            Terrain::Extreme => "extremely challenging terrain",
            Terrain::Cliff => "dangerous cliff face",
            Terrain::Glacier => "icy glacial surface",
            Terrain::Volcano => "volcanic rock and ash",
        }
    }

    /// Check if vehicles can generally navigate this terrain
    pub fn vehicle_accessible(&self) -> bool {
        match self {
            // Vehicles can handle roads and moderate terrains
            Terrain::Road | Terrain::Pavement | Terrain::Sidewalk => true,
            Terrain::Grass | Terrain::Dirt | Terrain::Gravel => true,

            // Some vehicles can handle challenging terrains (off-road vehicles)
            Terrain::Sand | Terrain::Rocky | Terrain::Snow => true,

            // Difficult for most vehicles
            Terrain::Muddy | Terrain::Forest => false,

            // Very difficult/impossible for vehicles
            Terrain::Steep
            | Terrain::Mountain
            | Terrain::Swamp
            | Terrain::Desert
            | Terrain::Extreme
            | Terrain::Cliff
            | Terrain::Glacier
            | Terrain::Volcano => false,
        }
    }

    /// Check if this terrain is suitable for walking/biological movement
    pub fn walkable(&self) -> bool {
        match self {
            // Most terrains are walkable with enough energy
            Terrain::Road | Terrain::Pavement | Terrain::Sidewalk => true,
            Terrain::Grass | Terrain::Dirt | Terrain::Gravel | Terrain::Sand => true,
            Terrain::Rocky | Terrain::Muddy | Terrain::Snow | Terrain::Forest => true,
            Terrain::Steep | Terrain::Mountain | Terrain::Swamp | Terrain::Desert => true,
            Terrain::Extreme => true, // Challenging but possible

            // Only extremely dangerous terrains are not walkable
            Terrain::Cliff | Terrain::Glacier | Terrain::Volcano => false,
        }
    }

    /// Get all terrain variants
    pub fn all_terrains() -> Vec<Terrain> {
        vec![
            Terrain::Road,
            Terrain::Pavement,
            Terrain::Sidewalk,
            Terrain::Grass,
            Terrain::Dirt,
            Terrain::Gravel,
            Terrain::Sand,
            Terrain::Rocky,
            Terrain::Muddy,
            Terrain::Snow,
            Terrain::Forest,
            Terrain::Steep,
            Terrain::Mountain,
            Terrain::Swamp,
            Terrain::Desert,
            Terrain::Extreme,
            Terrain::Cliff,
            Terrain::Glacier,
            Terrain::Volcano,
        ]
    }

    /// Get terrains suitable for a given difficulty level
    pub fn by_difficulty(max_difficulty: u8) -> Vec<Terrain> {
        Self::all_terrains()
            .into_iter()
            .filter(|terrain| terrain.difficulty_level() <= max_difficulty)
            .collect()
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Terrain::Road => "Road",
            Terrain::Pavement => "Pavement",
            Terrain::Sidewalk => "Sidewalk",
            Terrain::Grass => "Grass",
            Terrain::Dirt => "Dirt",
            Terrain::Gravel => "Gravel",
            Terrain::Sand => "Sand",
            Terrain::Rocky => "Rocky",
            Terrain::Muddy => "Muddy",
            Terrain::Snow => "Snow",
            Terrain::Forest => "Forest",
            Terrain::Steep => "Steep",
            Terrain::Mountain => "Mountain",
            Terrain::Swamp => "Swamp",
            Terrain::Desert => "Desert",
            Terrain::Extreme => "Extreme",
            Terrain::Cliff => "Cliff",
            Terrain::Glacier => "Glacier",
            Terrain::Volcano => "Volcano",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for Terrain {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "road" => Terrain::Road,
            "pavement" => Terrain::Pavement,
            "sidewalk" => Terrain::Sidewalk,
            "grass" => Terrain::Grass,
            "dirt" => Terrain::Dirt,
            "gravel" => Terrain::Gravel,
            "sand" | "sandy" => Terrain::Sand,
            "rock" | "rocks" | "rocky" => Terrain::Rocky,
            "mud" | "muddy" => Terrain::Muddy,
            "snow" => Terrain::Snow,
            "forest" => Terrain::Forest,
            "steep" | "steep_hill" => Terrain::Steep,
            "mountain" => Terrain::Mountain,
            "swamp" => Terrain::Swamp,
            "desert" => Terrain::Desert,
            "extreme" | "extreme_terrain" => Terrain::Extreme,
            "cliff" => Terrain::Cliff,
            "glacier" => Terrain::Glacier,
            "volcano" => Terrain::Volcano,
            _ => Terrain::Dirt, // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::EnergyLevel;

    #[test]
    fn test_terrain_difficulty() {
        assert_eq!(Terrain::Road.difficulty_level(), 1);
        assert_eq!(Terrain::Rocky.difficulty_level(), 4);
        assert_eq!(Terrain::Mountain.difficulty_level(), 6);
        assert_eq!(Terrain::Volcano.difficulty_level(), 9);
    }

    #[test]
    fn test_energy_requirements() {
        assert_eq!(
            Terrain::Road.required_energy_level(),
            EnergyLevel::Exhausted
        );
        assert_eq!(Terrain::Rocky.required_energy_level(), EnergyLevel::Tired);
        assert_eq!(
            Terrain::Mountain.required_energy_level(),
            EnergyLevel::Normal
        );
        assert_eq!(
            Terrain::Volcano.required_energy_level(),
            EnergyLevel::Hyperactive
        );
    }

    #[test]
    fn test_vehicle_accessibility() {
        assert!(Terrain::Road.vehicle_accessible());
        assert!(Terrain::Grass.vehicle_accessible());
        assert!(!Terrain::Forest.vehicle_accessible());
        assert!(!Terrain::Cliff.vehicle_accessible());
    }

    #[test]
    fn test_walkability() {
        assert!(Terrain::Road.walkable());
        assert!(Terrain::Mountain.walkable());
        assert!(!Terrain::Cliff.walkable());
        assert!(!Terrain::Volcano.walkable());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Terrain::from("road"), Terrain::Road);
        assert_eq!(Terrain::from("rocky"), Terrain::Rocky);
        assert_eq!(Terrain::from("extreme_terrain"), Terrain::Extreme);
        assert_eq!(Terrain::from("unknown"), Terrain::Dirt); // fallback
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Terrain::Road), "Road");
        assert_eq!(format!("{}", Terrain::Mountain), "Mountain");
    }

    #[test]
    fn test_by_difficulty() {
        let easy_terrains = Terrain::by_difficulty(2);
        assert!(easy_terrains.contains(&Terrain::Road));
        assert!(easy_terrains.contains(&Terrain::Grass));
        assert!(!easy_terrains.contains(&Terrain::Mountain));
    }
}

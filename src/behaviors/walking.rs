use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy, Intensity, Terrain};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkingPace {
    Crawl,
    Shuffle,
    Walk,
    BriskWalk,
    Jog,
    Run,
    Sprint,
}

impl WalkingPace {
    /// Get the difficulty level of this pace (1-7)
    pub fn difficulty_level(&self) -> u8 {
        match self {
            WalkingPace::Crawl => 1,
            WalkingPace::Shuffle => 2,
            WalkingPace::Walk => 3,
            WalkingPace::BriskWalk => 4,
            WalkingPace::Jog => 5,
            WalkingPace::Run => 6,
            WalkingPace::Sprint => 7,
        }
    }

    /// Get the energy cost for this pace
    pub fn energy_cost(&self) -> u8 {
        match self {
            WalkingPace::Crawl => 0,     // Very low energy
            WalkingPace::Shuffle => 0,   // Same as basic movement
            WalkingPace::Walk => 0,      // Same as basic movement
            WalkingPace::BriskWalk => 1, // Small extra cost
            WalkingPace::Jog => 1,       // Small extra cost
            WalkingPace::Run => 2,       // Moderate extra cost
            WalkingPace::Sprint => 3,    // High extra cost
        }
    }

    /// Get the minimum energy level required for this pace
    pub fn required_energy_level(&self) -> EnergyLevel {
        match self {
            WalkingPace::Crawl => EnergyLevel::Collapsed,
            WalkingPace::Shuffle => EnergyLevel::Exhausted,
            WalkingPace::Walk => EnergyLevel::Exhausted,
            WalkingPace::BriskWalk => EnergyLevel::Tired,
            WalkingPace::Jog => EnergyLevel::Normal,
            WalkingPace::Run => EnergyLevel::Energetic,
            WalkingPace::Sprint => EnergyLevel::Hyperactive,
        }
    }

    /// Get the speed multiplier for this pace (relative to normal walking)
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            WalkingPace::Crawl => 0.2,
            WalkingPace::Shuffle => 0.5,
            WalkingPace::Walk => 1.0,
            WalkingPace::BriskWalk => 1.5,
            WalkingPace::Jog => 2.0,
            WalkingPace::Run => 3.0,
            WalkingPace::Sprint => 5.0,
        }
    }

    /// Get a description of this pace
    pub fn description(&self) -> &'static str {
        match self {
            WalkingPace::Crawl => "very slow movement, barely moving forward",
            WalkingPace::Shuffle => "slow shuffling movement with small steps",
            WalkingPace::Walk => "normal steady walking pace",
            WalkingPace::BriskWalk => "energetic walking with purpose",
            WalkingPace::Jog => "light jogging, sustainable running pace",
            WalkingPace::Run => "steady running pace",
            WalkingPace::Sprint => "maximum speed burst, unsustainable",
        }
    }

    /// Check if this pace can be sustained for long periods
    pub fn is_sustainable(&self) -> bool {
        match self {
            WalkingPace::Crawl | WalkingPace::Shuffle | WalkingPace::Walk => true,
            WalkingPace::BriskWalk | WalkingPace::Jog => true, // Can be sustained for moderate periods
            WalkingPace::Run => false,                         // Short to medium periods only
            WalkingPace::Sprint => false,                      // Very short bursts only
        }
    }

    /// Get paces available for a given energy level
    pub fn available_for_energy(energy: EnergyLevel) -> Vec<WalkingPace> {
        vec![
            WalkingPace::Crawl,
            WalkingPace::Shuffle,
            WalkingPace::Walk,
            WalkingPace::BriskWalk,
            WalkingPace::Jog,
            WalkingPace::Run,
            WalkingPace::Sprint,
        ]
        .into_iter()
        .filter(|pace| energy >= pace.required_energy_level())
        .collect()
    }
}

impl std::fmt::Display for WalkingPace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            WalkingPace::Crawl => "Crawl",
            WalkingPace::Shuffle => "Shuffle",
            WalkingPace::Walk => "Walk",
            WalkingPace::BriskWalk => "Brisk Walk",
            WalkingPace::Jog => "Jog",
            WalkingPace::Run => "Run",
            WalkingPace::Sprint => "Sprint",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for WalkingPace {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "crawl" | "crawling" => WalkingPace::Crawl,
            "shuffle" | "shuffling" => WalkingPace::Shuffle,
            "walk" | "walking" => WalkingPace::Walk,
            "brisk" | "brisk_walk" | "fast_walk" => WalkingPace::BriskWalk,
            "jog" | "jogging" | "light_run" => WalkingPace::Jog,
            "run" | "running" => WalkingPace::Run,
            "sprint" | "sprinting" | "dash" => WalkingPace::Sprint,
            _ => WalkingPace::Walk, // Default fallback
        }
    }
}

impl From<Intensity> for WalkingPace {
    fn from(intensity: Intensity) -> Self {
        match intensity {
            Intensity::Gentle => WalkingPace::Shuffle,
            Intensity::Moderate => WalkingPace::Walk,
            Intensity::Vigorous => WalkingPace::BriskWalk,
            Intensity::Intense => WalkingPace::Run,
            Intensity::Maximum => WalkingPace::Sprint,
        }
    }
}

#[derive(Error, Debug)]
pub enum WalkingError {
    #[error("Cannot walk: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for walking: need at least {required}, have {current}")]
    InsufficientEnergyForWalking {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Terrain too difficult for walking: {terrain} - {description}")]
    TerrainToodifficult {
        terrain: Terrain,
        description: String,
    },

    #[error("Pace too demanding: {pace} requires {required} energy, have {current}")]
    PaceToodemanding {
        pace: WalkingPace,
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Cannot walk on {terrain}: not suitable for pedestrian movement")]
    TerrainNotWalkable { terrain: Terrain },
}

pub type WalkingResult = Result<String, WalkingError>;

/// Walking capability - depends on Moving trait and energy
pub trait Walking: Moving + HasEnergy {
    /// Get the entity's natural walking ability (affects performance on terrain)
    fn walking_ability(&self) -> u8 {
        3 // Default: moderate walking ability (1-5 scale)
    }

    /// Basic walking - more controlled than general movement
    fn walk(&mut self) -> WalkingResult {
        self.walk_at_pace(WalkingPace::Walk)
    }

    /// Running - faster but more energy-intensive (kept for backwards compatibility)
    fn run(&mut self) -> WalkingResult {
        self.walk_at_pace(WalkingPace::Run)
    }

    /// Walk at a specific pace - new enum-based method
    fn walk_at_pace(&mut self, pace: WalkingPace) -> WalkingResult {
        let current_energy = self.energy();
        let required_energy = pace.required_energy_level();

        if current_energy < required_energy {
            return Err(WalkingError::PaceToodemanding {
                pace,
                required: required_energy,
                current: current_energy,
            });
        }

        let pace_energy_cost = pace.energy_cost();
        if pace_energy_cost > 0 {
            self.consume_energy_levels(pace_energy_cost);
        }

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let pace_description = pace.description();
                Ok(format!("Entity {} ({})", pace, pace_description))
            }
            Err(movement_error) => Err(WalkingError::MovementError(movement_error)),
        }
    }

    /// Walk on terrain - now uses Terrain enum
    fn walk_on_terrain(&mut self, terrain: Terrain) -> WalkingResult {
        let current_energy = self.energy();

        // Check if terrain is walkable
        if !terrain.walkable() {
            return Err(WalkingError::TerrainNotWalkable { terrain });
        }

        let required_energy = terrain.required_energy_level();

        // Apply walking ability modifier to terrain difficulty
        let walking_ability = self.walking_ability();
        let energy_reduction = match walking_ability {
            5 => 1, // Excellent walker - reduce energy requirement
            4 => 0, // Good walker - no change
            3 => 0, // Average walker - no change
            2 => 0, // Poor walker - no change (terrain cost will handle this)
            1 => 0, // Very poor walker - no change
            _ => 0,
        };

        let adjusted_required = match required_energy {
            EnergyLevel::Hyperactive => {
                if energy_reduction > 0 {
                    EnergyLevel::Energetic
                } else {
                    required_energy
                }
            }
            EnergyLevel::Energetic => {
                if energy_reduction > 0 {
                    EnergyLevel::Normal
                } else {
                    required_energy
                }
            }
            _ => required_energy,
        };

        if current_energy < adjusted_required {
            return Err(WalkingError::TerrainToodifficult {
                terrain,
                description: format!(
                    "Need {} energy for {} terrain, have {}",
                    adjusted_required, terrain, current_energy
                ),
            });
        }

        // Terrain walking consumes extra energy based on difficulty
        let mut terrain_energy_cost = terrain.energy_cost();

        // Apply walking ability modifier to energy cost
        if walking_ability >= 4 && terrain_energy_cost > 0 {
            terrain_energy_cost = terrain_energy_cost.saturating_sub(1);
        } else if walking_ability <= 2 {
            terrain_energy_cost += 1;
        }

        if terrain_energy_cost > 0 {
            self.consume_energy_levels(terrain_energy_cost);
        }

        // Use basic walking as foundation
        match self.walk() {
            Ok(_) => Ok(format!(
                "Entity walks across {} terrain ({})",
                terrain,
                terrain.description()
            )),
            Err(walking_error) => Err(walking_error),
        }
    }

    /// Walk with specific intensity - converts Intensity to WalkingPace
    fn walk_with_intensity(&mut self, intensity: Intensity) -> WalkingResult {
        let pace = WalkingPace::from(intensity);
        self.walk_at_pace(pace)
    }

    /// Get available paces for current energy level
    fn available_paces(&self) -> Vec<WalkingPace> {
        WalkingPace::available_for_energy(self.energy())
    }

    /// Sprint - maximum effort for short distance
    fn sprint(&mut self) -> WalkingResult {
        self.walk_at_pace(WalkingPace::Sprint)
    }

    /// Jog - sustainable running pace
    fn jog(&mut self) -> WalkingResult {
        self.walk_at_pace(WalkingPace::Jog)
    }

    /// Walk at maximum sustainable pace for current energy
    fn walk_max_pace(&mut self) -> WalkingResult {
        let available_paces = self.available_paces();
        let max_sustainable_pace = available_paces
            .into_iter()
            .filter(|pace| pace.is_sustainable())
            .max_by_key(|pace| pace.difficulty_level())
            .unwrap_or(WalkingPace::Walk);

        self.walk_at_pace(max_sustainable_pace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::moving::Moving;

    #[derive(Debug)]
    struct TestWalker {
        energy: EnergyLevel,
        walking_ability: u8,
    }

    impl HasEnergy for TestWalker {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }
        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestWalker {}

    impl Walking for TestWalker {
        fn walking_ability(&self) -> u8 {
            self.walking_ability
        }
    }

    #[test]
    fn test_basic_walking() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Normal,
            walking_ability: 3,
        };

        let result = walker.walk();
        assert!(result.is_ok());
        assert_eq!(walker.energy(), EnergyLevel::Tired); // Energy consumed by basic movement
    }

    #[test]
    fn test_running_requires_more_energy() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Energetic,
            walking_ability: 3,
        };

        let result = walker.run();
        assert!(result.is_ok());
        // Should consume energy for run pace + basic movement
        assert!(walker.energy() < EnergyLevel::Energetic);
    }

    #[test]
    fn test_terrain_walking() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Hyperactive,
            walking_ability: 3,
        };

        let result = walker.walk_on_terrain(Terrain::Mountain);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pace_walking() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Hyperactive,
            walking_ability: 3,
        };

        let result = walker.walk_at_pace(WalkingPace::Sprint);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pace_too_demanding() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Tired,
            walking_ability: 3,
        };

        let result = walker.walk_at_pace(WalkingPace::Sprint);
        assert!(result.is_err());
    }

    #[test]
    fn test_walking_ability_affects_terrain() {
        let mut good_walker = TestWalker {
            energy: EnergyLevel::Energetic,
            walking_ability: 5,
        };

        let mut poor_walker = TestWalker {
            energy: EnergyLevel::Energetic,
            walking_ability: 2,
        };

        // Both try to walk on rocky terrain
        let good_result = good_walker.walk_on_terrain(Terrain::Rocky);
        let poor_result = poor_walker.walk_on_terrain(Terrain::Rocky);

        // Good walker should have an easier time
        assert!(good_result.is_ok());
        // Both should succeed, but good walker should have more energy left
        if poor_result.is_ok() {
            assert!(good_walker.energy() >= poor_walker.energy());
        }
    }

    #[test]
    fn test_intensity_conversion() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Hyperactive,
            walking_ability: 3,
        };

        let result = walker.walk_with_intensity(Intensity::Maximum);
        assert!(result.is_ok());
    }

    #[test]
    fn test_available_paces() {
        let walker = TestWalker {
            energy: EnergyLevel::Normal,
            walking_ability: 3,
        };

        let available = walker.available_paces();
        assert!(available.contains(&WalkingPace::Walk));
        assert!(available.contains(&WalkingPace::Jog));
        assert!(!available.contains(&WalkingPace::Sprint)); // Requires Hyperactive
    }

    #[test]
    fn test_max_sustainable_pace() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Normal,
            walking_ability: 3,
        };

        let result = walker.walk_max_pace();
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_walkable_terrain() {
        let mut walker = TestWalker {
            energy: EnergyLevel::Hyperactive,
            walking_ability: 5,
        };

        let result = walker.walk_on_terrain(Terrain::Volcano);
        assert!(result.is_err());
        if let Err(WalkingError::TerrainNotWalkable { terrain }) = result {
            assert_eq!(terrain, Terrain::Volcano);
        } else {
            panic!("Expected TerrainNotWalkable error");
        }
    }
}

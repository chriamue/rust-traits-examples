//! Unified Race Competition using intermediate traits
//!
//! This module demonstrates how intermediate traits like LandMove
//! can simplify competition design by abstracting over similar behaviors.

use crate::behaviors::{flying::Flying, land_move::LandMove, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnifiedLeg {
    pub participant_name: String,
    pub participant_category: String, // "Land Mover", "Swimmer", "Flyer"
    pub activity: String,
    pub starting_energy: EnergyLevel,
    pub result: Result<String, String>,
    pub final_energy: EnergyLevel,
    pub capability_score: u32,
}

impl fmt::Display for UnifiedLeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.result.is_ok() { "✅" } else { "❌" };
        write!(
            f,
            "    {} {} [{}]: {} -> {} - {} (Score: {})",
            status,
            self.participant_name,
            self.participant_category,
            self.starting_energy,
            self.final_energy,
            self.activity,
            self.capability_score
        )
    }
}

#[derive(Debug, Clone)]
pub struct UnifiedRaceResult {
    pub team_name: String,
    pub land_leg: UnifiedLeg,
    pub water_leg: UnifiedLeg,
    pub air_leg: UnifiedLeg,
    pub completed_legs: u8,
    pub abstraction_bonus: u32,
}

impl UnifiedRaceResult {
    pub fn score(&self) -> u32 {
        let completion_bonus = self.completed_legs as u32 * 400;
        let energy_bonus = (self.land_leg.final_energy as u32
            + self.water_leg.final_energy as u32
            + self.air_leg.final_energy as u32)
            * 15;
        let capability_bonus = (self.land_leg.capability_score
            + self.water_leg.capability_score
            + self.air_leg.capability_score)
            / 10;

        completion_bonus + energy_bonus + capability_bonus + self.abstraction_bonus
    }
}

impl fmt::Display for UnifiedRaceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "🌐 Team: {} - Unified Race Result:", self.team_name)?;
        writeln!(f, "  🚶🚗 Land Movement Leg:")?;
        writeln!(f, "{}", self.land_leg)?;
        writeln!(f, "  🏊 Water Movement Leg:")?;
        writeln!(f, "{}", self.water_leg)?;
        writeln!(f, "  🛩️  Air Movement Leg:")?;
        writeln!(f, "{}", self.air_leg)?;
        writeln!(f, "  Completed Legs: {}/3", self.completed_legs)?;
        writeln!(f, "  Abstraction Bonus: {} points", self.abstraction_bonus)?;
        writeln!(f, "  Total Score: {}", self.score())?;
        Ok(())
    }
}

/// Unified race team using intermediate trait abstractions
pub struct UnifiedRaceTeam<L, S, F>
where
    L: LandMove,             // Can be walking animal OR driving vehicle
    S: Swimming + HasEnergy, // Swimming entity
    F: Flying + HasEnergy,   // Flying entity
{
    pub name: String,
    pub land_mover: L,
    pub swimmer: S,
    pub flyer: F,
}

impl<L, S, F> UnifiedRaceTeam<L, S, F>
where
    L: LandMove,
    S: Swimming + HasEnergy,
    F: Flying + HasEnergy,
{
    pub fn new(name: String, land_mover: L, swimmer: S, flyer: F) -> Self {
        Self {
            name,
            land_mover,
            swimmer,
            flyer,
        }
    }

    /// Execute the unified race
    pub fn race(&mut self) -> UnifiedRaceResult {
        println!("🌐 Team {} starts the unified race!", self.name);
        println!("   Using LandMove abstraction for first leg!");

        let mut completed_legs = 0;
        let mut abstraction_bonus = 150; // Bonus for using abstraction

        // Leg 1: Land Movement (Walking OR Driving)
        println!("  🚶🚗 Leg 1: Land movement...");
        let land_start_energy = self.land_mover.energy();
        let land_result = match self.land_mover.land_move() {
            Ok(result) => {
                completed_legs += 1;
                println!("    ✅ {}", result);
                abstraction_bonus += 100; // Bonus for successful abstraction use
                Ok(result)
            }
            Err(e) => {
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let land_leg = UnifiedLeg {
            participant_name: self.get_land_mover_name(),
            participant_category: self.get_land_mover_category(),
            activity: "Land Movement".to_string(),
            starting_energy: land_start_energy,
            result: land_result,
            final_energy: self.land_mover.energy(),
            capability_score: self.get_land_mover_capability(),
        };

        // Leg 2: Swimming
        println!("  🏊 Leg 2: Swimming...");
        let swim_start_energy = self.swimmer.energy();
        let swim_result = match self.swimmer.swim() {
            Ok(result) => {
                completed_legs += 1;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let water_leg = UnifiedLeg {
            participant_name: self.get_swimmer_name(),
            participant_category: "Swimmer".to_string(),
            activity: "Swimming".to_string(),
            starting_energy: swim_start_energy,
            result: swim_result,
            final_energy: self.swimmer.energy(),
            capability_score: self.swimmer.max_depth(),
        };

        // Leg 3: Flying
        println!("  🛩️  Leg 3: Flying...");
        let fly_start_energy = self.flyer.energy();
        let fly_result = match self.flyer.fly() {
            Ok(result) => {
                completed_legs += 1;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let air_leg = UnifiedLeg {
            participant_name: self.get_flyer_name(),
            participant_category: "Flyer".to_string(),
            activity: "Flying".to_string(),
            starting_energy: fly_start_energy,
            result: fly_result,
            final_energy: self.flyer.energy(),
            capability_score: self.flyer.max_altitude(),
        };

        println!(
            "  🏁 Team {} completed {}/3 legs",
            self.name, completed_legs
        );

        UnifiedRaceResult {
            team_name: self.name.clone(),
            land_leg,
            water_leg,
            air_leg,
            completed_legs,
            abstraction_bonus,
        }
    }

    // Helper methods to get names and info from participants
    fn get_land_mover_name(&self) -> String {
        "Land Mover".to_string() // Simplified since we removed the name methods
    }

    fn get_land_mover_category(&self) -> String {
        "Land Mover".to_string() // Simplified since we removed the type methods
    }

    fn get_land_mover_capability(&self) -> u32 {
        100 // Default capability score since we removed max_land_speed
    }

    fn get_swimmer_name(&self) -> String {
        "Swimmer".to_string()
    }

    fn get_flyer_name(&self) -> String {
        "Flyer".to_string()
    }
}

/// Extended unified race with multiple land movement challenges
impl<L, S, F> UnifiedRaceTeam<L, S, F>
where
    L: LandMove,
    S: Swimming + HasEnergy,
    F: Flying + HasEnergy,
{
    /// Extended race that tests all 4 LandMove functions
    pub fn extended_race(&mut self) -> UnifiedRaceResult {
        println!("🌐 Team {} starts the EXTENDED unified race!", self.name);
        println!("   Testing all LandMove capabilities!");

        let mut completed_legs = 0;
        let mut abstraction_bonus = 200; // Higher bonus for extended race

        // Extended Land Movement Test
        println!("  🚶🚗 Extended Land Movement Challenge...");
        let land_start_energy = self.land_mover.energy();

        // Test 1: Basic movement
        print!("    Test 1 - Basic: ");
        let test1_result = self.land_mover.land_move();
        let test1_success = test1_result.is_ok();
        if test1_success {
            println!("✅ {}", test1_result.as_ref().unwrap());
        } else {
            println!("❌ {}", test1_result.as_ref().unwrap_err());
        }
        // Convert to consistent type
        let test1_result = test1_result.map_err(|e| e.to_string());

        // Test 2: Fast movement (if enough energy)
        print!("    Test 2 - Fast: ");
        let test2_result = if self.land_mover.energy() >= EnergyLevel::Normal {
            let result = self.land_mover.land_move_fast();
            let success = result.is_ok();
            if success {
                println!("✅ {}", result.as_ref().unwrap());
            } else {
                println!("❌ {}", result.as_ref().unwrap_err());
            }
            result.map_err(|e| e.to_string())
        } else {
            println!("⏭️  Skipped (insufficient energy)");
            Err("Insufficient energy".to_string())
        };

        // Test 3: Terrain navigation (if enough energy)
        print!("    Test 3 - Terrain: ");
        let test3_result = if self.land_mover.energy() >= EnergyLevel::Tired {
            let result = self.land_mover.navigate_terrain("rocky".into());
            let success = result.is_ok();
            if success {
                println!("✅ {}", result.as_ref().unwrap());
            } else {
                println!("❌ {}", result.as_ref().unwrap_err());
            }
            result.map_err(|e| e.to_string())
        } else {
            println!("⏭️  Skipped (insufficient energy)");
            Err("Insufficient energy".to_string())
        };

        // Test 4: Intensity movement (if enough energy)
        print!("    Test 4 - Intensity: ");
        let test4_result = if self.land_mover.energy() >= EnergyLevel::Normal {
            let result = self.land_mover.land_move_at_intensity("vigorous");
            let success = result.is_ok();
            if success {
                println!("✅ {}", result.as_ref().unwrap());
            } else {
                println!("❌ {}", result.as_ref().unwrap_err());
            }
            result.map_err(|e| e.to_string())
        } else {
            println!("⏭️  Skipped (insufficient energy)");
            Err("Insufficient energy".to_string())
        };

        // Calculate land leg success - now all results have the same type
        let tests_passed = [&test1_result, &test2_result, &test3_result, &test4_result]
            .iter()
            .filter(|r| r.is_ok())
            .count();

        let land_result = if tests_passed > 0 {
            completed_legs += 1;
            abstraction_bonus += tests_passed as u32 * 50; // Bonus for each test passed
            Ok(format!("Completed {}/4 land movement tests", tests_passed))
        } else {
            Err("Failed all land movement tests".to_string())
        };

        let land_leg = UnifiedLeg {
            participant_name: self.get_land_mover_name(),
            participant_category: format!("Land Mover ({}/4 tests)", tests_passed),
            activity: "Extended Land Movement".to_string(),
            starting_energy: land_start_energy,
            result: land_result,
            final_energy: self.land_mover.energy(),
            capability_score: tests_passed as u32 * 25,
        };

        // Regular swimming and flying legs
        let (water_leg, air_leg) = self.run_water_and_air_legs(&mut completed_legs);

        println!(
            "  🏁 Team {} completed {}/3 legs in extended race",
            self.name, completed_legs
        );

        UnifiedRaceResult {
            team_name: self.name.clone(),
            land_leg,
            water_leg,
            air_leg,
            completed_legs,
            abstraction_bonus,
        }
    }

    fn run_water_and_air_legs(&mut self, completed_legs: &mut u8) -> (UnifiedLeg, UnifiedLeg) {
        // Swimming leg
        println!("  🏊 Leg 2: Swimming...");
        let swim_start_energy = self.swimmer.energy();
        let swim_result = match self.swimmer.swim() {
            Ok(result) => {
                *completed_legs += 1;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let water_leg = UnifiedLeg {
            participant_name: self.get_swimmer_name(),
            participant_category: "Swimmer".to_string(),
            activity: "Swimming".to_string(),
            starting_energy: swim_start_energy,
            result: swim_result,
            final_energy: self.swimmer.energy(),
            capability_score: self.swimmer.max_depth(),
        };

        // Flying leg
        println!("  🛩️  Leg 3: Flying...");
        let fly_start_energy = self.flyer.energy();
        let fly_result = match self.flyer.fly() {
            Ok(result) => {
                *completed_legs += 1;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let air_leg = UnifiedLeg {
            participant_name: self.get_flyer_name(),
            participant_category: "Flyer".to_string(),
            activity: "Flying".to_string(),
            starting_energy: fly_start_energy,
            result: fly_result,
            final_energy: self.flyer.energy(),
            capability_score: self.flyer.max_altitude(),
        };

        (water_leg, air_leg)
    }
}

pub struct UnifiedRaceCompetition {
    pub results: Vec<UnifiedRaceResult>,
}

impl UnifiedRaceCompetition {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_team_result(&mut self, result: UnifiedRaceResult) {
        self.results.push(result);
    }

    pub fn display_results(&self) {
        println!("\n🌐 UNIFIED RACE COMPETITION RESULTS 🌐");
        println!("======================================");

        let mut rankings = self.results.iter().collect::<Vec<_>>();
        rankings.sort_by_key(|b| std::cmp::Reverse(b.score()));

        for (position, result) in rankings.iter().enumerate() {
            let medal = match position {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "🏁",
            };

            println!("\n{} Position {}: {}", medal, position + 1, result);
        }
    }

    pub fn analyze_abstraction_benefits(&self) {
        println!("\n🔬 ABSTRACTION BENEFITS ANALYSIS:");
        println!("=================================");

        println!("\n🚶🚗 LANDMOVE ABSTRACTION SUCCESS:");
        println!("  ✅ Single trait covers both walking and driving");
        println!("  ✅ Animals and vehicles can compete in same leg");
        println!("  ✅ Unified interface for all land-based movement");
        println!("  ✅ Consistent energy management across movement types");
        println!("  ✅ Four distinct movement capabilities tested");

        println!("\n📊 PERFORMANCE METRICS:");
        for result in &self.results {
            println!(
                "  Team {}: {} abstraction bonus points",
                result.team_name, result.abstraction_bonus
            );
        }

        println!("\n🎯 DESIGN ADVANTAGES:");
        println!("  1. 🧩 Reduced Complexity: One constraint instead of two");
        println!("  2. 🔄 Increased Flexibility: Any land mover can participate");
        println!("  3. 🎪 Natural Grouping: Logical behavior categorization");
        println!("  4. 🚀 Code Reuse: Shared implementations across types");
        println!("  5. 🛡️  Type Safety: Compile-time guarantees maintained");
        println!("  6. 🎯 Comprehensive Testing: All four movement types evaluated");
    }
}

impl Default for UnifiedRaceCompetition {
    fn default() -> Self {
        Self::new()
    }
}

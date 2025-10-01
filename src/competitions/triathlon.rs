//! Triathlon competition for animals that can walk, swim, and fly

use crate::animals::Animal;
use crate::behaviors::{Flying, Swimming, Walking};
use crate::core::{EnergyLevel, HasEnergy};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TriathlonResult {
    pub participant_name: String,
    pub species: String,
    pub starting_energy: EnergyLevel,
    pub walk_result: Result<String, String>,
    pub swim_result: Result<String, String>,
    pub fly_result: Result<String, String>,
    pub final_energy: EnergyLevel,
    pub completed_stages: u8,
    pub total_time_penalty: u32, // Higher penalty for lower energy performance
}

impl TriathlonResult {
    pub fn score(&self) -> u32 {
        let completion_bonus = self.completed_stages as u32 * 100;
        let energy_bonus = self.final_energy as u32 * 10;

        // Lower time penalty is better, so we subtract it
        completion_bonus + energy_bonus - self.total_time_penalty
    }

    pub fn is_winner(&self, other: &Self) -> bool {
        match self.completed_stages.cmp(&other.completed_stages) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => self.score() > other.score(),
        }
    }
}

impl fmt::Display for TriathlonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "🏆 {} ({}) - Triathlon Result:",
            self.participant_name, self.species
        )?;
        writeln!(f, "   Starting Energy: {}", self.starting_energy)?;

        writeln!(
            f,
            "   🚶 Walking: {}",
            self.walk_result
                .as_ref()
                .map_or_else(|e| e.as_str(), |s| s.as_str())
        )?;
        writeln!(
            f,
            "   🏊 Swimming: {}",
            self.swim_result
                .as_ref()
                .map_or_else(|e| e.as_str(), |s| s.as_str())
        )?;
        writeln!(
            f,
            "   🛩️  Flying: {}",
            self.fly_result
                .as_ref()
                .map_or_else(|e| e.as_str(), |s| s.as_str())
        )?;

        writeln!(
            f,
            "   Final Energy: {} ({})",
            self.final_energy,
            self.final_energy.to_points()
        )?;
        writeln!(f, "   Completed Stages: {}/3", self.completed_stages)?;
        writeln!(f, "   Total Score: {}", self.score())?;

        if self.completed_stages == 3 {
            writeln!(f, "   🎉 COMPLETED TRIATHLON! 🎉")?;
        } else {
            writeln!(f, "   ⚠️  Did not complete all stages")?;
        }

        Ok(())
    }
}

pub struct Triathlon {
    pub results: Vec<TriathlonResult>,
}

impl Triathlon {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Add a participant to the triathlon
    /// T must implement all three movement traits plus Animal and HasEnergy
    pub fn add_participant<T>(&mut self, participant: &mut T)
    where
        T: Walking + Swimming + Flying + Animal + HasEnergy,
    {
        let starting_energy = participant.energy();
        let participant_name = participant.name();
        let species = participant.species().to_string();

        println!(
            "🏁 {} ({}) enters the triathlon with {} energy!",
            participant_name, species, starting_energy
        );

        let mut completed_stages = 0;
        let mut total_time_penalty = 0;

        // Stage 1: Walking
        println!("  🚶 Stage 1: Walking...");
        let walk_result = match participant.walk() {
            Ok(result) => {
                completed_stages += 1;
                // Calculate time penalty based on energy after walking
                let energy_after_walk = participant.energy();
                total_time_penalty += Self::calculate_time_penalty(energy_after_walk);
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 200; // Heavy penalty for failure
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        // Stage 2: Swimming (only if still has energy)
        println!("  🏊 Stage 2: Swimming...");
        let swim_result = if participant.energy() > EnergyLevel::Collapsed {
            match participant.swim() {
                Ok(result) => {
                    completed_stages += 1;
                    let energy_after_swim = participant.energy();
                    total_time_penalty += Self::calculate_time_penalty(energy_after_swim);
                    println!("    ✅ {}", result);
                    Ok(result)
                }
                Err(e) => {
                    total_time_penalty += 200;
                    println!("    ❌ Failed: {}", e);
                    Err(e.to_string())
                }
            }
        } else {
            total_time_penalty += 200;
            println!("    ❌ Too exhausted to swim");
            Err("Too exhausted to continue".to_string())
        };

        // Stage 3: Flying (only if still has energy)
        println!("  🛩️  Stage 3: Flying...");
        let fly_result = if participant.energy() > EnergyLevel::Collapsed {
            match participant.fly() {
                Ok(result) => {
                    completed_stages += 1;
                    let energy_after_fly = participant.energy();
                    total_time_penalty += Self::calculate_time_penalty(energy_after_fly);
                    println!("    ✅ {}", result);
                    Ok(result)
                }
                Err(e) => {
                    total_time_penalty += 200;
                    println!("    ❌ Failed: {}", e);
                    Err(e.to_string())
                }
            }
        } else {
            total_time_penalty += 200;
            println!("    ❌ Too exhausted to fly");
            Err("Too exhausted to continue".to_string())
        };

        let final_energy = participant.energy();

        let result = TriathlonResult {
            participant_name,
            species,
            starting_energy,
            walk_result,
            swim_result,
            fly_result,
            final_energy,
            completed_stages,
            total_time_penalty,
        };

        println!(
            "  🏁 {} finished with {}/3 stages completed",
            result.participant_name, result.completed_stages
        );

        self.results.push(result);
    }

    fn calculate_time_penalty(energy: EnergyLevel) -> u32 {
        // Lower energy = higher time penalty
        match energy {
            EnergyLevel::Hyperactive => 10,
            EnergyLevel::Energetic => 20,
            EnergyLevel::Normal => 30,
            EnergyLevel::Tired => 50,
            EnergyLevel::Exhausted => 80,
            EnergyLevel::Collapsed => 100,
        }
    }

    pub fn get_winner(&self) -> Option<&TriathlonResult> {
        self.results
            .iter()
            .max_by(|a, b| match a.completed_stages.cmp(&b.completed_stages) {
                std::cmp::Ordering::Equal => a.score().cmp(&b.score()),
                other => other,
            })
    }

    pub fn get_rankings(&self) -> Vec<&TriathlonResult> {
        let mut ranked = self.results.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| match b.completed_stages.cmp(&a.completed_stages) {
            std::cmp::Ordering::Equal => b.score().cmp(&a.score()),
            other => other,
        });
        ranked
    }

    pub fn display_results(&self) {
        println!("\n🏆 TRIATHLON RESULTS 🏆");
        println!("========================");

        let rankings = self.get_rankings();

        for (position, result) in rankings.iter().enumerate() {
            let medal = match position {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "🏃",
            };

            println!("\n{} Position {}: {}", medal, position + 1, result);
        }

        if let Some(winner) = self.get_winner() {
            println!(
                "\n🎉 TRIATHLON CHAMPION: {} ({}) 🎉",
                winner.participant_name, winner.species
            );
            println!("   Score: {} points", winner.score());
        }
    }
}

impl Default for Triathlon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animals::Duck;

    #[test]
    fn test_triathlon() {
        let mut triathlon = Triathlon::new();
        let mut duck = Duck::new("Test Duck".to_string());

        triathlon.add_participant(&mut duck);

        assert_eq!(triathlon.results.len(), 1);
        assert_eq!(triathlon.results[0].participant_name, "Test Duck");
    }

    #[test]
    fn test_scoring() {
        let result = TriathlonResult {
            participant_name: "Test".to_string(),
            species: "Duck".to_string(),
            starting_energy: EnergyLevel::Normal,
            walk_result: Ok("success".to_string()),
            swim_result: Ok("success".to_string()),
            fly_result: Ok("success".to_string()),
            final_energy: EnergyLevel::Tired,
            completed_stages: 3,
            total_time_penalty: 100,
        };

        // 3 stages * 100 + Tired(2) * 10 - 100 penalty = 300 + 20 - 100 = 220
        assert_eq!(result.score(), 220);
    }
}

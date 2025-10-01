//! Relay (Staffel) competition where teams of different animals compete
//! Each team member contributes their specialty

use crate::animals::Animal;
use crate::behaviors::{flying::Flying, swimming::Swimming, walking::Walking};
use crate::core::{EnergyLevel, HasEnergy};
use std::fmt;

#[derive(Debug, Clone)]
pub struct RelayLeg {
    pub participant_name: String,
    pub species: String,
    pub activity: String,
    pub starting_energy: EnergyLevel,
    pub result: Result<String, String>,
    pub final_energy: EnergyLevel,
    pub time_penalty: u32,
}

impl fmt::Display for RelayLeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.result.is_ok() { "✅" } else { "❌" };
        write!(
            f,
            "    {} {} ({}): {} -> {} [{}]",
            status,
            self.participant_name,
            self.species,
            self.starting_energy,
            self.final_energy,
            self.activity
        )
    }
}

#[derive(Debug, Clone)]
pub struct RelayResult {
    pub team_name: String,
    pub swimmer: RelayLeg,
    pub walker: RelayLeg,
    pub flyer: RelayLeg,
    pub total_time_penalty: u32,
    pub completed_legs: u8,
}

impl RelayResult {
    pub fn score(&self) -> u32 {
        let completion_bonus = self.completed_legs as u32 * 150;
        let energy_bonus = (self.swimmer.final_energy as u32
            + self.walker.final_energy as u32
            + self.flyer.final_energy as u32)
            * 5;

        completion_bonus + energy_bonus - self.total_time_penalty
    }

    pub fn is_complete(&self) -> bool {
        self.completed_legs == 3
    }
}

impl fmt::Display for RelayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "🏃‍♂️ Team: {} - Relay Result:", self.team_name)?;
        writeln!(f, "  🏊 Swimming Leg:")?;
        writeln!(f, "{}", self.swimmer)?;
        writeln!(f, "  🚶 Walking Leg:")?;
        writeln!(f, "{}", self.walker)?;
        writeln!(f, "  🛩️  Flying Leg:")?;
        writeln!(f, "{}", self.flyer)?;
        writeln!(f, "  Completed Legs: {}/3", self.completed_legs)?;
        writeln!(f, "  Total Time Penalty: {}", self.total_time_penalty)?;
        writeln!(f, "  Team Score: {}", self.score())?;

        if self.is_complete() {
            writeln!(f, "  🎉 COMPLETED RELAY! 🎉")?;
        } else {
            writeln!(f, "  ⚠️  Did not complete all legs")?;
        }

        Ok(())
    }
}

/// A relay team consists of three specialists: (Swimmer, Walker, Flyer)
pub struct RelayTeam<S, W, F>
where
    S: Swimming + Animal + HasEnergy,
    W: Walking + Animal + HasEnergy,
    F: Flying + Animal + HasEnergy,
{
    pub name: String,
    pub swimmer: S,
    pub walker: W,
    pub flyer: F,
}

impl<S, W, F> RelayTeam<S, W, F>
where
    S: Swimming + Animal + HasEnergy,
    W: Walking + Animal + HasEnergy,
    F: Flying + Animal + HasEnergy,
{
    pub fn new(name: String, swimmer: S, walker: W, flyer: F) -> Self {
        Self {
            name,
            swimmer,
            walker,
            flyer,
        }
    }

    /// Execute the relay race
    pub fn race(&mut self) -> RelayResult {
        println!("🏁 Team {} starts the relay!", self.name);

        let mut total_time_penalty = 0;
        let mut completed_legs = 0;

        // Leg 1: Swimming
        println!(
            "  🏊 Leg 1: {} ({}) swimming...",
            self.swimmer.name(),
            self.swimmer.species()
        );
        let swimmer_start_energy = self.swimmer.energy();
        let swim_result = match self.swimmer.swim() {
            Ok(result) => {
                completed_legs += 1;
                let penalty = Self::calculate_time_penalty(self.swimmer.energy());
                total_time_penalty += penalty;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 300; // Heavy penalty for failure
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let swimmer_leg = RelayLeg {
            participant_name: self.swimmer.name(),
            species: self.swimmer.species().to_string(),
            activity: "Swimming".to_string(),
            starting_energy: swimmer_start_energy,
            result: swim_result,
            final_energy: self.swimmer.energy(),
            time_penalty: total_time_penalty,
        };

        // Leg 2: Walking (handoff to walker)
        println!(
            "  🚶 Leg 2: {} ({}) walking...",
            self.walker.name(),
            self.walker.species()
        );
        let walker_start_energy = self.walker.energy();
        let walk_result = match self.walker.walk() {
            Ok(result) => {
                completed_legs += 1;
                let penalty = Self::calculate_time_penalty(self.walker.energy());
                total_time_penalty += penalty;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 300;
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let walker_leg = RelayLeg {
            participant_name: self.walker.name(),
            species: self.walker.species().to_string(),
            activity: "Walking".to_string(),
            starting_energy: walker_start_energy,
            result: walk_result,
            final_energy: self.walker.energy(),
            time_penalty: Self::calculate_time_penalty(self.walker.energy()),
        };

        // Leg 3: Flying (handoff to flyer)
        println!(
            "  🛩️  Leg 3: {} ({}) flying...",
            self.flyer.name(),
            self.flyer.species()
        );
        let flyer_start_energy = self.flyer.energy();
        let fly_result = match self.flyer.fly() {
            Ok(result) => {
                completed_legs += 1;
                let penalty = Self::calculate_time_penalty(self.flyer.energy());
                total_time_penalty += penalty;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 300;
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let flyer_leg = RelayLeg {
            participant_name: self.flyer.name(),
            species: self.flyer.species().to_string(),
            activity: "Flying".to_string(),
            starting_energy: flyer_start_energy,
            result: fly_result,
            final_energy: self.flyer.energy(),
            time_penalty: Self::calculate_time_penalty(self.flyer.energy()),
        };

        println!(
            "  🏁 Team {} completed {}/3 legs",
            self.name, completed_legs
        );

        RelayResult {
            team_name: self.name.clone(),
            swimmer: swimmer_leg,
            walker: walker_leg,
            flyer: flyer_leg,
            total_time_penalty,
            completed_legs,
        }
    }

    fn calculate_time_penalty(energy: EnergyLevel) -> u32 {
        match energy {
            EnergyLevel::Hyperactive => 5,
            EnergyLevel::Energetic => 15,
            EnergyLevel::Normal => 25,
            EnergyLevel::Tired => 40,
            EnergyLevel::Exhausted => 70,
            EnergyLevel::Collapsed => 100,
        }
    }
}

pub struct RelayCompetition {
    pub results: Vec<RelayResult>,
}

impl RelayCompetition {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_team_result(&mut self, result: RelayResult) {
        self.results.push(result);
    }

    pub fn get_winner(&self) -> Option<&RelayResult> {
        self.results
            .iter()
            .max_by(|a, b| match a.completed_legs.cmp(&b.completed_legs) {
                std::cmp::Ordering::Equal => a.score().cmp(&b.score()),
                other => other,
            })
    }

    pub fn get_rankings(&self) -> Vec<&RelayResult> {
        let mut ranked = self.results.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| match b.completed_legs.cmp(&a.completed_legs) {
            std::cmp::Ordering::Equal => b.score().cmp(&a.score()),
            other => other,
        });
        ranked
    }

    pub fn display_results(&self) {
        println!("\n🏃‍♂️ RELAY COMPETITION RESULTS 🏃‍♂️");
        println!("===============================");

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
            println!("\n🎉 RELAY CHAMPIONS: Team {} 🎉", winner.team_name);
            println!("   Score: {} points", winner.score());
        }
    }
}

impl Default for RelayCompetition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animals::{Dog, Duck, Eagle};

    #[test]
    fn test_relay_team() {
        let duck = Duck::new("Swimming Duck".to_string());
        let dog = Dog::new("Running Dog".to_string(), "Greyhound".to_string());
        let eagle = Eagle::new("Flying Eagle".to_string());

        let mut team = RelayTeam::new("Mixed Team".to_string(), duck, dog, eagle);

        let result = team.race();
        assert_eq!(result.team_name, "Mixed Team");
        assert!(result.completed_legs <= 3);
    }
}

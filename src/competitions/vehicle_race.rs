//! Vehicle Race Competition
//!
//! This module implements various vehicle racing competitions that demonstrate
//! trait composition across different vehicle types and capabilities.

use crate::behaviors::{driving::Driving, flying::Flying, swimming::Swimming};
use crate::core::{EnergyLevel, HasEnergy};
use crate::vehicles::Vehicle;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RaceLeg {
    pub participant_name: String,
    pub vehicle_type: String,
    pub activity: String,
    pub starting_energy: EnergyLevel,
    pub result: Result<String, String>,
    pub final_energy: EnergyLevel,
    pub time_penalty: u32,
    pub max_capability: u32, // max speed, depth, or altitude
}

impl fmt::Display for RaceLeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.result.is_ok() { "✅" } else { "❌" };
        write!(
            f,
            "    {} {} ({}): {} -> {} [{}] (Max: {})",
            status,
            self.participant_name,
            self.vehicle_type,
            self.starting_energy,
            self.final_energy,
            self.activity,
            self.max_capability
        )
    }
}

#[derive(Debug, Clone)]
pub struct VehicleRaceResult {
    pub team_name: String,
    pub land_leg: RaceLeg,
    pub water_leg: RaceLeg,
    pub air_leg: RaceLeg,
    pub total_time_penalty: u32,
    pub completed_legs: u8,
}

impl VehicleRaceResult {
    pub fn score(&self) -> u32 {
        let completion_bonus = self.completed_legs as u32 * 200;
        let energy_bonus = (self.land_leg.final_energy as u32
            + self.water_leg.final_energy as u32
            + self.air_leg.final_energy as u32)
            * 8;
        let capability_bonus = (self.land_leg.max_capability
            + self.water_leg.max_capability
            + self.air_leg.max_capability)
            / 10;

        completion_bonus + energy_bonus + capability_bonus - self.total_time_penalty
    }

    pub fn is_complete(&self) -> bool {
        self.completed_legs == 3
    }
}

impl fmt::Display for VehicleRaceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "🏁 Team: {} - Vehicle Race Result:", self.team_name)?;
        writeln!(f, "  🚗 Land Leg (Driving):")?;
        writeln!(f, "{}", self.land_leg)?;
        writeln!(f, "  🚢 Water Leg (Swimming):")?;
        writeln!(f, "{}", self.water_leg)?;
        writeln!(f, "  ✈️  Air Leg (Flying):")?;
        writeln!(f, "{}", self.air_leg)?;
        writeln!(f, "  Completed Legs: {}/3", self.completed_legs)?;
        writeln!(f, "  Total Time Penalty: {}", self.total_time_penalty)?;
        writeln!(f, "  Team Score: {}", self.score())?;

        if self.is_complete() {
            writeln!(f, "  🏆 COMPLETED VEHICLE RACE! 🏆")?;
        } else {
            writeln!(f, "  ⚠️  Did not complete all legs")?;
        }

        Ok(())
    }
}

/// Individual vehicle triathlon for multi-capability vehicles
#[derive(Debug, Clone)]
pub struct IndividualRaceResult {
    pub participant_name: String,
    pub vehicle_type: String,
    pub starting_energy: EnergyLevel,
    pub drive_result: Result<String, String>,
    pub swim_result: Result<String, String>,
    pub fly_result: Result<String, String>,
    pub final_energy: EnergyLevel,
    pub completed_stages: u8,
    pub total_time_penalty: u32,
    pub max_speed: u32,
    pub max_depth: u32,
    pub max_altitude: u32,
}

impl IndividualRaceResult {
    pub fn score(&self) -> u32 {
        let completion_bonus = self.completed_stages as u32 * 250;
        let energy_bonus = self.final_energy as u32 * 15;
        let versatility_bonus = (self.max_speed + self.max_depth + self.max_altitude) / 5;

        completion_bonus + energy_bonus + versatility_bonus - self.total_time_penalty
    }
}

impl fmt::Display for IndividualRaceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "🏎️ {} ({}) - Individual Race Result:",
            self.participant_name, self.vehicle_type
        )?;
        writeln!(f, "   Starting Energy: {}", self.starting_energy)?;

        writeln!(
            f,
            "   🚗 Driving: {}",
            self.drive_result
                .as_ref()
                .map_or_else(|e| e.as_str(), |s| s.as_str())
        )?;
        writeln!(
            f,
            "   🚢 Swimming: {}",
            self.swim_result
                .as_ref()
                .map_or_else(|e| e.as_str(), |s| s.as_str())
        )?;
        writeln!(
            f,
            "   ✈️  Flying: {}",
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
        writeln!(
            f,
            "   Capabilities: Speed {}km/h, Depth {}m, Altitude {}m",
            self.max_speed, self.max_depth, self.max_altitude
        )?;
        writeln!(f, "   Total Score: {}", self.score())?;

        if self.completed_stages == 3 {
            writeln!(f, "   🎉 COMPLETED INDIVIDUAL TRIATHLON! 🎉")?;
        } else {
            writeln!(f, "   ⚠️  Did not complete all stages")?;
        }

        Ok(())
    }
}

/// A vehicle race team consists of three specialists: (Driver, Swimmer, Flyer)
pub struct VehicleRaceTeam<D, S, F>
where
    D: Driving + Vehicle + HasEnergy,
    S: Swimming + Vehicle + HasEnergy,
    F: Flying + Vehicle + HasEnergy,
{
    pub name: String,
    pub driver: D,
    pub swimmer: S,
    pub flyer: F,
}

impl<D, S, F> VehicleRaceTeam<D, S, F>
where
    D: Driving + Vehicle + HasEnergy,
    S: Swimming + Vehicle + HasEnergy,
    F: Flying + Vehicle + HasEnergy,
{
    pub fn new(name: String, driver: D, swimmer: S, flyer: F) -> Self {
        Self {
            name,
            driver,
            swimmer,
            flyer,
        }
    }

    /// Execute the vehicle race
    pub fn race(&mut self) -> VehicleRaceResult {
        println!("🏁 Team {} starts the vehicle race!", self.name);

        let mut total_time_penalty = 0;
        let mut completed_legs = 0;

        // Leg 1: Land driving
        println!(
            "  🚗 Leg 1: {} ({}) driving on land...",
            self.driver.name(),
            self.driver.vehicle_type()
        );
        let driver_start_energy = self.driver.energy();
        let drive_result = match self.driver.drive() {
            Ok(result) => {
                completed_legs += 1;
                let penalty = Self::calculate_time_penalty(self.driver.energy());
                total_time_penalty += penalty;
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 400; // Heavy penalty for failure
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let driver_leg = RaceLeg {
            participant_name: self.driver.name(),
            vehicle_type: self.driver.vehicle_type().to_string(),
            activity: "Driving".to_string(),
            starting_energy: driver_start_energy,
            result: drive_result,
            final_energy: self.driver.energy(),
            time_penalty: Self::calculate_time_penalty(self.driver.energy()),
            max_capability: self.driver.max_speed(),
        };

        // Leg 2: Water swimming
        println!(
            "  🚢 Leg 2: {} ({}) swimming in water...",
            self.swimmer.name(),
            self.swimmer.vehicle_type()
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
                total_time_penalty += 400;
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let swimmer_leg = RaceLeg {
            participant_name: self.swimmer.name(),
            vehicle_type: self.swimmer.vehicle_type().to_string(),
            activity: "Swimming".to_string(),
            starting_energy: swimmer_start_energy,
            result: swim_result,
            final_energy: self.swimmer.energy(),
            time_penalty: Self::calculate_time_penalty(self.swimmer.energy()),
            max_capability: self.swimmer.max_depth(),
        };

        // Leg 3: Air flying
        println!(
            "  ✈️  Leg 3: {} ({}) flying in air...",
            self.flyer.name(),
            self.flyer.vehicle_type()
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
                total_time_penalty += 400;
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        let flyer_leg = RaceLeg {
            participant_name: self.flyer.name(),
            vehicle_type: self.flyer.vehicle_type().to_string(),
            activity: "Flying".to_string(),
            starting_energy: flyer_start_energy,
            result: fly_result,
            final_energy: self.flyer.energy(),
            time_penalty: Self::calculate_time_penalty(self.flyer.energy()),
            max_capability: self.flyer.max_altitude(),
        };

        println!(
            "  🏁 Team {} completed {}/3 legs",
            self.name, completed_legs
        );

        VehicleRaceResult {
            team_name: self.name.clone(),
            land_leg: driver_leg,
            water_leg: swimmer_leg,
            air_leg: flyer_leg,
            total_time_penalty,
            completed_legs,
        }
    }

    fn calculate_time_penalty(energy: EnergyLevel) -> u32 {
        match energy {
            EnergyLevel::Hyperactive => 10,
            EnergyLevel::Energetic => 20,
            EnergyLevel::Normal => 35,
            EnergyLevel::Tired => 60,
            EnergyLevel::Exhausted => 100,
            EnergyLevel::Collapsed => 150,
        }
    }
}

/// Individual triathlon for vehicles that can do all three activities
pub struct VehicleTriathlon {
    pub results: Vec<IndividualRaceResult>,
}

impl VehicleTriathlon {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Add a participant to the individual triathlon
    pub fn add_participant<T>(&mut self, participant: &mut T)
    where
        T: Driving + Swimming + Flying + Vehicle + HasEnergy,
    {
        let starting_energy = participant.energy();
        let participant_name = participant.name();
        let vehicle_type = participant.vehicle_type().to_string();
        let max_speed = participant.max_speed();
        let max_depth = participant.max_depth();
        let max_altitude = participant.max_altitude();

        println!(
            "🏁 {} ({}) enters the individual vehicle triathlon with {} energy!",
            participant_name, vehicle_type, starting_energy
        );

        let mut completed_stages = 0;
        let mut total_time_penalty = 0;

        // Stage 1: Driving
        println!("  🚗 Stage 1: Driving...");
        let drive_result = match participant.drive() {
            Ok(result) => {
                completed_stages += 1;
                let energy_after = participant.energy();
                total_time_penalty += Self::calculate_time_penalty(energy_after);
                println!("    ✅ {}", result);
                Ok(result)
            }
            Err(e) => {
                total_time_penalty += 300;
                println!("    ❌ Failed: {}", e);
                Err(e.to_string())
            }
        };

        // Stage 2: Swimming
        println!("  🚢 Stage 2: Swimming...");
        let swim_result = if participant.energy() > EnergyLevel::Collapsed {
            match participant.swim() {
                Ok(result) => {
                    completed_stages += 1;
                    let energy_after = participant.energy();
                    total_time_penalty += Self::calculate_time_penalty(energy_after);
                    println!("    ✅ {}", result);
                    Ok(result)
                }
                Err(e) => {
                    total_time_penalty += 300;
                    println!("    ❌ Failed: {}", e);
                    Err(e.to_string())
                }
            }
        } else {
            total_time_penalty += 300;
            println!("    ❌ Too exhausted to swim");
            Err("Too exhausted to continue".to_string())
        };

        // Stage 3: Flying
        println!("  ✈️  Stage 3: Flying...");
        let fly_result = if participant.energy() > EnergyLevel::Collapsed {
            match participant.fly() {
                Ok(result) => {
                    completed_stages += 1;
                    let energy_after = participant.energy();
                    total_time_penalty += Self::calculate_time_penalty(energy_after);
                    println!("    ✅ {}", result);
                    Ok(result)
                }
                Err(e) => {
                    total_time_penalty += 300;
                    println!("    ❌ Failed: {}", e);
                    Err(e.to_string())
                }
            }
        } else {
            total_time_penalty += 300;
            println!("    ❌ Too exhausted to fly");
            Err("Too exhausted to continue".to_string())
        };

        let final_energy = participant.energy();

        let result = IndividualRaceResult {
            participant_name,
            vehicle_type,
            starting_energy,
            drive_result,
            swim_result,
            fly_result,
            final_energy,
            completed_stages,
            total_time_penalty,
            max_speed,
            max_depth,
            max_altitude,
        };

        println!(
            "  🏁 {} finished with {}/3 stages completed",
            result.participant_name, result.completed_stages
        );

        self.results.push(result);
    }

    fn calculate_time_penalty(energy: EnergyLevel) -> u32 {
        match energy {
            EnergyLevel::Hyperactive => 15,
            EnergyLevel::Energetic => 25,
            EnergyLevel::Normal => 40,
            EnergyLevel::Tired => 70,
            EnergyLevel::Exhausted => 110,
            EnergyLevel::Collapsed => 150,
        }
    }

    pub fn get_winner(&self) -> Option<&IndividualRaceResult> {
        self.results
            .iter()
            .max_by(|a, b| match a.completed_stages.cmp(&b.completed_stages) {
                std::cmp::Ordering::Equal => a.score().cmp(&b.score()),
                other => other,
            })
    }

    pub fn get_rankings(&self) -> Vec<&IndividualRaceResult> {
        let mut ranked = self.results.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| match b.completed_stages.cmp(&a.completed_stages) {
            std::cmp::Ordering::Equal => b.score().cmp(&a.score()),
            other => other,
        });
        ranked
    }

    pub fn display_results(&self) {
        println!("\n🏎️ INDIVIDUAL VEHICLE TRIATHLON RESULTS 🏎️");
        println!("============================================");

        let rankings = self.get_rankings();

        for (position, result) in rankings.iter().enumerate() {
            let medal = match position {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "🏁",
            };

            println!("\n{} Position {}: {}", medal, position + 1, result);
        }

        if let Some(winner) = self.get_winner() {
            println!(
                "\n🎉 INDIVIDUAL TRIATHLON CHAMPION: {} ({}) 🎉",
                winner.participant_name, winner.vehicle_type
            );
            println!("   Score: {} points", winner.score());
        }
    }
}

impl Default for VehicleTriathlon {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VehicleRaceCompetition {
    pub results: Vec<VehicleRaceResult>,
}

impl VehicleRaceCompetition {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_team_result(&mut self, result: VehicleRaceResult) {
        self.results.push(result);
    }

    pub fn get_winner(&self) -> Option<&VehicleRaceResult> {
        self.results
            .iter()
            .max_by(|a, b| match a.completed_legs.cmp(&b.completed_legs) {
                std::cmp::Ordering::Equal => a.score().cmp(&b.score()),
                other => other,
            })
    }

    pub fn get_rankings(&self) -> Vec<&VehicleRaceResult> {
        let mut ranked = self.results.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| match b.completed_legs.cmp(&a.completed_legs) {
            std::cmp::Ordering::Equal => b.score().cmp(&a.score()),
            other => other,
        });
        ranked
    }

    pub fn display_results(&self) {
        println!("\n🏁 VEHICLE RACE TEAM COMPETITION RESULTS 🏁");
        println!("===========================================");

        let rankings = self.get_rankings();

        for (position, result) in rankings.iter().enumerate() {
            let medal = match position {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "🏁",
            };

            println!("\n{} Position {}: {}", medal, position + 1, result);
        }

        if let Some(winner) = self.get_winner() {
            println!(
                "\n🏆 VEHICLE RACE TEAM CHAMPIONS: Team {} 🏆",
                winner.team_name
            );
            println!("   Score: {} points", winner.score());
        }
    }
}

impl Default for VehicleRaceCompetition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicles::{airplane::Airplane, car::Car, ship::Ship};

    #[test]
    fn test_vehicle_race_team() {
        let car = Car::new(
            "Test Car".to_string(),
            "Test Motors".to_string(),
            2023,
            crate::vehicles::car::EngineType::Gasoline {
                cylinders: 4,
                displacement: 2.0,
            },
        );
        let ship = Ship::new(
            "Test Ship".to_string(),
            "Test Shipyard".to_string(),
            2023,
            crate::vehicles::ship::ShipType::Speedboat,
            10,
            crate::vehicles::ship::PropulsionType::Diesel {
                engines: 1,
                power_each: 300,
            },
        );
        let airplane = Airplane::new(
            "Test Plane".to_string(),
            "Test Aviation".to_string(),
            2023,
            crate::vehicles::airplane::AirplaneType::Private,
            10,
            crate::vehicles::airplane::AirplaneEngine::Piston {
                engines: 1,
                power_each: 200,
            },
        );

        let mut team = VehicleRaceTeam::new("Test Team".to_string(), car, ship, airplane);

        let result = team.race();
        assert_eq!(result.team_name, "Test Team");
        assert!(result.completed_legs <= 3);
    }
}

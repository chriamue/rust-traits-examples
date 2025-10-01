//! Unified Race Competition Example
//!
//! This example demonstrates how intermediate traits like LandMove
//! can elegantly abstract over similar behaviors, allowing for more
//! flexible and natural competition design.

use rust_traits_examples::vehicles::{airplane, car, helicopter, ship};
use rust_traits_examples::{
    animals::*,
    behaviors::land_move::LandMove,
    behaviors::*,
    competitions::unified_race::*,
    core::{EnergyLevel, HasEnergy},
    vehicles::*,
};

fn main() {
    println!("🌐 UNIFIED RACE CHAMPIONSHIP 🌐");
    println!("==============================");
    println!("Featuring the revolutionary LandMove abstraction!");
    println!();

    demonstrate_landmove_abstraction();

    println!("\n{}", "=".repeat(60));

    run_unified_competitions();

    println!("\n{}", "=".repeat(60));

    analyze_abstraction_power();
}

fn demonstrate_landmove_abstraction() {
    println!("🚶🚗 LANDMOVE ABSTRACTION CONCEPT");
    println!("=================================");

    println!("The LandMove trait unifies walking and driving:");
    println!();

    println!("🐾 WALKING ANIMALS:");
    println!("  ✅ Dogs, Eagles, Ducks, Penguins can walk");
    println!("  ✅ All implement LandMove directly through sealed trait approach");
    println!("  ✅ Speed: 3-15 km/h depending on energy");
    println!("  ✅ High efficiency: ~100 km per energy unit");

    println!("\n🚗 DRIVING VEHICLES:");
    println!("  ✅ Cars, Motorcycles, Airplanes can drive");
    println!("  ✅ All implement LandMove directly through sealed trait approach");
    println!("  ✅ Speed: 80-200+ km/h depending on vehicle type");
    println!("  ✅ Variable efficiency: 15-120 km per energy unit");

    println!("\n🌐 UNIFIED INTERFACE:");
    println!("  ```rust");
    println!("  pub trait LandMove: Moving + HasEnergy + sealed::Sealed {{");
    println!("      fn max_land_speed(&self) -> u32;");
    println!("      fn land_efficiency(&self) -> u32;");
    println!("      fn land_mover_name(&self) -> String;");
    println!("      fn land_mover_type(&self) -> String;");
    println!("      fn land_move(&mut self) -> LandMoveResult;");
    println!("      fn navigate_terrain(&mut self, terrain: &str) -> LandMoveResult;");
    println!("  }}");
    println!("  ```");

    println!("\n🎯 COMPETITION BENEFITS:");
    println!("  ✅ Animals and vehicles can compete in same leg");
    println!("  ✅ Single trait constraint: L: LandMove");
    println!("  ✅ Natural abstraction over related behaviors");
    println!("  ✅ Consistent interface for all land movement");
}

fn run_unified_competitions() {
    println!("🏁 UNIFIED RACE COMPETITIONS");
    println!("============================");

    let mut competition = UnifiedRaceCompetition::new();

    // Team 1: Mixed Animal-Vehicle Team
    println!("🌿 TEAM 1: 'Nature Meets Technology'");
    println!("  Strategy: Walking animal + Swimming animal + Flying vehicle");

    let mut dog = Dog::new("Land Runner".to_string(), "Border Collie".to_string());
    dog.set_energy(EnergyLevel::Energetic);

    let mut whale = Whale::new("Ocean Cruiser".to_string());
    whale.set_energy(EnergyLevel::Normal);

    let mut fighter_jet = Airplane::new(
        "Sky Blazer".to_string(),
        "AeroTech".to_string(),
        2023,
        airplane::AirplaneType::Military,
        15,
        airplane::AirplaneEngine::Jet {
            engines: 2,
            thrust_each: 400,
        },
    );
    fighter_jet.set_energy(EnergyLevel::Normal);

    println!(
        "    🐕 Land Mover: {} ({}) - Max Speed: {} km/h",
        dog.land_mover_name(),
        dog.land_mover_type(),
        dog.max_land_speed()
    );
    println!(
        "    🐋 Swimmer: {} - Max Depth: {} m",
        whale.name(),
        whale.max_depth()
    );
    println!(
        "    ✈️  Flyer: {} - Max Altitude: {} m",
        fighter_jet.name(),
        fighter_jet.max_altitude()
    );

    let mut team1 = UnifiedRaceTeam::new(
        "Nature Meets Technology".to_string(),
        dog,         // Dog directly implements LandMove
        whale,       // Swimming
        fighter_jet, // Flying
    );

    // Team 2: All-Vehicle Team
    println!("\n🚗 TEAM 2: 'Mechanical Mastery'");
    println!("  Strategy: Driving vehicle + Swimming vehicle + Flying vehicle");

    let mut sports_car = Car::new(
        "Road Warrior".to_string(),
        "SpeedTech".to_string(),
        2023,
        car::EngineType::Gasoline {
            cylinders: 8,
            displacement: 4.0,
        },
    );
    sports_car.set_energy(EnergyLevel::Normal);

    let mut speedboat = Ship::new(
        "Wave Cutter".to_string(),
        "Marine Dynamics".to_string(),
        2023,
        ship::ShipType::Speedboat,
        8,
        ship::PropulsionType::Diesel {
            engines: 3,
            power_each: 600,
        },
    );
    speedboat.set_energy(EnergyLevel::Normal);

    let mut helicopter = Helicopter::new(
        "Rotor Master".to_string(),
        "Vertical Flight".to_string(),
        2023,
        helicopter::HelicopterType::Military,
        18,
        helicopter::HelicopterEngine::Turboshaft {
            engines: 2,
            power_each: 1500,
        },
    );
    helicopter.set_energy(EnergyLevel::Normal);

    println!(
        "    🏎️  Land Mover: {} ({}) - Max Speed: {} km/h",
        sports_car.land_mover_name(),
        sports_car.land_mover_type(),
        sports_car.max_land_speed()
    );
    println!(
        "    🚤 Swimmer: {} - Surface Speed Specialist (Max Depth: {}m)",
        speedboat.name(),
        speedboat.max_depth()
    );
    println!(
        "    🚁 Flyer: {} - Max Altitude: {} m",
        helicopter.name(),
        helicopter.max_altitude()
    );

    let mut team2 = UnifiedRaceTeam::new(
        "Mechanical Mastery".to_string(),
        sports_car, // Car directly implements LandMove
        speedboat,  // Swimming
        helicopter, // Flying
    );

    // Team 3: Balanced Hybrid Team
    println!("\n⚖️  TEAM 3: 'Balanced Hybrid'");
    println!("  Strategy: Vehicle driver + Animal swimmer + Animal flyer");

    let mut electric_car = Car::new(
        "Silent Speedster".to_string(),
        "EcoMotors".to_string(),
        2023,
        car::EngineType::Electric {
            battery_capacity: 100,
        },
    );
    electric_car.set_energy(EnergyLevel::Normal);

    let mut penguin = Penguin::new("Ice Swimmer".to_string());
    penguin.set_energy(EnergyLevel::Energetic);

    let mut eagle = Eagle::new("Sky Hunter".to_string());
    eagle.set_energy(EnergyLevel::Hyperactive);

    println!(
        "    🚗 Land Mover: {} ({}) - Efficiency: {} km/energy",
        electric_car.land_mover_name(),
        electric_car.land_mover_type(),
        electric_car.land_efficiency()
    );
    println!(
        "    🐧 Swimmer: {} - Max Depth: {} m",
        penguin.name(),
        penguin.max_depth()
    );
    println!(
        "    🦅 Flyer: {} - Max Altitude: {} m",
        eagle.name(),
        eagle.max_altitude()
    );

    let mut team3 = UnifiedRaceTeam::new(
        "Balanced Hybrid".to_string(),
        electric_car, // Car directly implements LandMove
        penguin,      // Swimming
        eagle,        // Flying
    );

    println!("\n🏁 UNIFIED RACES START!");
    println!("=======================");

    // Run races
    println!("\n📊 RACE 1: Nature Meets Technology");
    println!("{}", "-".repeat(40));
    let result1 = team1.race();

    println!("\n📊 RACE 2: Mechanical Mastery");
    println!("{}", "-".repeat(35));
    let result2 = team2.race();

    println!("\n📊 RACE 3: Balanced Hybrid");
    println!("{}", "-".repeat(30));
    let result3 = team3.race();

    // Add results
    competition.add_team_result(result1);
    competition.add_team_result(result2);
    competition.add_team_result(result3);

    // Display results
    competition.display_results();
    competition.analyze_abstraction_benefits();
}

fn analyze_abstraction_power() {
    println!("🔬 ABSTRACTION POWER ANALYSIS");
    println!("=============================");

    println!("\n🎯 LANDMOVE TRAIT ACHIEVEMENTS:");
    println!("===============================");

    println!("\n1. 🧩 UNIFIED ABSTRACTION:");
    println!("   ✅ Single trait covers walking (biological) and driving (mechanical)");
    println!("   ✅ Consistent interface for all land-based movement");
    println!("   ✅ Natural grouping of related behaviors");

    println!("\n2. 🔄 SEALED TRAIT IMPLEMENTATION:");
    println!("   Individual implementations for each concrete type:");
    println!("   ```rust");
    println!("   impl sealed::Sealed for Dog {{}}");
    println!("   impl LandMove for Dog {{ /* walking behavior */ }}");
    println!("   ");
    println!("   impl sealed::Sealed for Car {{}}");
    println!("   impl LandMove for Car {{ /* driving behavior */ }}");
    println!("   ```");
    println!("   ✅ No conflicting blanket implementations");
    println!("   ✅ Explicit control over which types can be land movers");
    println!("   ✅ Direct trait implementation without wrappers");

    println!("\n3. 🎪 COMPETITION SIMPLIFICATION:");
    println!("   Before LandMove (hypothetical, not valid Rust):");
    println!("   ```rust");
    println!("   // Complex constraint - either walking OR driving");
    println!("   struct Team<L> where L: (Walking | Driving) + HasEnergy // ❌ Not valid Rust");
    println!("   ```");
    println!("   ");
    println!("   After LandMove:");
    println!("   ```rust");
    println!("   // Simple, elegant constraint");
    println!("   struct UnifiedRaceTeam<L> where L: LandMove {{ }} // ✅ Clean and clear");
    println!("   ```");

    println!("\n4. 🌍 CROSS-DOMAIN COMPATIBILITY:");
    println!("   ✅ Animals and vehicles compete naturally in same leg");
    println!("   ✅ Fair comparison based on land movement capability");
    println!("   ✅ No artificial distinctions between biological and mechanical");

    println!("\n5. 🚀 EXTENSIBILITY:");
    println!("   Adding new land movers requires explicit opt-in:");
    println!("   ```rust");
    println!("   struct Bicycle {{ /* ... */ }}");
    println!("   impl sealed::Sealed for Bicycle {{}}");
    println!("   impl LandMove for Bicycle {{");
    println!("       fn max_land_speed(&self) -> u32 {{ 25 }}");
    println!("       // ... other methods");
    println!("   }}");
    println!("   // ✅ Bicycle can now participate in land movement!");
    println!("   ```");

    println!("\n🏆 DESIGN PATTERN SUCCESS:");
    println!("==========================");

    println!("\n📚 LESSONS LEARNED:");
    println!("   1. 🎯 Sealed traits prevent conflicting implementations");
    println!("   2. 🧩 Individual implementations provide maximum control");
    println!("   3. 🌐 Abstract over similarities, preserve differences");
    println!("   4. 🔄 Explicit trait boundaries enable flexible designs");
    println!("   5. 🛡️  Type safety maintained while increasing flexibility");

    println!("\n🌟 REAL-WORLD APPLICATIONS:");
    println!("   🚦 Traffic Management: All land vehicles under one interface");
    println!("   🎮 Game Development: Unified movement for all land entities");
    println!("   🤖 Robotics: Abstract over legs, wheels, tracks, etc.");
    println!("   📱 UI Frameworks: Abstract over different input methods");

    println!("\n🎉 SEALED TRAIT SUCCESS! 🎉");
    println!("================================");
    println!("🌟 LandMove elegantly unifies walking and driving!");
    println!("🌟 Animals and vehicles compete naturally together!");
    println!("🌟 Sealed traits provide explicit control and safety!");
    println!("🌟 The power of thoughtful abstraction in Rust! 🦀");
}

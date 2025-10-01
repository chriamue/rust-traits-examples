//! Vehicle Race Competition Example
//!
//! This example demonstrates vehicle racing competitions that showcase
//! trait composition across different vehicle types and capabilities.

use rust_traits_examples::{
    behaviors::{driving::Driving, flying::Flying, swimming::Swimming},
    competitions::vehicle_race::*,
    vehicles::*,
};

fn main() {
    println!("🏁 VEHICLE RACE CHAMPIONSHIP 🏁");
    println!("===============================");
    println!();

    run_individual_triathlon();

    println!("\n{}", "=".repeat(60));

    run_team_relay_race();

    println!("\n{}", "=".repeat(60));

    demonstrate_race_strategies();

    println!("\n{}", "=".repeat(60));

    final_race_analysis();
}

fn run_individual_triathlon() {
    println!("🏎️ INDIVIDUAL VEHICLE TRIATHLON");
    println!("===============================");
    println!("Only vehicles that can drive, swim, AND fly can compete!");
    println!();

    let triathlon = VehicleTriathlon::new();

    println!("🌟 MULTI-CAPABILITY CONTESTANTS:");

    // For this demo, we need to create a special amphibious vehicle that can also fly
    // Let's create a fictional flying amphibious vehicle
    println!("🚫 CHALLENGE: Finding vehicles that can do ALL THREE activities");
    println!("   Most real vehicles specialize in 1-2 domains, not all 3!");
    println!();
    println!("   Real examples of limitations:");
    println!("   - 🚗 Cars: Drive only");
    println!("   - 🚢 Ships: Swim only");
    println!("   - 🚁 Helicopters: Fly only");
    println!("   - ✈️  Airplanes: Fly + Drive (taxi), but can't swim");
    println!("   - 🌊 Amphibious vehicles: Drive + Swim, but can't fly");
    println!();

    println!("💡 THEORETICAL MULTI-CAPABILITY VEHICLE:");
    println!("   For demonstration, imagine a fictional 'Super Vehicle' that could:");
    println!("   ✅ Drive on roads");
    println!("   ✅ Swim/navigate in water");
    println!("   ✅ Fly in air (like a flying car concept)");
    println!();
    println!("   This would require implementing:");
    println!("   - Driving trait (land mobility)");
    println!("   - Swimming trait (water mobility)");
    println!("   - Flying trait (air mobility)");
    println!();

    println!("📊 REALITY CHECK:");
    println!("   In our current vehicle system:");
    println!("   - Only 0 vehicles can compete in individual triathlon");
    println!("   - This demonstrates the rarity of true multi-domain capability");
    println!("   - Engineering trade-offs make jack-of-all-trades vehicles challenging");

    triathlon.display_results();
    analyze_individual_results(&triathlon);
}

fn run_team_relay_race() {
    println!("🏁 VEHICLE RACE TEAM RELAY");
    println!("==========================");
    println!("Teams of specialists: Driver → Swimmer → Flyer");
    println!();

    let mut competition = VehicleRaceCompetition::new();

    // Team 1: Speed Demons
    println!("🔥 TEAM 1: 'Speed Demons'");
    println!("  Strategy: Maximum speed in each domain");

    let sports_car = Car::new(
        "Velocity Viper".to_string(),
        "SpeedCorp".to_string(),
        2023,
        car::EngineType::Gasoline {
            cylinders: 8,
            displacement: 5.0,
        },
    );

    let speedboat = Ship::new(
        "Hydro Rocket".to_string(),
        "AquaSpeed".to_string(),
        2023,
        ship::ShipType::Speedboat,
        5,
        ship::PropulsionType::Diesel {
            engines: 2,
            power_each: 800,
        },
    );

    let fighter_jet = Airplane::new(
        "Sky Streak".to_string(),
        "Military Aviation".to_string(),
        2023,
        airplane::AirplaneType::Military,
        12,
        airplane::AirplaneEngine::Jet {
            engines: 2,
            thrust_each: 200,
        },
    );

    println!(
        "    🚗 Driver: {} - Max Speed: {} km/h",
        sports_car.name(),
        sports_car.max_speed()
    );
    println!(
        "    🚢 Swimmer: {} - Max Depth: {} m",
        speedboat.name(),
        speedboat.max_depth()
    );
    println!(
        "    ✈️  Flyer: {} - Max Altitude: {} m",
        fighter_jet.name(),
        fighter_jet.max_altitude()
    );

    let mut team1 = VehicleRaceTeam::new(
        "Speed Demons".to_string(),
        sports_car,
        speedboat,
        fighter_jet,
    );

    // Team 2: Efficiency Experts
    println!("\n⚡ TEAM 2: 'Efficiency Experts'");
    println!("  Strategy: Maximum fuel efficiency and endurance");

    let hybrid_car = Car::new(
        "Eco Cruiser".to_string(),
        "GreenTech".to_string(),
        2023,
        car::EngineType::Hybrid {
            gas_engine: Box::new(car::EngineType::Gasoline {
                cylinders: 4,
                displacement: 1.8,
            }),
            electric_motor: 80,
        },
    );

    let yacht = Ship::new(
        "Wind Sailor".to_string(),
        "EcoMarine".to_string(),
        2023,
        ship::ShipType::Yacht,
        30,
        ship::PropulsionType::Wind { sails: 3 },
    );

    let cargo_plane = Airplane::new(
        "Long Hauler".to_string(),
        "Cargo Express".to_string(),
        2023,
        airplane::AirplaneType::Cargo,
        40,
        airplane::AirplaneEngine::Turboprop {
            engines: 4,
            power_each: 500,
        },
    );

    println!(
        "    🚗 Driver: {} - Efficiency: {} km/energy",
        hybrid_car.name(),
        hybrid_car.fuel_efficiency()
    );
    println!("    🚢 Swimmer: {} - Wind Powered!", yacht.name());
    println!(
        "    ✈️  Flyer: {} - Long Range Specialist",
        cargo_plane.name()
    );

    let mut team2 = VehicleRaceTeam::new(
        "Efficiency Experts".to_string(),
        hybrid_car,
        yacht,
        cargo_plane,
    );

    // Team 3: Emergency Response (without truck)
    println!("\n🚨 TEAM 3: 'Emergency Response'");
    println!("  Strategy: Rapid response and rescue capabilities");

    let emergency_car = Car::new(
        "Emergency Responder".to_string(),
        "Emergency Motors".to_string(),
        2023,
        car::EngineType::Gasoline {
            cylinders: 6,
            displacement: 3.5,
        },
    );

    let coast_guard = Ship::new(
        "Sea Rescue".to_string(),
        "Coast Guard".to_string(),
        2023,
        ship::ShipType::Ferry,
        100,
        ship::PropulsionType::Diesel {
            engines: 3,
            power_each: 600,
        },
    );

    let rescue_helicopter = Helicopter::new(
        "Sky Medic".to_string(),
        "Rescue Aviation".to_string(),
        2023,
        helicopter::HelicopterType::Emergency,
        15,
        helicopter::HelicopterEngine::Turboshaft {
            engines: 2,
            power_each: 1000,
        },
    );

    println!(
        "    🚗 Driver: {} - Emergency Response Vehicle",
        emergency_car.name()
    );
    println!(
        "    🚢 Swimmer: {} - Coast Guard Vessel",
        coast_guard.name()
    );
    println!(
        "    🚁 Flyer: {} - Medical Helicopter",
        rescue_helicopter.name()
    );

    let mut team3 = VehicleRaceTeam::new(
        "Emergency Response".to_string(),
        emergency_car,
        coast_guard,
        rescue_helicopter,
    );

    // Team 4: All-Electric
    println!("\n🔋 TEAM 4: 'All-Electric Future'");
    println!("  Strategy: Clean energy across all domains");

    let electric_car = Car::new(
        "Tesla Racer".to_string(),
        "Electric Motors".to_string(),
        2023,
        car::EngineType::Electric {
            battery_capacity: 100,
        },
    );

    let electric_boat = Ship::new(
        "Silent Runner".to_string(),
        "Electric Marine".to_string(),
        2023,
        ship::ShipType::Yacht,
        20,
        ship::PropulsionType::Electric {
            motors: 4,
            power_each: 200,
        },
    );

    let electric_plane = Airplane::new(
        "Zero Emission".to_string(),
        "Green Aviation".to_string(),
        2023,
        airplane::AirplaneType::Private,
        18,
        airplane::AirplaneEngine::Electric {
            motors: 6,
            battery_capacity: 200,
        },
    );

    println!("    🚗 Driver: {} - Pure Electric", electric_car.name());
    println!("    🚢 Swimmer: {} - Electric Marine", electric_boat.name());
    println!(
        "    ✈️  Flyer: {} - Electric Aviation",
        electric_plane.name()
    );

    let mut team4 = VehicleRaceTeam::new(
        "All-Electric Future".to_string(),
        electric_car,
        electric_boat,
        electric_plane,
    );

    println!("\n🏁 RACE START!");
    println!("===============");

    // Run all team races
    println!("\n📊 RACE 1: Speed Demons");
    println!("{}", "-".repeat(30));
    let result1 = team1.race();

    println!("\n📊 RACE 2: Efficiency Experts");
    println!("{}", "-".repeat(30));
    let result2 = team2.race();

    println!("\n📊 RACE 3: Emergency Response");
    println!("{}", "-".repeat(30));
    let result3 = team3.race();

    println!("\n📊 RACE 4: All-Electric Future");
    println!("{}", "-".repeat(30));
    let result4 = team4.race();

    // Add all results to competition
    competition.add_team_result(result1);
    competition.add_team_result(result2);
    competition.add_team_result(result3);
    competition.add_team_result(result4);

    // Display comprehensive results
    competition.display_results();

    analyze_team_results(&competition);
}

fn analyze_individual_results(triathlon: &VehicleTriathlon) {
    println!("\n📊 INDIVIDUAL TRIATHLON ANALYSIS:");
    println!("=================================");

    let rankings = triathlon.get_rankings();

    if rankings.is_empty() {
        println!("\n🎯 NO PARTICIPANTS ANALYSIS:");
        println!("   - Zero vehicles qualified for individual triathlon");
        println!("   - This demonstrates the rarity of true multi-capability vehicles");
        println!("   - Engineering trade-offs make universal vehicles challenging");
        println!("   - Real-world vehicles typically specialize in 1-2 domains");
    } else {
        for result in &rankings {
            println!("\n🏎️ {} Analysis:", result.participant_name);
            println!("   Vehicle Versatility Score:");
            println!("     Max Speed: {} km/h", result.max_speed);
            println!("     Max Depth: {} m", result.max_depth);
            println!("     Max Altitude: {} m", result.max_altitude);

            let total_capability = result.max_speed + result.max_depth + result.max_altitude;
            println!("     Total Capability Points: {}", total_capability);

            if result.completed_stages == 3 {
                println!("   🏆 True Multi-Domain Champion!");
                println!("     Successfully demonstrated land, sea, and air capability");
            } else {
                println!("   ⚠️  Limited multi-domain capability");
            }
        }
    }

    println!("\n🎯 KEY INSIGHTS:");
    println!("   - Very few vehicles can compete in individual triathlon");
    println!("   - Multi-capability vehicles sacrifice specialization for versatility");
    println!("   - Real-world applications: Emergency response, military operations");
    println!("   - Design challenge: Balancing multiple conflicting requirements");
    println!("   - Future concepts: Flying cars, transforming vehicles");
}

fn analyze_team_results(competition: &VehicleRaceCompetition) {
    println!("\n📈 TEAM RACE STRATEGIC ANALYSIS:");
    println!("================================");

    let rankings = competition.get_rankings();

    for (i, result) in rankings.iter().enumerate() {
        println!("\n{}. Team: {}", i + 1, result.team_name);
        println!("   Strategy Analysis:");

        match result.team_name.as_str() {
            "Speed Demons" => {
                println!("     💨 Focus: Raw performance and speed");
                println!("     🎯 Strength: Maximum capability in each domain");
                println!("     ⚠️  Risk: High energy consumption");
            }
            "Efficiency Experts" => {
                println!("     ⚡ Focus: Sustainability and endurance");
                println!("     🎯 Strength: Energy conservation");
                println!("     ⚠️  Risk: Lower peak performance");
            }
            "Emergency Response" => {
                println!("     🚨 Focus: Reliability and rescue capability");
                println!("     🎯 Strength: Real-world utility");
                println!("     ⚠️  Risk: Optimized for different goals");
            }
            "All-Electric Future" => {
                println!("     🔋 Focus: Clean technology");
                println!("     🎯 Strength: Environmental friendliness");
                println!("     ⚠️  Risk: Current technology limitations");
            }
            _ => {}
        }

        // Performance breakdown
        println!("   Performance Breakdown:");
        println!(
            "     Land: {} (Max: {})",
            if result.land_leg.result.is_ok() {
                "✅"
            } else {
                "❌"
            },
            result.land_leg.max_capability
        );
        println!(
            "     Water: {} (Max: {})",
            if result.water_leg.result.is_ok() {
                "✅"
            } else {
                "❌"
            },
            result.water_leg.max_capability
        );
        println!(
            "     Air: {} (Max: {})",
            if result.air_leg.result.is_ok() {
                "✅"
            } else {
                "❌"
            },
            result.air_leg.max_capability
        );

        let total_capability = result.land_leg.max_capability
            + result.water_leg.max_capability
            + result.air_leg.max_capability;
        println!("     Combined Capability: {}", total_capability);
    }
}

fn demonstrate_race_strategies() {
    println!("🎯 VEHICLE RACE STRATEGIES");
    println!("==========================");

    println!("\n1. 🏎️ INDIVIDUAL TRIATHLON STRATEGY:");
    println!("   Requirements: Vehicle must implement Driving + Swimming + Flying");
    println!("   ```rust");
    println!("   fn add_participant<T>(participant: &mut T)");
    println!("   where T: Driving + Swimming + Flying + Vehicle + HasEnergy");
    println!("   ```");
    println!("   Advantages:");
    println!("     ✅ Single vehicle handles all challenges");
    println!("     ✅ No coordination between team members");
    println!("     ✅ Tests ultimate versatility");
    println!("   Disadvantages:");
    println!("     ❌ Very few vehicles qualify");
    println!("     ❌ Jack-of-all-trades, master of none");
    println!("     ❌ Complex engineering challenges");

    println!("\n2. 🏁 TEAM RELAY STRATEGY:");
    println!("   Requirements: Three specialists with different capabilities");
    println!("   ```rust");
    println!("   struct VehicleRaceTeam<D, S, F>");
    println!("   where");
    println!("       D: Driving + Vehicle + HasEnergy,");
    println!("       S: Swimming + Vehicle + HasEnergy,");
    println!("       F: Flying + Vehicle + HasEnergy,");
    println!("   ```");
    println!("   Advantages:");
    println!("     ✅ Each vehicle optimized for their domain");
    println!("     ✅ Many possible team combinations");
    println!("     ✅ Allows specialization");
    println!("     ✅ Real-world team dynamics");
    println!("   Disadvantages:");
    println!("     ❌ Coordination complexity");
    println!("     ❌ Team only as strong as weakest member");
    println!("     ❌ Handoff logistics");

    println!("\n3. 🎮 STRATEGIC DECISIONS:");
    println!("   Speed vs Efficiency:");
    println!("     🏎️  High performance = High energy consumption");
    println!("     ⚡ High efficiency = Lower peak performance");
    println!("     🎯 Balance depends on race format and length");

    println!("\n   Specialization vs Versatility:");
    println!("     🔧 Specialists: Maximum performance in one domain");
    println!("     🌟 Generalists: Flexibility across domains");
    println!("     🎯 Choice depends on competition format");

    println!("\n   Technology Choices:");
    println!("     ⛽ Traditional: Proven, high performance");
    println!("     🔋 Electric: Efficient, environmentally friendly");
    println!("     🔄 Hybrid: Best of both worlds, complexity cost");
}

fn final_race_analysis() {
    println!("🏆 FINAL RACE ANALYSIS");
    println!("======================");

    println!("\n🎯 TRAIT COMPOSITION SUCCESS FACTORS:");
    println!("=====================================");

    println!("\n1. 🧩 FLEXIBILITY:");
    println!("   ✅ Same competition framework works for both individual and team races");
    println!("   ✅ Easy to add new vehicle types with different capabilities");
    println!("   ✅ Mix and match capabilities as needed");
    println!("   ✅ Extensible scoring and analysis systems");

    println!("\n2. 🛡️ TYPE SAFETY:");
    println!("   ✅ Compiler prevents invalid team compositions");
    println!("   ✅ Can't put a car in the flying leg");
    println!("   ✅ Clear capability requirements for each competition");
    println!("   ✅ Runtime errors eliminated by compile-time checking");

    println!("\n3. 🚀 PERFORMANCE:");
    println!("   ✅ Zero-cost abstractions - trait calls optimized away");
    println!("   ✅ Static dispatch for maximum speed");
    println!("   ✅ Generic functions work with any valid combination");
    println!("   ✅ No runtime penalty for trait composition");

    println!("\n4. 🔧 MAINTAINABILITY:");
    println!("   ✅ Adding new competition types requires minimal changes");
    println!("   ✅ Vehicle implementations independent of competitions");
    println!("   ✅ Clear separation of concerns");
    println!("   ✅ Easy to test individual components");

    println!("\n🌍 REAL-WORLD APPLICATIONS:");
    println!("===========================");
    println!("   🚗 Automotive Industry: Multi-capability vehicle development");
    println!("   🏁 Racing Competitions: Different race formats and categories");
    println!("   🚨 Emergency Services: Multi-domain response capabilities");
    println!("   🎖️  Military Operations: Combined arms coordination");
    println!("   🎮 Game Development: Vehicle systems with varied capabilities");
    println!("   🏗️  Logistics: Multi-modal transportation optimization");

    println!("\n🎓 DESIGN LESSONS:");
    println!("==================");
    println!("   1. Traits enable both specialization AND generalization");
    println!("   2. Composition beats inheritance for flexibility");
    println!("   3. Type system prevents logical errors before runtime");
    println!("   4. Generic programming enables code reuse across domains");
    println!("   5. Same patterns scale from simple to complex systems");

    println!("\n💡 ENGINEERING INSIGHTS:");
    println!("========================");
    println!("   Individual Triathlon Challenges:");
    println!("     - Engineering a vehicle for all domains is extremely difficult");
    println!("     - Trade-offs between competing requirements");
    println!("     - Weight, power, complexity all increase");
    println!("     - Real examples: Flying cars still in development");

    println!("\n   Team Relay Advantages:");
    println!("     - Specialization allows optimization for specific tasks");
    println!("     - Proven approach in real-world operations");
    println!("     - Each vehicle can be best-in-class for its domain");
    println!("     - Coordination challenges but better overall performance");

    println!("\n🎉 VEHICLE RACE CHAMPIONSHIP COMPLETE! 🎉");
    println!("Trait composition enables unlimited racing possibilities!");
    println!("The type system guides us toward realistic and effective designs!");
}

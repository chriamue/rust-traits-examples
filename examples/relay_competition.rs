// examples/relay_competition.rs
//! Relay Competition Example
//!
//! This example demonstrates how trait composition enables team competitions
//! where each member contributes their specialized capabilities.

use rust_traits_examples::{
    animals::*,
    behaviors::*,
    competitions::relay::*,
    core::{EnergyLevel, HasEnergy},
};

fn main() {
    println!("🏃‍♂️ANIMAL RELAY CHAMPIONSHIP 🏃‍♂️");
    println!("===============================");
    println!();

    println!("📋 COMPETITION FORMAT:");
    println!("  - Teams of 3 specialists: Swimmer → Walker → Flyer");
    println!("  - Each member performs their specialty");
    println!("  - Handoff between legs (no rest between team members)");
    println!("  - Team score based on all members' performance");
    println!();

    let mut competition = RelayCompetition::new();

    // Team 1: The Specialists
    println!("🔵 TEAM 1: 'The Specialists'");
    println!("  Strategy: Use the best animal for each discipline");

    let whale = Whale::new("Deep Dive Danny".to_string());
    let greyhound = Dog::new("Lightning Bolt".to_string(), dog::DogBreed::Greyhound);
    let eagle = Eagle::new("Sky Dominator".to_string());

    println!(
        "  🏊 Swimmer: {} ({}) - Max depth: {}m",
        whale.name(),
        whale.species(),
        whale.max_depth()
    );
    println!(
        "  🚶 Walker: {} ({}) - Built for speed",
        greyhound.name(),
        greyhound.species()
    );
    println!(
        "  🛩️  Flyer: {} ({}) - Max altitude: {}m",
        eagle.name(),
        eagle.species(),
        eagle.max_altitude()
    );

    let mut team_specialists =
        RelayTeam::new("The Specialists".to_string(), whale, greyhound, eagle);

    // Team 2: The Penguin Squad
    println!("\n🟡 TEAM 2: 'Penguin Squad'");
    println!("  Strategy: Aquatic expertise with diverse support");

    let penguin = Penguin::new("Torpedo Tux".to_string());
    let collie = Dog::new("Steady Stride".to_string(), dog::DogBreed::BorderCollie);
    let falcon = Eagle::new("Wind Cutter".to_string());

    println!(
        "  🏊 Swimmer: {} ({}) - Max depth: {}m",
        penguin.name(),
        penguin.species(),
        penguin.max_depth()
    );
    println!(
        "  🚶 Walker: {} ({}) - Endurance specialist",
        collie.name(),
        collie.species()
    );
    println!(
        "  🛩️  Flyer: {} ({}) - Max altitude: {}m",
        falcon.name(),
        falcon.species(),
        falcon.max_altitude()
    );

    let mut team_penguin = RelayTeam::new("Penguin Squad".to_string(), penguin, collie, falcon);

    // Team 3: The All-Rounders (all ducks)
    println!("\n🟢 TEAM 3: 'The All-Rounders'");
    println!("  Strategy: Consistent performance across all disciplines");

    let mut duck_swimmer = Duck::new("Splash Master".to_string());
    duck_swimmer.set_energy(EnergyLevel::Energetic); // Boost for swimming

    let mut duck_walker = Duck::new("Waddle Champion".to_string());
    duck_walker.set_energy(EnergyLevel::Normal);

    let mut duck_flyer = Duck::new("Sky Sailor".to_string());
    duck_flyer.set_energy(EnergyLevel::Hyperactive); // Boost for flying

    println!(
        "  🏊 Swimmer: {} ({}) - Energy: {}",
        duck_swimmer.name(),
        duck_swimmer.species(),
        duck_swimmer.energy()
    );
    println!(
        "  🚶 Walker: {} ({}) - Energy: {}",
        duck_walker.name(),
        duck_walker.species(),
        duck_walker.energy()
    );
    println!(
        "  🛩️  Flyer: {} ({}) - Energy: {}",
        duck_flyer.name(),
        duck_flyer.species(),
        duck_flyer.energy()
    );

    let mut team_all_rounders = RelayTeam::new(
        "The All-Rounders".to_string(),
        duck_swimmer,
        duck_walker,
        duck_flyer,
    );

    // Team 4: The Underdogs
    println!("\n🟣 TEAM 4: 'The Underdogs'");
    println!("  Strategy: Prove that teamwork beats individual excellence");

    let labrador = Dog::new("Paddle Paws".to_string(), dog::DogBreed::Labrador);
    let mut rescue_dog = Dog::new("Mountain Rescue".to_string(), dog::DogBreed::SaintBernard);
    rescue_dog.set_energy(EnergyLevel::Tired); // Handicap for challenge

    let mut young_eagle = Eagle::new("Rookie Flyer".to_string());
    young_eagle.set_energy(EnergyLevel::Normal); // Moderate energy

    println!(
        "  🏊 Swimmer: {} ({}) - Water dog advantage",
        labrador.name(),
        labrador.species()
    );
    println!(
        "  🚶 Walker: {} ({}) - Energy: {} (challenging!)",
        rescue_dog.name(),
        rescue_dog.species(),
        rescue_dog.energy()
    );
    println!(
        "  🛩️  Flyer: {} ({}) - Young but determined",
        young_eagle.name(),
        young_eagle.species()
    );

    let mut team_underdogs = RelayTeam::new(
        "The Underdogs".to_string(),
        labrador,
        rescue_dog,
        young_eagle,
    );

    println!("\n🚫 INVALID TEAM COMBINATIONS:");
    println!("  The following would NOT compile due to trait constraints:");
    println!("  ```rust");
    println!("  // ❌ Cannot use Whale as walker (no Walking trait)");
    println!("  let bad_team = RelayTeam::new(\"Bad\".to_string(), whale, whale, eagle);");
    println!("  ");
    println!("  // ❌ Cannot use Dog as flyer (no Flying trait)");
    println!("  let bad_team = RelayTeam::new(\"Bad\".to_string(), whale, dog, dog);");
    println!("  ```");

    println!("\n🏁 RACE START!");
    println!("===============");

    // Run all team races
    println!("\n📊 RACE 1: The Specialists");
    println!("{}", "-".repeat(30));
    let result1 = team_specialists.race();

    println!("\n📊 RACE 2: Penguin Squad");
    println!("{}", "-".repeat(30));
    let result2 = team_penguin.race();

    println!("\n📊 RACE 3: The All-Rounders");
    println!("{}", "-".repeat(30));
    let result3 = team_all_rounders.race();

    println!("\n📊 RACE 4: The Underdogs");
    println!("{}", "-".repeat(30));
    let result4 = team_underdogs.race();

    // Add all results to competition
    competition.add_team_result(result1);
    competition.add_team_result(result2);
    competition.add_team_result(result3);
    competition.add_team_result(result4);

    // Display comprehensive results
    competition.display_results();

    // Strategic analysis
    println!("\n📈 STRATEGIC ANALYSIS:");
    println!("======================");

    let rankings = competition.get_rankings();

    for (i, result) in rankings.iter().enumerate() {
        println!("\n{}. Team: {}", i + 1, result.team_name);
        println!("   Completed legs: {}/3", result.completed_legs);
        println!("   Total score: {} points", result.score());

        // Analyze team composition strategy
        match result.team_name.as_str() {
            "The Specialists" => {
                println!("   💡 Strategy: Specialists for each discipline");
                println!("   🎯 Advantage: Each member performs their strength");
                if result.is_complete() {
                    println!("   ✅ Specialization strategy succeeded!");
                } else {
                    println!("   ⚠️  Specialists couldn't overcome energy constraints");
                }
            }
            "Penguin Squad" => {
                println!("   💡 Strategy: Aquatic expertise with balanced team");
                println!("   🎯 Advantage: Strong swimming with solid support");
            }
            "The All-Rounders" => {
                println!("   💡 Strategy: Consistent generalists (all ducks)");
                println!("   🎯 Advantage: Predictable performance, no weak links");
                if result.completed_legs == 3 {
                    println!("   ✅ Consistency paid off!");
                }
            }
            "The Underdogs" => {
                println!("   💡 Strategy: Overcome individual limitations with teamwork");
                println!("   🎯 Advantage: Determination and heart");
                if result.completed_legs >= 2 {
                    println!("   🏆 Exceeded expectations!");
                }
            }
            _ => {}
        }

        // Leg-by-leg analysis
        println!("   📊 Performance breakdown:");
        println!(
            "     Swimming: {}",
            if result.swimmer.result.is_ok() {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "     Walking:  {}",
            if result.walker.result.is_ok() {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "     Flying:   {}",
            if result.flyer.result.is_ok() {
                "✅"
            } else {
                "❌"
            }
        );
    }

    println!("\n🎯 RELAY COMPETITION INSIGHTS:");
    println!("==============================");

    println!("\n1. 🧩 Team Composition Strategies:");
    println!("   ✅ Specialists: Maximum performance in each discipline");
    println!("   ✅ Generalists: Consistent, predictable performance");
    println!("   ✅ Mixed: Balance specialization with versatility");

    println!("\n2. 🔗 Trait Composition Benefits:");
    println!("   ✅ Type-safe team assembly");
    println!("   ✅ Compile-time constraint enforcement");
    println!("   ✅ Each member contributes their strength");
    println!("   ✅ No invalid assignments possible");

    println!("\n3. ⚡ Performance Insights:");
    println!("   ✅ Energy management affects team success");
    println!("   ✅ Starting energy levels matter for each leg");
    println!("   ✅ Handoffs don't provide rest (realistic simulation)");

    println!("\n4. 🛠️  Design Flexibility:");
    println!("   ✅ Easy to create new team combinations");
    println!("   ✅ Generic RelayTeam<S, W, F> accepts any valid trio");
    println!("   ✅ Extensible scoring and analysis systems");

    if let Some(winner) = competition.get_winner() {
        println!("\n🏆 WINNING STRATEGY ANALYSIS:");
        println!("=============================");
        println!("Champions: Team {}", winner.team_name);
        println!("Final score: {} points", winner.score());
        println!();
        println!("Success factors:");

        if winner.completed_legs == 3 {
            println!("  ✅ Completed all relay legs");
        }

        let total_final_energy = winner.swimmer.final_energy as u32
            + winner.walker.final_energy as u32
            + winner.flyer.final_energy as u32;

        if total_final_energy >= 6 {
            // Average of Tired (2) across all members
            println!("  ✅ Team maintained good energy levels");
        }

        if winner.total_time_penalty < 150 {
            println!("  ✅ Excellent time management");
        } else if winner.total_time_penalty < 300 {
            println!("  ✅ Good time management");
        }

        println!("\nThis demonstrates how trait composition enables:");
        println!("  🎯 Flexible team strategies");
        println!("  🎯 Type-safe specialization");
        println!("  🎯 Compile-time constraint enforcement");
        println!("  🎯 Optimal resource allocation");
    }
}

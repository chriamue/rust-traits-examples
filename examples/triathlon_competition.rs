// examples/triathlon_competition.rs
//! Triathlon Competition Example
//!
//! This example demonstrates how trait composition enables competitions
//! that require multiple capabilities from a single participant.

use rust_traits_examples::{
    animals::*,
    competitions::triathlon::*,
    core::{EnergyLevel, HasEnergy},
};

fn main() {
    println!("🏊‍♂️🚶‍♂️🛩️ ANIMAL TRIATHLON CHAMPIONSHIP 🛩️🚶‍♂️🏊‍♂️");
    println!("================================================");
    println!();

    // Create a triathlon competition
    let mut triathlon = Triathlon::new();

    println!("📋 COMPETITION RULES:");
    println!("  - Participants must be able to WALK, SWIM, and FLY");
    println!("  - Three stages: Walking → Swimming → Flying");
    println!("  - Energy management is crucial for success");
    println!("  - Scoring based on completion and remaining energy");
    println!();

    // Only ducks can participate (they're the only animals with all three capabilities)
    println!("🦆 CREATING PARTICIPANTS:");

    let mut speedster = Duck::new("Speedster McGillicuddy".to_string());
    speedster.set_energy(EnergyLevel::Normal);
    println!(
        "  ✅ {} - Energy: {} (Balanced approach)",
        speedster.name(),
        speedster.energy()
    );

    let mut iron_duck = Duck::new("Iron Duck".to_string());
    iron_duck.set_energy(EnergyLevel::Hyperactive);
    println!(
        "  ✅ {} - Energy: {} (High energy strategy)",
        iron_duck.name(),
        iron_duck.energy()
    );

    let mut endurance_expert = Duck::new("Endurance Expert".to_string());
    endurance_expert.set_energy(EnergyLevel::Energetic);
    println!(
        "  ✅ {} - Energy: {} (Steady performer)",
        endurance_expert.name(),
        endurance_expert.energy()
    );

    let mut underdog = Duck::new("The Underdog".to_string());
    underdog.set_energy(EnergyLevel::Tired);
    println!(
        "  ✅ {} - Energy: {} (Challenging start)",
        underdog.name(),
        underdog.energy()
    );

    println!();
    println!("🚫 ANIMALS THAT CANNOT PARTICIPATE:");
    println!("  ❌ Dogs - Can walk and swim, but cannot fly");
    println!("  ❌ Eagles - Can walk and fly, but cannot swim");
    println!("  ❌ Whales - Can only swim");
    println!("  ❌ Penguins - Can walk and swim, but cannot fly");
    println!();

    // Demonstrate type safety
    println!("🔒 TYPE SAFETY DEMONSTRATION:");
    println!("   The following would NOT compile:");
    println!("   ```rust");
    println!("   let mut dog = Dog::new(\"Rex\".to_string(), \"Labrador\".to_string());");
    println!("   triathlon.add_participant(&mut dog); // ❌ COMPILE ERROR!");
    println!("   // Error: Dog doesn't implement Flying trait");
    println!("   ```");
    println!();

    println!("🏁 STARTING TRIATHLON:");
    println!("=====================");

    // Run the competition
    triathlon.add_participant(&mut speedster);
    println!();

    triathlon.add_participant(&mut iron_duck);
    println!();

    triathlon.add_participant(&mut endurance_expert);
    println!();

    triathlon.add_participant(&mut underdog);

    // Display comprehensive results
    triathlon.display_results();

    // Analyze the results
    println!("\n📊 PERFORMANCE ANALYSIS:");
    println!("========================");

    let rankings = triathlon.get_rankings();

    for (i, result) in rankings.iter().enumerate() {
        println!(
            "\n{}. {} ({} → {}):",
            i + 1,
            result.participant_name,
            result.starting_energy,
            result.final_energy
        );

        println!("   Stages completed: {}/3", result.completed_stages);
        println!("   Final score: {} points", result.score());

        if result.completed_stages == 3 {
            println!("   🎉 Completed full triathlon!");
        } else {
            println!("   ⚠️  Exhausted after {} stages", result.completed_stages);
        }

        // Energy analysis
        let energy_drop = result.starting_energy as i32 - result.final_energy as i32;
        println!("   Energy consumed: {} levels", energy_drop);

        match energy_drop {
            0..=2 => println!("   💪 Excellent energy management"),
            3..=4 => println!("   👍 Good energy management"),
            5..=6 => println!("   ⚠️  High energy consumption"),
            _ => println!("   😰 Severe energy depletion"),
        }
    }

    println!("\n🎯 KEY INSIGHTS:");
    println!("================");
    println!("1. 🏊‍♂️ Trait Composition Advantage:");
    println!("   - Only animals with ALL required traits can compete");
    println!("   - Compile-time safety prevents invalid participants");
    println!("   - No runtime errors from unsupported operations");

    println!("\n2. ⚡ Energy Management Strategy:");
    println!("   - Starting energy level affects performance");
    println!("   - Each activity consumes energy progressively");
    println!("   - Strategic pacing can improve outcomes");

    println!("\n3. 🔄 Flexible Design:");
    println!("   - Easy to add new animals with different capabilities");
    println!("   - Competition rules enforced by type system");
    println!("   - Extensible scoring and ranking system");

    if let Some(winner) = triathlon.get_winner() {
        println!("\n🏆 CHAMPION STRATEGY ANALYSIS:");
        println!("==============================");
        println!(
            "Winner: {} with {} points",
            winner.participant_name,
            winner.score()
        );
        println!("Starting energy: {}", winner.starting_energy);
        println!("Final energy: {}", winner.final_energy);
        println!("Success factors:");

        if winner.starting_energy >= EnergyLevel::Energetic {
            println!("  ✅ Started with high energy reserves");
        }
        if winner.completed_stages == 3 {
            println!("  ✅ Completed all three stages");
        }
        if winner.final_energy >= EnergyLevel::Tired {
            println!("  ✅ Maintained reasonable energy levels");
        }
    }
}

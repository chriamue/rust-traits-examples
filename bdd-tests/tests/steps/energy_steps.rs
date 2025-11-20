use crate::TraitsWorld;
use cucumber::{given, then, when};
use rust_traits_examples::behaviors::*;
use rust_traits_examples::core::*;

#[given(expr = "the dog has energy level {string}")]
async fn dog_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut dog) = world.dog {
        let energy_level = match level.as_str() {
            "Collapsed" => EnergyLevel::Collapsed,
            "Exhausted" => EnergyLevel::Exhausted,
            "Tired" => EnergyLevel::Tired,
            "Normal" => EnergyLevel::Normal,
            "Energetic" => EnergyLevel::Energetic,
            "Hyperactive" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };
        dog.set_energy(energy_level);
        world.energy_before = Some(energy_level);
    }
}

#[given(expr = "the duck has energy level {string}")]
async fn duck_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut duck) = world.duck {
        let energy_level = match level.as_str() {
            "Collapsed" => EnergyLevel::Collapsed,
            "Exhausted" => EnergyLevel::Exhausted,
            "Tired" => EnergyLevel::Tired,
            "Normal" => EnergyLevel::Normal,
            "Energetic" => EnergyLevel::Energetic,
            "Hyperactive" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };
        duck.set_energy(energy_level);
        world.energy_before = Some(energy_level);
    }
}

#[given(expr = "the eagle has energy level {string}")]
async fn eagle_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut eagle) = world.eagle {
        let energy_level = match level.as_str() {
            "Collapsed" => EnergyLevel::Collapsed,
            "Exhausted" => EnergyLevel::Exhausted,
            "Tired" => EnergyLevel::Tired,
            "Normal" => EnergyLevel::Normal,
            "Energetic" => EnergyLevel::Energetic,
            "Hyperactive" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };
        eagle.set_energy(energy_level);
        world.energy_before = Some(energy_level);
    }
}

#[when(expr = "the dog walks")]
async fn dog_walks(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.last_result = Some(dog.walk().map_err(|e| e.to_string()));
        world.energy_after = Some(dog.energy());
    }
}

#[when(expr = "the dog runs")]
async fn dog_runs(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.last_result = Some(dog.run().map_err(|e| e.to_string()));
        world.energy_after = Some(dog.energy());
    }
}

#[when(expr = "the duck swims")]
async fn duck_swims(world: &mut TraitsWorld) {
    if let Some(ref mut duck) = world.duck {
        world.last_result = Some(duck.swim().map_err(|e| e.to_string()));
        world.energy_after = Some(duck.energy());
    }
}

#[when(expr = "the eagle flies")]
async fn eagle_flies(world: &mut TraitsWorld) {
    if let Some(ref mut eagle) = world.eagle {
        world.last_result = Some(eagle.fly().map_err(|e| e.to_string()));
        world.energy_after = Some(eagle.energy());
    }
}

#[when(expr = "the dog attempts to walk")]
async fn dog_attempts_walk(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.last_result = Some(dog.walk().map_err(|e| e.to_string()));
    }
}

#[when(expr = "the dog attempts to run")]
async fn dog_attempts_run(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.last_result = Some(dog.run().map_err(|e| e.to_string()));
    }
}

#[when(expr = "the duck rests")]
async fn duck_rests(world: &mut TraitsWorld) {
    if let Some(ref mut duck) = world.duck {
        world.energy_before = Some(duck.energy());
        duck.rest();
        world.energy_after = Some(duck.energy());
        world.last_result = Some(Ok("Rested".to_string()));
    }
}

#[then(expr = "the dog's energy should decrease")]
async fn energy_should_decrease(world: &mut TraitsWorld) {
    assert!(
        world.energy_after.unwrap() < world.energy_before.unwrap(),
        "Energy should decrease after activity"
    );
}

#[then(expr = "the dog's energy should decrease by {int} levels")]
async fn energy_should_decrease_by(world: &mut TraitsWorld, levels: i32) {
    let before = world.energy_before.unwrap() as i32;
    let after = world.energy_after.unwrap() as i32;
    assert_eq!(
        before - after,
        levels,
        "Energy should decrease by {} levels",
        levels
    );
}

#[then(expr = "the dog should have energy level {string}")]
async fn dog_should_have_energy(world: &mut TraitsWorld, level: String) {
    let expected = match level.as_str() {
        "Collapsed" => EnergyLevel::Collapsed,
        "Exhausted" => EnergyLevel::Exhausted,
        "Tired" => EnergyLevel::Tired,
        "Normal" => EnergyLevel::Normal,
        "Energetic" => EnergyLevel::Energetic,
        "Hyperactive" => EnergyLevel::Hyperactive,
        _ => EnergyLevel::Normal,
    };

    assert_eq!(
        world.dog.as_ref().unwrap().energy(),
        expected,
        "Dog should have {} energy",
        level
    );
}

#[then(expr = "the duck should have energy level {string}")]
async fn duck_should_have_energy(world: &mut TraitsWorld, level: String) {
    let expected = match level.as_str() {
        "Collapsed" => EnergyLevel::Collapsed,
        "Exhausted" => EnergyLevel::Exhausted,
        "Tired" => EnergyLevel::Tired,
        "Normal" => EnergyLevel::Normal,
        "Energetic" => EnergyLevel::Energetic,
        "Hyperactive" => EnergyLevel::Hyperactive,
        _ => EnergyLevel::Normal,
    };

    assert_eq!(
        world.duck.as_ref().unwrap().energy(),
        expected,
        "Duck should have {} energy",
        level
    );
}

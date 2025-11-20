use crate::TraitsWorld;
use cucumber::{given, then, when};
use rust_traits_examples::behaviors::*;
use rust_traits_examples::core::*;

// Helper function to parse energy levels
fn parse_energy_level(level: &str) -> EnergyLevel {
    match level {
        "Collapsed" => EnergyLevel::Collapsed,
        "Exhausted" => EnergyLevel::Exhausted,
        "Tired" => EnergyLevel::Tired,
        "Normal" => EnergyLevel::Normal,
        "Energetic" => EnergyLevel::Energetic,
        "Hyperactive" => EnergyLevel::Hyperactive,
        _ => EnergyLevel::Normal,
    }
}

// Dog energy steps
#[given(expr = "the dog has energy level {string}")]
async fn dog_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut dog) = world.dog {
        let energy_level = parse_energy_level(&level);
        dog.set_energy(energy_level);
        world.energy_before = Some(energy_level);
        println!("✅ Set dog energy to: {}", energy_level);
    }
}

// Duck energy steps
#[given(expr = "the duck has energy level {string}")]
async fn duck_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut duck) = world.duck {
        let energy_level = parse_energy_level(&level);
        duck.set_energy(energy_level);
        world.energy_before = Some(energy_level);
        println!("✅ Set duck energy to: {}", energy_level);
    }
}

// Eagle energy steps
#[given(expr = "the eagle has energy level {string}")]
async fn eagle_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut eagle) = world.eagle {
        let energy_level = parse_energy_level(&level);
        eagle.set_energy(energy_level);
        world.energy_before = Some(energy_level);
        println!("✅ Set eagle energy to: {}", energy_level);
    }
}

// Car energy steps
#[given(expr = "the car has energy level {string}")]
async fn car_has_energy_level(world: &mut TraitsWorld, level: String) {
    if let Some(ref mut car) = world.car {
        let energy_level = parse_energy_level(&level);
        car.set_energy(energy_level);
        world.energy_before = Some(energy_level);
        println!("✅ Set car energy to: {}", energy_level);
    }
}

// Dog movement steps
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

#[when(expr = "the dog attempts to walk")]
async fn dog_attempts_walk(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.energy_before = Some(dog.energy());
        world.last_result = Some(dog.walk().map_err(|e| e.to_string()));
        world.energy_after = Some(dog.energy());
    }
}

#[when(expr = "the dog attempts to run")]
async fn dog_attempts_run(world: &mut TraitsWorld) {
    if let Some(ref mut dog) = world.dog {
        world.energy_before = Some(dog.energy());
        world.last_result = Some(dog.run().map_err(|e| e.to_string()));
        world.energy_after = Some(dog.energy());
    }
}

#[when(expr = "the dog attempts to swim")]
async fn dog_attempts_swim(world: &mut TraitsWorld) {
    use rust_traits_examples::behaviors::Swimming;

    if let Some(ref mut dog) = world.dog {
        world.energy_before = Some(dog.energy());
        world.last_result = Some(dog.swim().map_err(|e| e.to_string()));
        world.energy_after = Some(dog.energy());

        match &world.last_result {
            Some(Ok(msg)) => println!("✅ Dog swim attempt: {}", msg),
            Some(Err(e)) => println!("❌ Dog swim attempt failed: {}", e),
            None => {}
        }
    } else {
        println!("❌ No dog available to attempt swimming");
    }
}

// Duck movement steps
#[when(expr = "the duck swims")]
async fn duck_swims(world: &mut TraitsWorld) {
    if let Some(ref mut duck) = world.duck {
        world.last_result = Some(duck.swim().map_err(|e| e.to_string()));
        world.energy_after = Some(duck.energy());
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

// Eagle movement steps
#[when(expr = "the eagle flies")]
async fn eagle_flies(world: &mut TraitsWorld) {
    if let Some(ref mut eagle) = world.eagle {
        world.last_result = Some(eagle.fly().map_err(|e| e.to_string()));
        world.energy_after = Some(eagle.energy());
    }
}

// Energy verification steps
#[then(expr = "the dog's energy should decrease")]
async fn energy_should_decrease(world: &mut TraitsWorld) {
    assert!(
        world.energy_after.unwrap() < world.energy_before.unwrap(),
        "Energy should decrease after activity"
    );
}

#[then(expr = "the duck's energy should decrease")]
async fn duck_energy_should_decrease(world: &mut TraitsWorld) {
    assert!(
        world.energy_after.unwrap() < world.energy_before.unwrap(),
        "Duck's energy should decrease after activity"
    );
}

#[then(expr = "the eagle's energy should decrease")]
async fn eagle_energy_should_decrease(world: &mut TraitsWorld) {
    assert!(
        world.energy_after.unwrap() < world.energy_before.unwrap(),
        "Eagle's energy should decrease after activity"
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
    let expected = parse_energy_level(&level);
    assert_eq!(
        world.dog.as_ref().unwrap().energy(),
        expected,
        "Dog should have {} energy",
        level
    );
}

#[then(expr = "the duck should have energy level {string}")]
async fn duck_should_have_energy(world: &mut TraitsWorld, level: String) {
    let expected = parse_energy_level(&level);
    assert_eq!(
        world.duck.as_ref().unwrap().energy(),
        expected,
        "Duck should have {} energy",
        level
    );
}

#[then(expr = "the eagle should have energy level {string}")]
async fn eagle_should_have_energy(world: &mut TraitsWorld, level: String) {
    let expected = parse_energy_level(&level);
    assert_eq!(
        world.eagle.as_ref().unwrap().energy(),
        expected,
        "Eagle should have {} energy",
        level
    );
}

#[then(expr = "the action should fail with {string} error")]
async fn action_should_fail_with_error(world: &mut TraitsWorld, error_type: String) {
    assert!(world.last_result.is_some(), "No action was performed");

    let result = world.last_result.as_ref().unwrap();
    assert!(result.is_err(), "Action should have failed but succeeded");

    let error_msg = result.as_ref().unwrap_err();

    // Map error type names to patterns in the actual error messages
    let expected_pattern = match error_type.as_str() {
        "Collapsed" => "collapsed",
        "InsufficientEnergy" => "Insufficient energy",
        "InsufficientEnergyForLandMove" => "Insufficient energy for land movement",
        _ => error_type.as_str(),
    };

    assert!(
        error_msg
            .to_lowercase()
            .contains(&expected_pattern.to_lowercase()),
        "Expected error containing '{}' but got '{}'",
        expected_pattern,
        error_msg
    );
}

#[then(expr = "the duck's energy should increase")]
async fn duck_energy_should_increase(world: &mut TraitsWorld) {
    assert!(
        world.energy_after.unwrap() > world.energy_before.unwrap(),
        "Duck's energy should increase after resting"
    );
}

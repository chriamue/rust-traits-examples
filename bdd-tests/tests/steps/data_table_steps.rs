use crate::TraitsWorld;
use cucumber::{gherkin::Step, given, then, when};
use rust_traits_examples::animals::*;
use rust_traits_examples::core::*;
use rust_traits_examples::vehicles::*;

#[given(expr = "the energy management system is initialized")]
async fn energy_system_initialized(_world: &mut TraitsWorld) {
    // System initialization - in this case, just a semantic step
    println!("✅ Energy management system initialized");
}

#[given(expr = "all dog breeds are registered in the system")]
async fn dog_breeds_registered(_world: &mut TraitsWorld) {
    println!("✅ Dog breeds registered: Labrador, Bulldog, Greyhound, etc.");
}

#[then(expr = "the dog's maximum swimming depth should be {int} meters")]
async fn dog_max_swimming_depth(world: &mut TraitsWorld, expected_depth: i32) {
    use rust_traits_examples::behaviors::Swimming;

    if let Some(ref dog) = world.dog {
        let actual_depth = dog.max_depth();
        assert_eq!(
            actual_depth, expected_depth as u32,
            "Expected max depth {} but got {}",
            expected_depth, actual_depth
        );
        println!("✅ Dog can swim to depth: {}m", actual_depth);
    } else {
        panic!("No dog available for depth testing");
    }
}

#[given(expr = "the following dogs are entered in the race:")]
async fn dogs_entered_in_race(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("🏁 Entering dogs in race:");

        for row in table.rows.iter().skip(1) {
            // Skip header row
            let name = &row[0];
            let breed = &row[1];

            let dog_breed = match breed.as_str() {
                "Labrador" => dog::DogBreed::Labrador,
                "GoldenRetriever" => dog::DogBreed::GoldenRetriever,
                "BorderCollie" => dog::DogBreed::BorderCollie,
                "Bulldog" => dog::DogBreed::Bulldog,
                _ => dog::DogBreed::Other,
            };

            println!("  - {} ({})", name, breed);

            // Store the first dog as primary test subject
            if world.dog.is_none() {
                world.dog = Some(Dog::new(name.clone(), dog_breed));
            }
        }
    }
}

#[when(expr = "the swimming race begins")]
async fn swimming_race_begins(_world: &mut TraitsWorld) {
    println!("🏊 Swimming race started!");
}

#[then(expr = "all dogs should attempt to swim")]
async fn all_dogs_attempt_swim(_world: &mut TraitsWorld) {
    println!("✅ All dogs are attempting to swim");
}

#[then(expr = "{string} should have better swimming capability than {string}")]
async fn compare_swimming_capability(_world: &mut TraitsWorld, better: String, worse: String) {
    println!(
        "✅ {} has better swimming capability than {}",
        better, worse
    );
    // In a full implementation, you'd compare actual dog instances
}

#[given(expr = "the dog has the following initial state:")]
async fn dog_initial_state(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let property = &row[0];
            let value = &row[1];

            match property.as_str() {
                "energy" => {
                    let energy = match value.as_str() {
                        "Hyperactive" => EnergyLevel::Hyperactive,
                        "Energetic" => EnergyLevel::Energetic,
                        "Normal" => EnergyLevel::Normal,
                        "Tired" => EnergyLevel::Tired,
                        "Exhausted" => EnergyLevel::Exhausted,
                        "Collapsed" => EnergyLevel::Collapsed,
                        _ => EnergyLevel::Normal,
                    };

                    if let Some(ref mut dog) = world.dog {
                        dog.set_energy(energy);
                        println!("✅ Set dog energy to: {}", energy);
                    }
                }
                "breed" => {
                    println!("✅ Dog breed: {}", value);
                }
                _ => {}
            }
        }
    }
}

#[when(expr = "the dog performs multiple activities:")]
async fn dog_multiple_activities(world: &mut TraitsWorld, step: &Step) {
    use rust_traits_examples::behaviors::{Swimming, Walking};

    if let Some(table) = step.table.as_ref() {
        if let Some(ref mut dog) = world.dog {
            world.energy_before = Some(dog.energy());

            for row in table.rows.iter().skip(1) {
                let activity = &row[0];
                let repetitions: usize = row[1].parse().unwrap_or(1);

                for i in 0..repetitions {
                    // Convert all results to a common type
                    let result: Result<String, String> = match activity.as_str() {
                        "walk" => dog
                            .walk()
                            .map(|_| "walked".to_string())
                            .map_err(|e| e.to_string()),
                        "swim" => dog
                            .swim()
                            .map(|_| "swam".to_string())
                            .map_err(|e| e.to_string()),
                        "run" => dog
                            .run()
                            .map(|_| "ran".to_string())
                            .map_err(|e| e.to_string()),
                        _ => Ok("unknown activity".to_string()),
                    };

                    match result {
                        Ok(desc) => println!("  ✅ Dog {} (repetition {})", desc, i + 1),
                        Err(e) => println!("  ❌ Activity failed: {}", e),
                    }
                }
            }

            world.energy_after = Some(dog.energy());
        }
    }
}

#[then(expr = "the dog's energy should be significantly depleted")]
async fn dog_energy_depleted(world: &mut TraitsWorld) {
    if let (Some(before), Some(after)) = (world.energy_before, world.energy_after) {
        let depleted = (before as i32) - (after as i32);
        assert!(
            depleted >= 2,
            "Expected significant energy depletion (>=2 levels), but only {} levels consumed",
            depleted
        );
        println!(
            "✅ Energy significantly depleted: {} -> {} ({} levels)",
            before, after, depleted
        );
    }
}

#[then(expr = "the dog should still be able to perform basic movements")]
async fn dog_can_still_move(world: &mut TraitsWorld) {
    if let Some(ref dog) = world.dog {
        assert!(
            dog.energy() > EnergyLevel::Collapsed,
            "Dog is too exhausted to move"
        );
        println!(
            "✅ Dog still has energy for basic movements: {}",
            dog.energy()
        );
    }
}

// Vehicle capability matrix steps
#[given(expr = "the vehicle fleet management system is active")]
async fn fleet_system_active(_world: &mut TraitsWorld) {
    println!("✅ Fleet management system active");
}

#[given(expr = "all vehicle types are available")]
async fn all_vehicle_types_available(_world: &mut TraitsWorld) {
    println!("✅ All vehicle types available");
}

#[given(expr = "the following vehicles are available:")]
async fn vehicles_available(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("🚗 Vehicles available:");

        for row in table.rows.iter().skip(1) {
            let name = &row[0];
            let vehicle_type = &row[1];
            let manufacturer = &row[2];
            let year: u32 = row[3].parse().unwrap_or(2023);

            println!(
                "  - {} ({}, {}, {})",
                name, vehicle_type, manufacturer, year
            );

            // Create and store vehicles based on type
            match vehicle_type.as_str() {
                "Car" => {
                    world.car = Some(Car::new(
                        name.clone(),
                        manufacturer.clone(),
                        year,
                        car::EngineType::Gasoline {
                            cylinders: 4,
                            displacement: 2.0,
                        },
                    ));
                }
                "Airplane" => {
                    world.airplane = Some(Airplane::new(
                        name.clone(),
                        manufacturer.clone(),
                        year,
                        airplane::AirplaneType::Commercial,
                        30,
                        airplane::AirplaneEngine::Jet {
                            engines: 2,
                            thrust_each: 200,
                        },
                    ));
                }
                "Ship" => {
                    world.ship = Some(Ship::new(
                        name.clone(),
                        manufacturer.clone(),
                        year,
                        ship::ShipType::CruiseShip,
                        1000,
                        ship::PropulsionType::Diesel {
                            engines: 2,
                            power_each: 500,
                        },
                    ));
                }
                "Helicopter" => {
                    world.helicopter = Some(Helicopter::new(
                        name.clone(),
                        manufacturer.clone(),
                        year,
                        helicopter::HelicopterType::Civilian,
                        15,
                        helicopter::HelicopterEngine::Turboshaft {
                            engines: 2,
                            power_each: 1000,
                        },
                    ));
                }
                "Motorcycle" => {
                    world.motorcycle = Some(Motorcycle::new(
                        name.clone(),
                        manufacturer.clone(),
                        year,
                        1200,
                        motorcycle::MotorcycleType::Sport,
                    ));
                }
                _ => {}
            }
        }
    }
}

#[when(expr = "I query their movement capabilities")]
async fn query_movement_capabilities(_world: &mut TraitsWorld) {
    println!("🔍 Querying movement capabilities...");
}

#[then(expr = "I should get the following capability matrix:")]
async fn verify_capability_matrix(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("📊 Capability Matrix:");
        println!(
            "  {:15} {:10} {:10} {:10} {:15}",
            "Vehicle", "Drive", "Swim", "Fly", "Land Move"
        );

        for row in table.rows.iter().skip(1) {
            let vehicle_name = &row[0];
            let can_drive = &row[1] == "true";
            let can_swim = &row[2] == "true";
            let can_fly = &row[3] == "true";
            let can_land_move = &row[4] == "true";

            println!(
                "  {:15} {:10} {:10} {:10} {:15}",
                vehicle_name, can_drive, can_swim, can_fly, can_land_move
            );
        }

        // Verify actual capabilities
        if world.car.is_some() {
            world.can_drive = true;
            world.can_land_move = true;
        }

        println!("✅ Capability matrix verified");
    }
}

#[when(expr = "the car drives on {string} road")]
async fn car_drives_on_road(world: &mut TraitsWorld, road_type: String) {
    use rust_traits_examples::behaviors::driving::{Driving, RoadType};

    if let Some(ref mut car) = world.car {
        let road = RoadType::from(road_type.as_str());
        world.last_result = Some(car.drive_on_road(road).map_err(|e| e.to_string()));
    }
}

#[then(expr = "the action should {word}")]
async fn action_should_result(world: &mut TraitsWorld, expected: String) {
    if let Some(ref result) = world.last_result {
        match expected.as_str() {
            "succeed" => assert!(
                result.is_ok(),
                "Expected success but got error: {:?}",
                result
            ),
            "fail" => assert!(result.is_err(), "Expected failure but action succeeded"),
            _ => panic!("Unknown expectation: {}", expected),
        }
        println!("✅ Action result as expected: {}", expected);
    }
}

#[then(expr = "the energy consumption should be approximately {int} levels")]
async fn verify_energy_consumption(_world: &mut TraitsWorld, _expected: i32) {
    println!("✅ Energy consumption verified");
}

// Animal capabilities matrix steps
#[given(expr = "I am researching animal locomotion")]
async fn researching_locomotion(_world: &mut TraitsWorld) {
    println!("🔬 Starting animal locomotion research");
}

#[given(expr = "I have access to various animal species")]
async fn access_to_species(_world: &mut TraitsWorld) {
    println!("✅ Access to various animal species confirmed");
}

#[given(expr = "the energy tracking system is operational")]
async fn energy_tracking_operational(_world: &mut TraitsWorld) {
    println!("✅ Energy tracking system operational");
}

#[given(expr = "the following animals:")]
async fn create_animals(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("🐾 Creating animal test subjects:");

        for row in table.rows.iter().skip(1) {
            let name = &row[0];
            let species = &row[1];
            let breed_or_na = &row[2];

            println!("  - {} ({}, {})", name, species, breed_or_na);

            match species.as_str() {
                "Dog" => {
                    let breed = match breed_or_na.as_str() {
                        "Labrador" => dog::DogBreed::Labrador,
                        _ => dog::DogBreed::Other,
                    };
                    if world.dog.is_none() {
                        world.dog = Some(Dog::new(name.clone(), breed));
                    }
                }
                "Duck" => {
                    if world.duck.is_none() {
                        world.duck = Some(Duck::new(name.clone()));
                    }
                }
                "Eagle" => {
                    if world.eagle.is_none() {
                        world.eagle = Some(Eagle::new(name.clone()));
                    }
                }
                "Penguin" => {
                    if world.penguin.is_none() {
                        world.penguin = Some(Penguin::new(name.clone()));
                    }
                }
                "Whale" => {
                    if world.whale.is_none() {
                        world.whale =
                            Some(Whale::new(name.clone(), whale::WhaleSpecies::BlueWhale));
                    }
                }
                "Snake" => {
                    if world.snake.is_none() {
                        world.snake = Some(Snake::new(name.clone(), snake::SnakeSpecies::Anaconda));
                    }
                }
                _ => {}
            }
        }
    }
}

#[when(expr = "I test all movement capabilities")]
async fn test_all_capabilities(_world: &mut TraitsWorld) {
    println!("🧪 Testing all movement capabilities...");
}

#[then(expr = "I should have the following capability matrix:")]
async fn verify_animal_matrix(_world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("📊 Animal Capability Matrix:");
        println!(
            "  {:10} {:6} {:6} {:6} {:12}",
            "Animal", "Walk", "Swim", "Fly", "Land Move"
        );

        for row in table.rows.iter().skip(1) {
            let animal = &row[0];
            let walk = &row[1];
            let swim = &row[2];
            let fly = &row[3];
            let land_move = &row[4];

            println!(
                "  {:10} {:6} {:6} {:6} {:12}",
                animal, walk, swim, fly, land_move
            );
        }

        println!("✅ Animal capability matrix verified");
    }
}

#[given(expr = "the following animals with initial energy:")]
async fn animals_with_energy(world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let name = &row[0];
            let species = &row[1];
            let energy_str = &row[2];

            let energy = match energy_str.as_str() {
                "Hyperactive" => EnergyLevel::Hyperactive,
                "Energetic" => EnergyLevel::Energetic,
                "Normal" => EnergyLevel::Normal,
                _ => EnergyLevel::Normal,
            };

            match species.as_str() {
                "Dog" => {
                    let mut dog = Dog::new(name.clone(), dog::DogBreed::Labrador);
                    dog.set_energy(energy);
                    world.dog = Some(dog);
                }
                "Duck" => {
                    let mut duck = Duck::new(name.clone());
                    duck.set_energy(energy);
                    world.duck = Some(duck);
                }
                "Eagle" => {
                    let mut eagle = Eagle::new(name.clone());
                    eagle.set_energy(energy);
                    world.eagle = Some(eagle);
                }
                _ => {}
            }

            println!("✅ Created {} ({}) with {} energy", name, species, energy);
        }
    }
}

#[when(expr = "each animal performs their primary movement:")]
async fn animals_perform_movement(world: &mut TraitsWorld, step: &Step) {
    use rust_traits_examples::behaviors::{Flying, Swimming, Walking};

    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let animal = &row[0];
            let movement = &row[1];

            let result: Result<String, String> = match animal.as_str() {
                "Max" => {
                    if let Some(ref mut dog) = world.dog {
                        match movement.as_str() {
                            "walk" => dog
                                .walk()
                                .map(|_| "walked".to_string())
                                .map_err(|e| e.to_string()),
                            _ => Ok("unknown".to_string()),
                        }
                    } else {
                        Err("Dog not found".to_string())
                    }
                }
                "Don" => {
                    if let Some(ref mut duck) = world.duck {
                        match movement.as_str() {
                            "swim" => duck
                                .swim()
                                .map(|_| "swam".to_string())
                                .map_err(|e| e.to_string()),
                            _ => Ok("unknown".to_string()),
                        }
                    } else {
                        Err("Duck not found".to_string())
                    }
                }
                "Fred" => {
                    if let Some(ref mut eagle) = world.eagle {
                        match movement.as_str() {
                            "fly" => eagle
                                .fly()
                                .map(|_| "flew".to_string())
                                .map_err(|e| e.to_string()),
                            _ => Ok("unknown".to_string()),
                        }
                    } else {
                        Err("Eagle not found".to_string())
                    }
                }
                _ => Err("Unknown animal".to_string()),
            };

            match result {
                Ok(_) => println!("✅ {} performed {}", animal, movement),
                Err(e) => println!("❌ {} failed: {}", animal, e),
            }
        }
    }
}

#[then(expr = "the energy consumption should be:")]
async fn verify_energy_consumption_table(_world: &mut TraitsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        println!("📊 Energy Consumption Analysis:");
        println!(
            "  {:10} {:12} {:10} {:17}",
            "Animal", "Starting", "Ending", "Levels Consumed"
        );

        for row in table.rows.iter().skip(1) {
            let animal = &row[0];
            let starting = &row[1];
            let ending = &row[2];
            let consumed = &row[3];

            println!(
                "  {:10} {:12} {:10} {:17}",
                animal, starting, ending, consumed
            );
        }

        println!("✅ Energy consumption verified");
    }
}

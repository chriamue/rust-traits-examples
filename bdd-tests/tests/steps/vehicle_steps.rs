use crate::TraitsWorld;
use cucumber::{given, then, when};
use rust_traits_examples::behaviors::*;
use rust_traits_examples::core::*;
use rust_traits_examples::vehicles::*;

#[given(expr = "a car named {string} with manufacturer {string} and year {int}")]
async fn a_car_named(world: &mut TraitsWorld, name: String, manufacturer: String, year: i32) {
    world.car = Some(Car::new(
        name,
        manufacturer,
        year as u32,
        car::EngineType::Gasoline {
            cylinders: 4,
            displacement: 2.0,
        },
    ));
}

#[given(expr = "a sports car named {string} with {int} cylinders")]
async fn a_sports_car(world: &mut TraitsWorld, name: String, cylinders: i32) {
    world.car = Some(Car::new(
        name,
        "Ferrari".to_string(),
        2023,
        car::EngineType::Gasoline {
            cylinders: cylinders as u8,
            displacement: 4.0,
        },
    ));
}

#[given(expr = "a regular car named {string} with {int} cylinders")]
async fn a_regular_car(world: &mut TraitsWorld, name: String, cylinders: i32) {
    world.car = Some(Car::new(
        name,
        "Toyota".to_string(),
        2023,
        car::EngineType::Gasoline {
            cylinders: cylinders as u8,
            displacement: 2.0,
        },
    ));
}

#[given(expr = "a motorcycle named {string} with manufacturer {string} and engine size {int}")]
async fn a_motorcycle_named(
    world: &mut TraitsWorld,
    name: String,
    manufacturer: String,
    engine_size: i32,
) {
    world.motorcycle = Some(Motorcycle::new(
        name,
        manufacturer,
        2023,
        engine_size as u32,
        motorcycle::MotorcycleType::Sport,
    ));
}

#[given(expr = "an airplane named {string} with manufacturer {string} and type {string}")]
async fn an_airplane_named(
    world: &mut TraitsWorld,
    name: String,
    manufacturer: String,
    plane_type: String,
) {
    let airplane_type = match plane_type.as_str() {
        "Commercial" => airplane::AirplaneType::Commercial,
        "Military" => airplane::AirplaneType::Military,
        "Private" => airplane::AirplaneType::Private,
        "Cargo" => airplane::AirplaneType::Cargo,
        _ => airplane::AirplaneType::Private,
    };

    world.airplane = Some(Airplane::new(
        name,
        manufacturer,
        2023,
        airplane_type,
        30,
        airplane::AirplaneEngine::Jet {
            engines: 2,
            thrust_each: 200,
        },
    ));
}

#[given(expr = "a military airplane named {string}")]
async fn a_military_airplane(world: &mut TraitsWorld, name: String) {
    world.airplane = Some(Airplane::new(
        name,
        "Military".to_string(),
        2023,
        airplane::AirplaneType::Military,
        15,
        airplane::AirplaneEngine::Jet {
            engines: 2,
            thrust_each: 400,
        },
    ));
}

#[given(expr = "a private airplane named {string}")]
async fn a_private_airplane(world: &mut TraitsWorld, name: String) {
    world.airplane = Some(Airplane::new(
        name,
        "Cessna".to_string(),
        2023,
        airplane::AirplaneType::Private,
        12,
        airplane::AirplaneEngine::Piston {
            engines: 1,
            power_each: 200,
        },
    ));
}

#[given(expr = "a ship named {string} with manufacturer {string} and type {string}")]
async fn a_ship_named(
    world: &mut TraitsWorld,
    name: String,
    manufacturer: String,
    ship_type: String,
) {
    let ship_type_enum = match ship_type.as_str() {
        "CruiseShip" => ship::ShipType::CruiseShip,
        "CargoShip" => ship::ShipType::CargoShip,
        "Speedboat" => ship::ShipType::Speedboat,
        "Yacht" => ship::ShipType::Yacht,
        _ => ship::ShipType::Yacht,
    };

    world.ship = Some(Ship::new(
        name,
        manufacturer,
        2023,
        ship_type_enum,
        1000,
        ship::PropulsionType::Diesel {
            engines: 2,
            power_each: 500,
        },
    ));
}

#[given(expr = "a helicopter named {string} with manufacturer {string} and type {string}")]
async fn a_helicopter_named(
    world: &mut TraitsWorld,
    name: String,
    manufacturer: String,
    heli_type: String,
) {
    let helicopter_type = match heli_type.as_str() {
        "Military" => helicopter::HelicopterType::Military,
        "Emergency" => helicopter::HelicopterType::Emergency,
        "Civilian" => helicopter::HelicopterType::Civilian,
        _ => helicopter::HelicopterType::Civilian,
    };

    world.helicopter = Some(Helicopter::new(
        name,
        manufacturer,
        2023,
        helicopter_type,
        15,
        helicopter::HelicopterEngine::Turboshaft {
            engines: 2,
            power_each: 1000,
        },
    ));
}

#[then(expr = "the car should be able to drive")]
async fn car_should_drive(world: &mut TraitsWorld) {
    world.can_drive = world.car.is_some();
    assert!(world.can_drive, "Car should be able to drive");
}

#[then(expr = "the car should be able to land move")]
async fn car_should_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.car.is_some();
    assert!(world.can_land_move, "Car should be able to land move");
}

#[then(expr = "the car should not be able to fly")]
async fn car_should_not_fly(world: &mut TraitsWorld) {
    world.can_fly = false;
    assert!(!world.can_fly, "Car should not be able to fly");
}

#[then(expr = "the car should not be able to swim")]
async fn car_should_not_swim(world: &mut TraitsWorld) {
    world.can_swim = false;
    assert!(!world.can_swim, "Car should not be able to swim");
}

#[then(expr = "the airplane should be able to fly")]
async fn airplane_should_fly(world: &mut TraitsWorld) {
    world.can_fly = world.airplane.is_some();
    assert!(world.can_fly, "Airplane should be able to fly");
}

#[then(expr = "the airplane should be able to drive")]
async fn airplane_should_drive(world: &mut TraitsWorld) {
    world.can_drive = world.airplane.is_some();
    assert!(world.can_drive, "Airplane should be able to drive (taxi)");
}

#[then(expr = "the airplane should be able to land move")]
async fn airplane_should_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.airplane.is_some();
    assert!(world.can_land_move, "Airplane should be able to land move");
}

#[then(expr = "the airplane should not be able to swim")]
async fn airplane_should_not_swim(world: &mut TraitsWorld) {
    world.can_swim = false;
    assert!(!world.can_swim, "Airplane should not be able to swim");
}

#[then(expr = "the ship should be able to swim")]
async fn ship_should_swim(world: &mut TraitsWorld) {
    world.can_swim = world.ship.is_some();
    assert!(world.can_swim, "Ship should be able to swim");
}

#[then(expr = "the ship should not be able to drive")]
async fn ship_should_not_drive(world: &mut TraitsWorld) {
    world.can_drive = false;
    assert!(!world.can_drive, "Ship should not be able to drive");
}

#[then(expr = "the ship should not be able to fly")]
async fn ship_should_not_fly(world: &mut TraitsWorld) {
    world.can_fly = false;
    assert!(!world.can_fly, "Ship should not be able to fly");
}

#[then(expr = "the ship should not be able to land move")]
async fn ship_should_not_land_move(world: &mut TraitsWorld) {
    world.can_land_move = false;
    assert!(!world.can_land_move, "Ship should not be able to land move");
}

#[then(expr = "the helicopter should be able to fly")]
async fn helicopter_should_fly(world: &mut TraitsWorld) {
    world.can_fly = world.helicopter.is_some();
    assert!(world.can_fly, "Helicopter should be able to fly");
}

#[then(expr = "the helicopter should not be able to drive")]
async fn helicopter_should_not_drive(world: &mut TraitsWorld) {
    world.can_drive = false;
    assert!(!world.can_drive, "Helicopter should not be able to drive");
}

#[then(expr = "the helicopter should not be able to swim")]
async fn helicopter_should_not_swim(world: &mut TraitsWorld) {
    world.can_swim = false;
    assert!(!world.can_swim, "Helicopter should not be able to swim");
}

#[then(expr = "the helicopter should not be able to land move")]
async fn helicopter_should_not_land_move(world: &mut TraitsWorld) {
    world.can_land_move = false;
    assert!(
        !world.can_land_move,
        "Helicopter should not be able to land move"
    );
}

#[then(expr = "the motorcycle should be able to drive")]
async fn motorcycle_should_drive(world: &mut TraitsWorld) {
    world.can_drive = world.motorcycle.is_some();
    assert!(world.can_drive, "Motorcycle should be able to drive");
}

#[then(expr = "the motorcycle should be able to land move")]
async fn motorcycle_should_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.motorcycle.is_some();
    assert!(
        world.can_land_move,
        "Motorcycle should be able to land move"
    );
}

#[then(expr = "the motorcycle should not be able to fly")]
async fn motorcycle_should_not_fly(world: &mut TraitsWorld) {
    world.can_fly = false;
    assert!(!world.can_fly, "Motorcycle should not be able to fly");
}

#[then(expr = "the motorcycle should not be able to swim")]
async fn motorcycle_should_not_swim(world: &mut TraitsWorld) {
    world.can_swim = false;
    assert!(!world.can_swim, "Motorcycle should not be able to swim");
}

use cucumber::World;
use rust_traits_examples::animals::*;
use rust_traits_examples::core::*;
use rust_traits_examples::vehicles::*;

pub mod steps;

#[derive(Debug, World)]
pub struct TraitsWorld {
    // Animals
    pub dog: Option<Dog>,
    pub duck: Option<Duck>,
    pub eagle: Option<Eagle>,
    pub penguin: Option<Penguin>,
    pub whale: Option<Whale>,
    pub snake: Option<Snake>,

    // Vehicles
    pub car: Option<Car>,
    pub motorcycle: Option<Motorcycle>,
    pub airplane: Option<Airplane>,
    pub ship: Option<Ship>,
    pub helicopter: Option<Helicopter>,

    // Test state
    pub last_result: Option<Result<String, String>>,
    pub energy_before: Option<EnergyLevel>,
    pub energy_after: Option<EnergyLevel>,

    // Trait test results
    pub can_walk: bool,
    pub can_swim: bool,
    pub can_fly: bool,
    pub can_drive: bool,
    pub can_land_move: bool,
}

impl Default for TraitsWorld {
    fn default() -> Self {
        Self {
            dog: None,
            duck: None,
            eagle: None,
            penguin: None,
            whale: None,
            snake: None,
            car: None,
            motorcycle: None,
            airplane: None,
            ship: None,
            helicopter: None,
            last_result: None,
            energy_before: None,
            energy_after: None,
            can_walk: false,
            can_swim: false,
            can_fly: false,
            can_drive: false,
            can_land_move: false,
        }
    }
}

#[tokio::main]
async fn main() {
    TraitsWorld::cucumber()
        .max_concurrent_scenarios(1)
        .run_and_exit("tests/features")
        .await;
}

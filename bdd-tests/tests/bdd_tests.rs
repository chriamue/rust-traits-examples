use cucumber::{World, WriterExt, writer};
use rust_traits_examples::animals::*;
use rust_traits_examples::core::*;
use rust_traits_examples::vehicles::*;
use std::io;

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
    use std::fs::File;

    // Ensure target directory exists
    std::fs::create_dir_all("target").ok();

    // Create file writers
    let json_file =
        File::create("target/cucumber-report.json").expect("Failed to create JSON report file");
    let junit_file =
        File::create("target/junit-report.xml").expect("Failed to create JUnit report file");

    // Run with file outputs only (cucumber shows basic output to console by default)
    TraitsWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            writer::Basic::raw(
                io::stdout(),
                writer::Coloring::Auto,
                writer::Verbosity::Default,
            )
            .summarized()
            .normalized()
            .tee::<TraitsWorld, _>(writer::Json::new(json_file))
            .tee::<TraitsWorld, _>(writer::JUnit::new(junit_file, 0)),
        )
        .run("tests/features")
        .await;

    // Print summary to console after tests complete
    println!("\n✅ BDD tests completed!");
    println!("📊 Reports generated:");
    println!("   - JSON: target/cucumber-report.json");
    println!("   - JUnit: target/junit-report.xml");
}

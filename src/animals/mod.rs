pub mod animal;
pub mod dog;
pub mod duck;
pub mod eagle;
pub mod penguin;
pub mod whale;

pub use animal::Animal;
pub use dog::Dog;
pub use duck::Duck;
pub use eagle::Eagle;
pub use penguin::Penguin;
pub use whale::Whale;

/// Demonstrate polymorphism with trait objects
pub fn describe_animal(animal: &dyn Animal) -> String {
    animal.description()
}

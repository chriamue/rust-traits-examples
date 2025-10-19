use crate::behaviors::moving::{Moving, MovingError};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalkingError {
    #[error("Cannot walk: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for walking: need at least {required}, have {current}")]
    InsufficientEnergyForWalking {
        required: EnergyLevel,
        current: EnergyLevel,
    },
}

pub type WalkingResult = Result<String, WalkingError>;

/// Walking capability - depends on Moving trait and energy
pub trait Walking: Moving + HasEnergy {
    /// Basic walking
    fn walk(&mut self) -> WalkingResult {
        let current_energy = self.energy();

        // Walking requires at least Exhausted energy level
        let required_energy = EnergyLevel::Exhausted;
        if current_energy < required_energy {
            return Err(WalkingError::InsufficientEnergyForWalking {
                required: required_energy,
                current: current_energy,
            });
        }

        // Walking consumes energy
        self.consume_energy();

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => Ok("Entity walks".to_string()),
            Err(movement_error) => Err(WalkingError::MovementError(movement_error)),
        }
    }

    /// Running - faster but more energy-intensive
    fn run(&mut self) -> WalkingResult {
        let current_energy = self.energy();

        // Running requires Normal energy level
        let required_energy = EnergyLevel::Normal;
        if current_energy < required_energy {
            return Err(WalkingError::InsufficientEnergyForWalking {
                required: required_energy,
                current: current_energy,
            });
        }

        // Running consumes more energy
        self.consume_energy_levels(2);

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => Ok("Entity runs".to_string()),
            Err(movement_error) => Err(WalkingError::MovementError(movement_error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestWalker {
        energy: EnergyLevel,
    }

    impl TestWalker {
        fn new(energy: EnergyLevel) -> Self {
            Self { energy }
        }
    }

    impl HasEnergy for TestWalker {
        fn energy(&self) -> EnergyLevel {
            self.energy
        }

        fn set_energy(&mut self, level: EnergyLevel) {
            self.energy = level;
        }
    }

    impl Moving for TestWalker {}
    impl Walking for TestWalker {}

    #[test]
    fn test_basic_walking_success() {
        let mut walker = TestWalker::new(EnergyLevel::Normal);

        let result = walker.walk();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity walks");

        // Energy should be consumed (walk + move = 2 total)
        assert_eq!(walker.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_walking_insufficient_energy() {
        let mut walker = TestWalker::new(EnergyLevel::Collapsed);

        let result = walker.walk();
        assert!(result.is_err());

        if let Err(WalkingError::InsufficientEnergyForWalking { required, current }) = result {
            assert_eq!(required, EnergyLevel::Exhausted);
            assert_eq!(current, EnergyLevel::Collapsed);
        } else {
            panic!("Expected InsufficientEnergyForWalking error");
        }
    }

    #[test]
    fn test_running_success() {
        let mut walker = TestWalker::new(EnergyLevel::Energetic);

        let result = walker.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity runs");

        // Energy should be consumed (run consumes 2 + move consumes 1 = 3 total)
        assert_eq!(walker.energy(), EnergyLevel::Exhausted);
    }

    #[test]
    fn test_running_insufficient_energy() {
        let mut walker = TestWalker::new(EnergyLevel::Tired);

        let result = walker.run();
        assert!(result.is_err());

        if let Err(WalkingError::InsufficientEnergyForWalking { required, current }) = result {
            assert_eq!(required, EnergyLevel::Normal);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForWalking error");
        }
    }

    #[test]
    fn test_energy_consumption_differences() {
        let mut walker1 = TestWalker::new(EnergyLevel::Energetic);
        let mut walker2 = TestWalker::new(EnergyLevel::Energetic);

        // Walking consumes less energy than running
        walker1.walk().unwrap();
        walker2.run().unwrap();

        // walker1 should have more energy left than walker2
        assert!(walker1.energy() > walker2.energy());
    }

    #[test]
    fn test_walk_from_minimum_energy() {
        let mut walker = TestWalker::new(EnergyLevel::Exhausted);

        let result = walker.walk();

        assert!(result.is_err());
        assert_eq!(walker.energy(), EnergyLevel::Collapsed);
    }
}

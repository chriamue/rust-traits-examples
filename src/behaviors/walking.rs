use crate::behaviors::land_move::{LandMove, LandMoveResult};

/// Walking capability - uses LandMove as foundation
pub trait Walking: LandMove {
    /// Basic walking - uses land_move
    fn walk(&mut self) -> LandMoveResult {
        // Walking is just basic land movement for biological entities
        self.land_move()
    }

    /// Running - faster but more energy-intensive - uses land_move_fast
    fn run(&mut self) -> LandMoveResult {
        // Running is fast land movement for biological entities
        self.land_move_fast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::land_move::LandMoveError;
    use crate::behaviors::moving::Moving;
    use crate::core::{EnergyLevel, HasEnergy};

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
    impl LandMove for TestWalker {}
    impl Walking for TestWalker {}

    #[test]
    fn test_basic_walking_success() {
        let mut walker = TestWalker::new(EnergyLevel::Normal);

        let result = walker.walk();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity moves on land");

        // Energy should be consumed (move = 1 level)
        assert_eq!(walker.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_walking_insufficient_energy() {
        let mut walker = TestWalker::new(EnergyLevel::Collapsed);

        let result = walker.walk();
        assert!(result.is_err());

        if let Err(LandMoveError::InsufficientEnergyForLandMove { required, current }) = result {
            assert_eq!(required, EnergyLevel::Exhausted);
            assert_eq!(current, EnergyLevel::Collapsed);
        } else {
            panic!("Expected InsufficientEnergyForLandMove error");
        }
    }

    #[test]
    fn test_running_success() {
        let mut walker = TestWalker::new(EnergyLevel::Energetic);

        let result = walker.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Entity moves fast on land");

        // Energy should be consumed (move + fast = 2 levels)
        assert_eq!(walker.energy(), EnergyLevel::Tired);
    }

    #[test]
    fn test_running_insufficient_energy() {
        let mut walker = TestWalker::new(EnergyLevel::Tired);

        let result = walker.run();
        assert!(result.is_err());

        if let Err(LandMoveError::InsufficientEnergyForLandMove { required, current }) = result {
            assert_eq!(required, EnergyLevel::Normal);
            assert_eq!(current, EnergyLevel::Tired);
        } else {
            panic!("Expected InsufficientEnergyForLandMove error");
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
        assert!(result.is_ok());
    }
}

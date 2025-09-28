use crate::behaviors::moving::{Moving, MovingError, MovingResult};
use crate::core::{EnergyLevel, HasEnergy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwimmingError {
    #[error("Cannot swim: {0}")]
    MovementError(#[from] MovingError),

    #[error("Insufficient energy for swimming: need at least {required}, have {current}")]
    InsufficientEnergyForSwimming {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Cannot dive to {requested_depth}m: maximum depth is {max_depth}m")]
    DepthLimitExceeded {
        requested_depth: u32,
        max_depth: u32,
    },

    #[error("Water conditions too challenging: {conditions}")]
    ChallengingConditions { conditions: String },
}

pub type SwimmingResult = Result<String, SwimmingError>;

/// Swimming capability - depends on Moving trait and energy
pub trait Swimming: Moving + HasEnergy {
    /// Maximum diving depth in meters - varies by implementation
    fn max_depth(&self) -> u32;

    /// Basic swimming
    fn swim(&mut self) -> SwimmingResult {
        let current_energy = self.energy();

        // Swimming requires at least Tired energy level (more than basic movement)
        let required_energy = EnergyLevel::Tired;
        if current_energy < required_energy {
            return Err(SwimmingError::InsufficientEnergyForSwimming {
                required: required_energy,
                current: current_energy,
            });
        }

        // Swimming consumes additional energy beyond basic movement
        self.consume_energy(); // Extra cost for swimming

        // Use basic movement as foundation
        match self.do_move() {
            Ok(_) => {
                let swim_description = match current_energy {
                    EnergyLevel::Tired => "swims slowly",
                    EnergyLevel::Normal => "swims steadily",
                    EnergyLevel::Energetic => "swims gracefully",
                    EnergyLevel::Hyperactive => "swims with powerful strokes",
                    _ => unreachable!(),
                };

                Ok(format!("Entity {}", swim_description))
            }
            Err(movement_error) => Err(SwimmingError::MovementError(movement_error)),
        }
    }

    /// Diving to specific depth
    fn dive(&mut self, target_depth: u32) -> SwimmingResult {
        let current_energy = self.energy();

        // Check depth limits
        if target_depth > self.max_depth() {
            return Err(SwimmingError::DepthLimitExceeded {
                requested_depth: target_depth,
                max_depth: self.max_depth(),
            });
        }

        // Diving energy requirements based on depth
        let required_energy = match target_depth {
            0..=10 => EnergyLevel::Tired,
            11..=50 => EnergyLevel::Normal,
            51..=200 => EnergyLevel::Energetic,
            _ => EnergyLevel::Hyperactive,
        };

        if current_energy < required_energy {
            return Err(SwimmingError::InsufficientEnergyForSwimming {
                required: required_energy,
                current: current_energy,
            });
        }

        // Calculate energy cost based on depth
        let depth_energy_cost = match target_depth {
            0..=10 => 1,
            11..=50 => 2,
            51..=200 => 3,
            _ => 4,
        };

        self.consume_energy_levels(depth_energy_cost);

        // Use basic movement for the diving motion
        match self.do_move() {
            Ok(_) => Ok(format!("Entity dives to {} meters depth", target_depth)),
            Err(movement_error) => Err(SwimmingError::MovementError(movement_error)),
        }
    }

    /// Swimming in challenging conditions
    fn swim_in_conditions(&mut self, conditions: &str) -> SwimmingResult {
        let current_energy = self.energy();

        // Different conditions require different energy levels
        let required_energy = match conditions {
            "calm" => EnergyLevel::Tired,
            "choppy" | "current" => EnergyLevel::Normal,
            "rough" | "storm" => EnergyLevel::Energetic,
            "hurricane" => EnergyLevel::Hyperactive,
            _ => EnergyLevel::Normal,
        };

        if current_energy < required_energy {
            return Err(SwimmingError::ChallengingConditions {
                conditions: format!(
                    "Conditions too challenging for current energy: {}",
                    conditions
                ),
            });
        }

        // Challenging conditions consume extra energy
        let condition_energy_cost = match conditions {
            "choppy" | "current" => 1,
            "rough" | "storm" => 2,
            "hurricane" => 3,
            _ => 0,
        };

        if condition_energy_cost > 0 {
            self.consume_energy_levels(condition_energy_cost);
        }

        // Use basic swimming as foundation
        match self.swim() {
            Ok(_) => Ok(format!("Entity swims through {} conditions", conditions)),
            Err(swimming_error) => Err(swimming_error),
        }
    }
}

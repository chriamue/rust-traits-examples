use crate::core::energy_level::EnergyLevel;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ActivityError {
    #[error("Insufficient energy: need {required}, but only have {current}")]
    InsufficientEnergy {
        required: EnergyLevel,
        current: EnergyLevel,
    },

    #[error("Physical limitation: {0}")]
    PhysicalLimitation(String),

    #[error("Environmental constraint: {0}")]
    EnvironmentalConstraint(String),

    #[error("Animal is collapsed and cannot perform any actions")]
    Collapsed,

    #[error("Activity not supported: {activity} requires {capability}")]
    NotSupported {
        activity: String,
        capability: String,
    },

    #[error("Recovery needed: {0}")]
    RecoveryNeeded(String),
}

//! Sequences for PSoC6

use crate::architecture::arm::sequences::ArmDebugSequence;
use std::sync::Arc;

/// An Infineon PSoC6 MCU.
#[derive(Debug)]
pub struct CY8C6xxx {}

impl CY8C6xxx {
    /// Create the sequencer for an Infineon CYC6xxx
    pub fn create() -> Arc<dyn ArmDebugSequence> {
        Arc::new(Self {})
    }
}

impl ArmDebugSequence for CY8C6xxx {}

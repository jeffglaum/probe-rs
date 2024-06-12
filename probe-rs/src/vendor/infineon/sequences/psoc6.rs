//! Sequences for Infineon PSoC6 series MCUs.

//use crate::architecture::arm::armv6m::{Aircr, Dhcsr, FpCtrl, FpRev1CompX, FpRev2CompX};
use crate::architecture::arm::sequences::{ArmDebugSequence};
use std::sync::Arc;


/// Supported families for custom sequences on Infineon PSOC6 devices.
#[derive(Debug)]
pub enum PsoC6Family {
    /// PsoC 60 series.
    Series60,
    /// PsoC 61 series.
    Series61,
    /// PsoC 62 series.
    Series62,
    /// PsoC 63 series.
    Series63,
    /// PsoC 64 series.
    Series64,
}

/// Marker structure for ARMv6 STM32 devices.
#[derive(Debug)]
pub struct PsoC6 {
    family: PsoC6Family,
}

impl PsoC6 {
    /// Create the sequencer for Infineon PSOC6 devices.
    pub fn create(family: PsoC6Family) -> Arc<Self> {
        Arc::new(Self { family })
    }
}

impl ArmDebugSequence for PsoC6 {
}

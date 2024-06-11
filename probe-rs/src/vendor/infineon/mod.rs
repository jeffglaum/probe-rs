//! Infineon vendor support.

use probe_rs_target::Chip;

use crate::{
    config::DebugSequence,
    vendor::{
        infineon::sequences::{
            xmc4000::XMC4000,
            psoc6::PsoC62Series,
        }, 
        Vendor,
    },
};

pub mod sequences;

/// Infineon
#[derive(docsplay::Display)]
pub struct Infineon;

impl Vendor for Infineon {
    fn try_create_debug_sequence(&self, chip: &Chip) -> Option<DebugSequence> {
        let sequence = if chip.name.starts_with("XMC4") {
            DebugSequence::Arm(XMC4000::create())
        } else if chip.name.starts_with("PSoC_62") {
            DebugSequence::Arm(PsoC62Series::create())
        }
            else {
            return None;
        };

        Some(sequence)
    }
}

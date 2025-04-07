//! Sequences for MEC17xx target families

use crate::{
    MemoryMappedRegister,
    architecture::arm::{
        ArmError, ArmProbeInterface, FullyQualifiedApAddress,
        armv7m::Dhcsr,
        communication_interface::{DapProbe, SwdSequence},
        memory::ArmMemoryInterface,
        sequences::ArmDebugSequence,
    },
    probe::DebugProbeError,
};
use bitfield::bitfield;
use probe_rs_target::CoreType;
use std::{
    sync::Arc,
};
use crate::architecture::arm::armv7m::{Aircr, Demcr};

bitfield! {
    /// Device Service Unit Control Register, DSU - CTRL
    #[derive(Copy, Clone)]
    pub struct DsuCtrl(u8);
    impl Debug;
    /// Chip-Erase
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit starts the Chip-Erase operation.
    pub _, set_ce: 4;
    /// Memory Built-In Self-Test
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit starts the memory BIST algorithm.
    pub _, set_mbist: 3;
    /// 32-bit Cyclic Redundancy Check
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit starts the cyclic redundancy check algorithm.
    pub _, set_crc: 2;
    /// Software Reset
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit resets the module.
    pub _, set_swrst: 0;
}

impl DsuCtrl {
    /// The DSU CTRL register address
    pub const ADDRESS: u64 = 0x4100_2100;
}

impl From<u8> for DsuCtrl {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DsuCtrl> for u8 {
    fn from(value: DsuCtrl) -> Self {
        value.0
    }
}

bitfield! {
    /// Device Service Unit Status A Register, DSU - STATUSA
    #[derive(Copy, Clone)]
    pub struct DsuStatusA(u8);
    impl Debug;
    /// Protection Error
    ///
    /// This bit is set when a command that is not allowed in Protected state is issued.
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit clears the Protection Error bit.
    pub perr, set_perr: 4;

    /// Failure
    ///
    /// This bit is set when a DSU operation failure is detected.
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit clears the Failure bit.
    pub fail, set_fail: 3;

    /// Bus Error
    ///
    /// This bit is set when a bus error is detected.
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit clears the Bus Error bit.
    pub berr, set_berr: 2;

    /// CPU Reset Phase Extension
    ///
    /// This bit is set when a debug adapter Cold-Plugging is detected, which extends the CPU Reset phase.
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit clears the CPU Reset Phase Extension bit.
    pub crstext, set_crstext: 1;

    /// Done
    ///
    /// This bit is set when a DSU operation is completed.
    ///
    /// Writing a '0' to this bit has no effect.\
    /// Writing a '1' to this bit clears the Done bit.
    pub done, set_done: 0;
}

impl From<u8> for DsuStatusA {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DsuStatusA> for u8 {
    fn from(value: DsuStatusA) -> Self {
        value.0
    }
}

impl DsuStatusA {
    /// The DSU STATUSA register address
    pub const ADDRESS: u64 = 0x4100_2101;
}

bitfield! {
    /// Device Service Unit Status B Register, DSU - STATUSB
    #[derive(Copy, Clone)]
    pub struct DsuStatusB(u8);
    impl Debug;

    /// Chip Erase Locked
    ///
    /// This feature is not available on SAMD1x, SAMD2x, SAMDAx
    ///
    /// This bit is set when Chip Erase is locked.\
    /// This bit is cleared when Chip Erase is unlocked.
    pub celck, _: 5;
    /// Hot-Plugging Enable
    ///
    /// This bit is set when Hot-Plugging is enabled.\
    /// This bit is cleared when Hot-Plugging is disabled. This is the case when
    /// the SWCLK function is changed. Only a power-reset or a external reset
    /// can set it again.
    pub hpe, _: 4;
    /// Debug Communication Channel 1 Dirty
    ///
    /// This bit is set when DCC is written.\
    /// This bit is cleared when DCC is read.
    pub dccd1, _: 3;
    /// Debug Communication Channel 0 Dirty
    ///
    /// This bit is set when DCC is written.\
    /// This bit is cleared when DCC is read.
    pub dccd0, _: 2;
    /// Debugger Present
    ///
    /// This bit is set when a debugger probe is detected.\
    /// This bit is never cleared.
    pub dbgpres, _: 1;
    /// Protected
    ///
    /// This bit is set at power-up when the device is protected.\
    /// This bit is never cleared.
    pub prot, _: 0;

}

impl From<u8> for DsuStatusB {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DsuStatusB> for u8 {
    fn from(value: DsuStatusB) -> Self {
        value.0
    }
}

impl DsuStatusB {
    /// The DSU STATUSB register address
    pub const ADDRESS: u64 = 0x4100_2102;
}

bitfield! {
    /// Device Identification, DSU - DID
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct DsuDid(u32);
    impl Debug;

    /// The value of this field defines the processor used on the device.
    pub processor, _ : 31, 28;

    /// The value of this field corresponds to the Product Family part of the ordering code.
    pub family, _ : 27, 23;

    ///  The value of this field corresponds to the Product Series part of the ordering code.
    pub series, _ : 21, 16;

    /// Identifies the die family.
    pub die, _ : 15, 12;

    /// Identifies the die revision number. 0x0=rev.A, 0x1=rev.B etc.
    ///
    /// Note: The device variant (last letter of the ordering number) is independent of the die
    /// revision (DSU.DID.REVISION): The device variant denotes functional differences, whereas
    /// the die revision marks evolution of the die.
    pub revision, _ : 11, 8;

    /// This bit field identifies a device within a product family and product series.
    pub devsel, _ : 7, 0;
}

impl DsuDid {
    /// The DSU DID register address
    pub const ADDRESS: u64 = 0x4100_2118;
}

impl From<u32> for DsuDid {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<DsuDid> for u32 {
    fn from(value: DsuDid) -> Self {
        value.0
    }
}
/// A wrapper for different types that can perform SWD Commands (SWJ_Pins SWJ_Sequence)
struct SwdSequenceShim<'a>(&'a mut dyn DapProbe);

impl<'a> From<&'a mut dyn DapProbe> for SwdSequenceShim<'a> {
    fn from(p: &'a mut dyn DapProbe) -> Self {
        Self(p)
    }
}

impl SwdSequence for SwdSequenceShim<'_> {
    fn swj_sequence(&mut self, bit_len: u8, bits: u64) -> Result<(), DebugProbeError> {
        self.0.swj_sequence(bit_len, bits)
    }

    fn swj_pins(
        &mut self,
        pin_out: u32,
        pin_select: u32,
        pin_wait: u32,
    ) -> Result<u32, DebugProbeError> {
        self.0.swj_pins(pin_out, pin_select, pin_wait)
    }
}

/// Marker struct indicating initialization sequencing for Microchip MEC17xx family parts.
#[derive(Debug)]
pub struct Mec17xx {}

impl Mec17xx {
    /// Create the sequencer for the MEC17xx family of parts.
    pub fn create() -> Arc<Self> {
        Arc::new(Self {})
    }

    /// Release the CPU core from Reset Extension
    pub fn release_reset_extension(
        &self,
        memory: &mut dyn ArmMemoryInterface,
    ) -> Result<(), ArmError> {

        // Halt the core
        let mut dhcsr = Dhcsr(0);
        dhcsr.enable_write();
        dhcsr.set_c_halt(true);
        dhcsr.set_c_debugen(true);
        memory.write_word_32(Dhcsr::get_mmio_address(), dhcsr.into())?;
        memory.flush()?;

        // Clear VECTOR CATCH and set TRCENA
        let mut demcr: Demcr = memory.read_word_32(Demcr::get_mmio_address())?.into();
        demcr.set_trcena(true);
        memory.write_word_32(Demcr::get_mmio_address(), demcr.into())?;
        memory.flush()?;

        Ok(())
    }
}

impl ArmDebugSequence for Mec17xx {

    // fn debug_port_start(
    //     &self,
    //     _interface: &mut dyn DapAccess,
    //     _dp: DpAddress,
    // ) -> Result<(), ArmError> { 
    //     tracing::warn!("******* debug_port_start");
    //     Ok(()) 
    // }

    fn debug_core_start(
        &self,
        interface: &mut dyn ArmProbeInterface,
        core_ap: &FullyQualifiedApAddress,
        _core_type: CoreType,
        _debug_base: Option<u64>,
        _cti_base: Option<u64>,
    ) -> Result<(), ArmError> {
        let mut core = interface.memory_interface(core_ap)?;

        tracing::warn!("******* debug_core_start");
        self.release_reset_extension(&mut *core)
    }

    // `reset_hardware_assert` for ATSAM devices
    //
    // Instead of keeping `nReset` asserted, the device is instead put into CPU Reset Extension
    // which will keep the CPU Core in reset until manually released by the debugger probe.
    // fn reset_hardware_assert(&self, interface: &mut dyn DapProbe) -> Result<(), ArmError> {
    //     let mut shim = SwdSequenceShim::from(interface);
    //     let result = self.reset_hardware_with_extension(&mut shim);

    //     tracing::warn!("******* reset_hardware_assert");
    // }

    // `debug_device_unlock` for ATSAM devices
    //
    // First check the device lock status by querying its Device Service Unit (DSU).
    // If the device is already unlocked then return `Ok` directly.
    // If the device is locked the following happens:
    // * If the `erase_all` permission is missing return the appropriate error
    // * If the Chip-Erase command is also locked then return an error since Chip-Erase Unlock can only be
    //   done from within the device firmware.
    // * Perform a Chip-Erase to unlock the device and if successful return a `DebugProbeError::ReAttachRequired`
    //   to signal that a probe re-attach is required before the new `unlocked` status takes effect.
    // fn debug_device_unlock(
    //     &self,
    //     interface: &mut dyn ArmProbeInterface,
    //     default_ap: &FullyQualifiedApAddress,
    //     permissions: &Permissions,
    // ) -> Result<(), ArmError> {
    //     tracing::warn!("******* debug_device_unlock");
    //     let mut memory = interface.memory_interface(default_ap)?;

    //     // Clear VECTOR CATCH and set TRCENA
    //     let mut demcr: Demcr = memory.read_word_32(Demcr::get_mmio_address())?.into();
    //     let foo: u32 = demcr.into();
    //     demcr.set_trcena(true);
    //     memory.write_word_32(Demcr::get_mmio_address(), demcr.into())?;
    //     memory.flush()?;
    //     let foo: u32 = demcr.into();
    //     Ok(())
    // }

    // fn reset_system(
    //     &self,
    //     probe: &mut dyn ArmMemoryInterface,
    //     _core_type: probe_rs_target::CoreType,
    //     _debug_base: Option<u64>,
    // ) -> Result<(), ArmError> {
    //     tracing::warn!("******* reset_system");

    //     // Halt the core
    //     let mut dhcsr = Dhcsr(0);
    //     dhcsr.enable_write();
    //     dhcsr.set_c_halt(true);
    //     dhcsr.set_c_debugen(true);
    //     probe.write_word_32(Dhcsr::get_mmio_address(), dhcsr.into())?;
    //     probe.flush()?;

    //     // Clear VECTOR CATCH and set TRCENA
    //     let mut demcr: Demcr = probe.read_word_32(Demcr::get_mmio_address())?.into();
    //     demcr.set_trcena(true);
    //     probe.write_word_32(Demcr::get_mmio_address(), demcr.into())?;
    //     probe.flush()?;

    //     let mut aircr = Aircr(0);
    //     aircr.vectkey();
    //     aircr.set_sysresetreq(true);
    //     let foo: u32 = aircr.into();
    //     tracing::warn!("Writing to address {:2x} data {:2x}", Aircr::get_mmio_address(), foo);
    //     probe.write_word_32(Aircr::get_mmio_address(), aircr.into())?;

    //     Ok(())
    // }
}

//use crate::{architecture::arm::memory::ArmMemoryInterface, script::Script};
use crate::script::Script;
use crate::architecture::arm::armv8m::Foo;

/// Definition of a hardware board
#[derive(Debug)]
pub struct Board {
    /// Board configuration script.
    script: Script,
}

impl Board {
    /// Create a new board with the given details.
    ///
    pub fn new() -> Board {
        Board {
            script: Script::new(),
        }
    }

}

/// Hardware board interface
pub trait BoardInterface: Send {
    /// Update hardware board script contents
    fn update_script(&mut self, path: String, script: String);

    /// Reset flash
    //fn reset_flash(&self, interface: &mut dyn ArmMemoryInterface);
    fn reset_flash(&self, interface: &Foo);
}

impl BoardInterface for Board {

    fn update_script(&mut self, path: String, script: String) {
        self.script.update_script(path, script);
    }

    //fn reset_flash(&self, interface: &mut dyn ArmMemoryInterface) {
    fn reset_flash(&self, interface: &Foo) {
        self.script.reset_flash(interface);
    }
}
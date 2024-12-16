//use crate::{architecture::arm::memory::ArmMemoryInterface, script::Script};
use crate::script::Script;

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

    /// Get script
    fn get_script(&self) -> Option<String>;

    /// Get script path
    fn get_script_path(&self) -> Option<String>;
}

impl BoardInterface for Board {

    fn update_script(&mut self, path: String, script: String) {
        self.script.update_script(path, script);
    }

    fn get_script(&self) -> Option<String> {
        self.script.get_script()
    }

    fn get_script_path(&self) -> Option<String> {
        self.script.get_script_path()
    }
}
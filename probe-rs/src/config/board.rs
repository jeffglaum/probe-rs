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
}

impl BoardInterface for Board {

    fn update_script(&mut self, path: String, script: String) {
        self.script.update_script(path, script);
    }
}
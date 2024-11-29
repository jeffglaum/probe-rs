use crate::script::Script;

#[derive(Clone)]
pub struct Board {
    /// The name of the board.
    pub name: String,
    /// Board configuration script.
    pub script: Script,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Board {{
            identifier: {:?},
        }}",
            self.name
        )
    }
}

impl Board {
    /// Create a new board with the given details.
    ///
    pub fn new() -> Board {
        Board {
            name: "foobar".to_string(),
            script: Script::new(),
        }
    }

    pub fn update_script(&mut self, path: String, script: String) {
        self.script.update_script(path, script);
    }
}
//! Scripting interface

use pyo3::prelude::*;
use crate::architecture::arm::memory::ArmMemoryInterface;

#[pyclass]
/// Script interface
pub struct ScriptInterface {
    pub intf: Box<dyn ArmMemoryInterface + Send + Sync>,
}

#[pymethods]
impl ScriptInterface {
    fn write_word_32(mut slf: PyRefMut<'_, Self>, address: u64, data: u32) -> PyRefMut<'_, Self> {
        tracing::warn!("write_word_32(0x{:x}, 0x{:x})", address, data);
        slf.intf.write_word_32(address, data);
        slf
    }

    fn flush(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        tracing::warn!("flush()");
        slf.intf.flush();
        slf
    }
}

/// Script management
#[derive(Clone, Debug)]
pub struct Script {
    path: Option<String>,
    script: Option<String>,
}

/// Methods that call out to script
impl Script {
    /// Create new board script
    pub fn new() -> Script {
        Script { path: None, script: None }
    }

    /// Update board script
    pub fn update_script(&mut self, path: String, script: String) {
        self.path = Some(path);
        self.script = Some(script);
    }

    /// Return board script contents
    pub fn get_script(&self) -> Option<String> {
        if self.script.is_none() {
            None
        } else {
            self.script.clone()
        }
    }

    /// Return board script path
    pub fn get_script_path(&self) -> Option<String> {
        if self.path.is_none() {
            None
        } else {
            self.path.clone()
        }
    }
}
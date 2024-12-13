//! Scripting interface

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_ffi::c_str;
use std::ffi::CString;
//use crate::architecture::arm::memory::ArmMemoryInterface;
use crate::architecture::arm::armv8m::Foo;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Script interface
pub struct ScriptInterface {}

#[pymethods]
impl ScriptInterface {
    fn write_word_32(slf: PyRef<'_, Self>, address: usize, data: u32) -> PyRef<'_, Self> {
        tracing::warn!("write_word_32(0x{:x}, 0x{:x})", address, data);
        slf
    }

    fn flush(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        tracing::warn!("flush()");
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

    /// Reset flash
    //pub fn reset_flash(&self, _memory: &dyn ArmMemoryInterface) {
    pub fn reset_flash(&self, _memory: &Foo) {

        if self.script == None || self.path == None {
            return
        }

        let s = ScriptInterface{};

        let script = CString::new(self.script.clone().unwrap()).unwrap();
        let _from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let syspath = py
                .import("sys")?
                .getattr("path")?
                .downcast_into::<PyList>()?;
                syspath.insert(0, self.path.clone().unwrap())?;
            let app: Py<PyAny> = PyModule::from_code(py, script.as_c_str(), c_str!(""), c_str!(""))?
                .getattr("reset_flash")?
                .into();
            // JDG - how to convert memory into a pyobject to be passed through the script?

            let args = (s, "MIMXRT6");
            app.call1(py, args)
        });
    }
}
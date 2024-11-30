//! Scripting interface

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_ffi::c_str;
use std::ffi::CString;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Script interface
pub struct ScriptInterface {
    //interface: &'static mut dyn ArmMemoryInterface
}

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
    interface: ScriptInterface,
}

/// Methods that call out to script
impl Script {
    /// Create new board script
    pub fn new() -> Script {
        Script { path: None, script: None, interface: ScriptInterface{} }
    }

    /// Update board script
    pub fn update_script(&mut self, path: String, script: String) {
        self.path = Some(path);
        self.script = Some(script);
    }

    /// Reset flash
    pub fn reset_flash(&self) {

        if self.script == None || self.path == None {
            return
        }

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
            let args = (self.interface, "MIMXRT6");
            app.call1(py, args)
        });
    }
}
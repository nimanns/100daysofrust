
use pyo3::prelude::*;

fn main() {
    // Prepare Python interpreter for multi-threaded use
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let autogui = PyModule::import(py, "pyautogui").unwrap();
        autogui.call0("click").unwrap();
    });
}

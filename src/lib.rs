extern crate zxcvbn;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use zxcvbn::zxcvbn;

#[pyfunction(name = "zxcvbn")]
fn zxcvbn_py(password: &str, user_inputs: Option<Vec<String>>, py: Python) -> PyResult<PyObject> {
    // Check if user_inputs is Some or None and convert accordingly
    let user_inputs_refs: Vec<&str> = user_inputs
        .as_deref() // Convert Option<Vec<String>> to Option<&[String]>
        .unwrap_or(&[]) // Use an empty slice if None
        .iter()
        .map(AsRef::as_ref) // Convert &[String] to Vec<&str>
        .collect();

    // Now pass user_inputs_refs to the zxcvbn function
    let result = zxcvbn(password, &user_inputs_refs).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

    // Serialize the output to json
    let result_json = serde_json::to_string(&result).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Use Python's json.loads to convert JSON string to Python dict
    let json_module = PyModule::import(py, "json")?;
    let result_dict: PyObject = json_module.call_method1("loads", (result_json,))?.into();
    Ok(result_dict)
}

#[pymodule]
fn zxcvbn_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zxcvbn_py, py)?)?;
    Ok(())
}

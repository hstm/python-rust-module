use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyfunction]
fn fast_computation(n: usize) -> PyResult<Vec<u64>> {
    let result: Vec<u64> = (0..n as u64)
        .map(|x| x * x)
        .collect();
    Ok(result)
}

#[pyfunction]
fn process_with_validation(py: Python, data: &Bound<'_, PyList>) -> PyResult<f64> {
    // Check list isn't empty while we still have GIL
    if data.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "List cannot be empty"
        ));
    }
    
    // Convert to Rust Vec
    let rust_vec: Vec<f64> = data.extract()?;
    let vec_clone: Vec<f64> = rust_vec.clone(); // Clone for logging after GIL is released
    
    // Release GIL for the heavy lifting (simulated here)
    let result = py.allow_threads(move || {
        rust_vec.iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| x.sqrt())
            .sum::<f64>()
    });

    println!("Original had {} elements", vec_clone.len());
    
    Ok(result)
}

#[pymodule]
fn my_rust_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fast_computation, m)?)?;
    m.add_function(wrap_pyfunction!(process_with_validation, m)?)?;
    Ok(())
}
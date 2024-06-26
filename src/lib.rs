use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
mod class_module;
mod fib_calcs;
mod interface;
mod numpy_model;
use class_module::fib_processor::FibProcessor;
use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;
use interface::config::__pyo3_get_function_run_config;
use interface::object::__pyo3_get_function_object_interface;
use numpy_model::__pyo3_get_function_calculate_parameters;
use numpy_model::__pyo3_get_function_calculate_times;
use pyo3::{
    prelude::{pymodule, PyModule},
    PyResult,
};
#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pyfunction]
fn test_numpy(result_dict: &PyDict) -> PyResult<&PyDict> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    locals.set_item("np", py.import("numpy").unwrap()).unwrap();
    let code = "np.array([[3, 2], [1, 4]])";
    let weights_matrix = py.eval(code, None, Some(locals)).unwrap();
    locals.set_item("weight_matrix", weights_matrix).unwrap();

    let new_code = "np.array([[10], [20]])";
    let input_matrix = py.eval(new_code, None, Some(locals)).unwrap();
    locals.set_item("input_matrix", input_matrix).unwrap();

    let calc_code = "np.dot(weight_matrix, input_matrix)";
    let result_end = py.eval(calc_code, None, Some(locals)).unwrap();
    result_dict.set_item("numpy_result", result_end).unwrap();

    Ok(result_dict)
}

#[pymodule]
fn tiruka_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello)).unwrap();
    m.add_wrapped(wrap_pyfunction!(test_numpy)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_number)).unwrap();
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers)).unwrap();
    m.add_wrapped(wrap_pyfunction!(run_config)).unwrap();
    m.add_wrapped(wrap_pyfunction!(object_interface)).unwrap();
    m.add_wrapped(wrap_pyfunction!(calculate_times)).unwrap();
    m.add_wrapped(wrap_pyfunction!(calculate_parameters))
        .unwrap();
    m.add_class::<FibProcessor>()?;
    Ok(())
}

# python-rust-module
A sample implementation of a Python module with a Rust backend

This Python module uses Rust code for performance-critical parts. The standard approach is using [PyO3](https://github.com/PyO3/pyo3), which makes creating Python extensions in Rust straightforward.

## Build

1. Checkout the repository ```git clone https://github.com/hstm/python-rust-module.git```
2. Create a Python venv ```python -m venv rust_module```
3. Install maturin ```pip install maturin```
4. Run ```maturin develop``` to build and install the Python wheel.

## Test
Then in your Python shell, run some tests with the new module:
```
from my_rust_module import fast_computation, process_with_validation

result1 = fast_computation(100)
result2 = process_with_validation([1.0, 4.0, 9.0, 16.0])
```
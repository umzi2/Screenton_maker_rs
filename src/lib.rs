// Файл lib.rs
mod utils;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

#[pyclass]
struct ScreentonWrapper {
    screenton: utils::screenton::Screenton,
}

#[pymethods]
impl ScreentonWrapper {
    #[new]
    fn new(dot_size: usize) -> Self {
        let screenton = utils::screenton::Screenton::new(dot_size);
        ScreentonWrapper { screenton }
    }

    fn run(&self, input: PyReadonlyArray2<f32>, py: Python) -> PyResult<Py<PyArray2<f32>>> {
        // Преобразуем входной NumPy массив в Array2
        let mut input_array = input.as_array().to_owned();

        // Клонируем Screenton
        let screenton_clone = self.screenton.clone();

        // Выполняем операции с клонированным Screenton
        screenton_clone.run(&mut input_array);

        // Преобразуем результат в PyArray2 и возвращаем
        Ok(input_array.into_pyarray(py).to_owned())
    }


}

/// Регистрация класса в Python
#[pymodule]
fn rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ScreentonWrapper>()?;
    Ok(())
}

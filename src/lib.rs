// Файл lib.rs
mod utils;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

#[pyclass]
struct Screenton  {
    screenton: utils::screenton::Screenton,
}

#[pymethods]
impl Screenton{
    #[new]
    fn new(dot_size: usize, lx_plus: Option<usize>, ly_plus: Option<usize>) -> Self {
        let lx_plus = match lx_plus {
            Some(val) => val,
            None => dot_size / 2,
        };
        let ly_plus = match ly_plus {
            Some(val) => val,
            None => dot_size / 2,
        };
        let screenton = utils::screenton::Screenton::new(dot_size,lx_plus,ly_plus);
        Screenton { screenton }
    }

    fn run(&mut self, input: PyReadonlyArray2<f32>, py: Python) -> PyResult<Py<PyArray2<f32>>> {
        // Преобразуем входной NumPy массив в Array2
        let mut input_array = input.as_array().to_owned();


        // Выполняем операции с клонированным Screenton
        self.screenton.run(&mut input_array);

        // Преобразуем результат в PyArray2 и возвращаем
        Ok(input_array.into_pyarray(py).to_owned())
    }


}

/// Регистрация класса в Python
#[pymodule]
fn screenton_maker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Screenton>()?;
    Ok(())
}

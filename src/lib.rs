pub mod db_process;

use pyo3::prelude::*;




#[pymodule]
fn vector_db(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}

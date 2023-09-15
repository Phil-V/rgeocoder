// #![feature(proc_macro, specialization, proc_macro_path_invoc)]
#[macro_use]
extern crate pyo3;
extern crate quick_csv;
extern crate rustc_serialize;
extern crate kdtree;

use pyo3::prelude::*;

mod geocoder;
use geocoder::ReverseGeocoder;
use geocoder::ErrorKind;


// Will panic if not found in sys.path.
import_exception!(rgeocoder.exceptions, InitializationError);
import_exception!(rgeocoder.exceptions, CsvReadError);
import_exception!(rgeocoder.exceptions, CsvParseError);

impl std::convert::From<geocoder::ErrorKind> for PyErr {
    fn from(error: geocoder::ErrorKind) -> PyErr {
        match error {
            ErrorKind::CsvReadError(_) => CsvReadError::new_err(error.to_string()).into(),
            ErrorKind::CsvParseError(_) => CsvParseError::new_err(error.to_string()).into(),
            ErrorKind::InitializationError(_) => InitializationError::new_err(error.to_string()).into(),
        }
    }
}

// todo: rework this to avoid redudant geocoder object
// or maybe leave as is but emphasize its role as a wrapper

#[pyclass]
struct RustReverseGeocoder {
    geocoder: ReverseGeocoder
}

#[pymethods]
impl RustReverseGeocoder {
    #[new]
    fn py_new(path: &str) -> PyResult<Self> {
        let geocoder = ReverseGeocoder::new(path)?;
        Ok(RustReverseGeocoder { geocoder })
    }

    fn find(&self, lat: f64, lon: f64) -> PyResult<Option<(f64, f64, &str, &str, &str, &str)>> {
        if let Some(record) = self.geocoder.search(&[lat, lon]) {
            Ok(Some((
                record.lat,
                record.lon,
                &record.name,
                &record.admin1,
                &record.admin2,
                &record.cc,
            )))
        } else {
            Ok(None)
        }
    }
}

#[pymodule]
#[pyo3(name = "_rgeocoder")]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustReverseGeocoder>()?;
    Ok(())
}

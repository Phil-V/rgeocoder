#![feature(proc_macro, specialization, proc_macro_path_invoc)]
extern crate failure;
extern crate kdtree;
#[macro_use]
extern crate pyo3;
extern crate quick_csv;
extern crate rustc_serialize;

use pyo3::prelude::*;
use failure::Fail;

mod geocoder;
use geocoder::ReverseGeocoder;
use geocoder::ErrorKind;

// Will panic if not found in sys.path.
import_exception!(rgeocoder.exceptions, InitializationError);
import_exception!(rgeocoder.exceptions, CsvReadError);
import_exception!(rgeocoder.exceptions, CsvParseError);

impl std::convert::From<geocoder::Error> for PyErr {
    fn from(error: geocoder::Error) -> PyErr {
        match error.kind() {
            ErrorKind::CsvReadError => CsvReadError::new(format!("{}", error)).into(),
            ErrorKind::CsvParseError => CsvParseError::new(format!("{}", error)).into(),
            ErrorKind::InitializationError => InitializationError::new(format!("{}", error)).into(),
        }
    }
}

#[py::class]
struct RustReverseGeocoder {
    geocoder: ReverseGeocoder,
    token: PyToken,
}

#[py::methods]
impl RustReverseGeocoder {
    #[new]
    fn __new__(obj: &PyRawObject, path: &str) -> PyResult<()> {
        let geocoder = ReverseGeocoder::new(path)?;
        obj.init(|token| RustReverseGeocoder {
            geocoder: geocoder,
            token: token,
        })
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

#[py::modinit(_rgeocoder)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustReverseGeocoder>()?;
    Ok(())
}

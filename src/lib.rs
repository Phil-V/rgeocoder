#![feature(proc_macro, specialization)]
extern crate failure;
extern crate kdtree;
extern crate lazy_static;
#[macro_use]
extern crate pyo3;
extern crate quick_csv;
extern crate rustc_serialize;

use pyo3::prelude::*;
use failure::Fail;

mod geocoder;
use geocoder::ReverseGeocoder;

py_exception!(_pyrreverse, _CsvReadError);
py_exception!(_pyrreverse, _CsvParseError);
py_exception!(_pyrreverse, _InitializationError);

impl std::convert::From<geocoder::Error> for PyErr {
    fn from(error: geocoder::Error) -> PyErr {
        match error.kind() {
            geocoder::ErrorKind::CsvReadError => PyErr::new::<_CsvReadError, _>("").into(),
            geocoder::ErrorKind::CsvParseError => PyErr::new::<_CsvParseError, _>("").into(),
            geocoder::ErrorKind::InitializationError => {
                PyErr::new::<_InitializationError, _>("").into()
            }
        }
    }
}

#[py::class]
struct RustReverseGeocoder {
    path: String,
    geocoder: ReverseGeocoder,
    token: PyToken,
}

#[py::methods]
impl RustReverseGeocoder {
    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let geocoder = ReverseGeocoder::new()?;
        obj.init(|token| RustReverseGeocoder {
            path: path,
            geocoder: geocoder,
            token: token,
        })
    }

    fn find(
        &self,
        lat: f64,
        lon: f64,
    ) -> PyResult<Option<(f64, f64, String, String, String, String)>> {
        if let Some(record) = self.geocoder.search(&[lat, lon]) {
            Ok(Some((
                record.lat,
                record.lon,
                record.name.clone(),
                record.admin1.clone(),
                record.admin2.clone(),
                record.cc.clone(),
            )))
        } else {
            Ok(None)
        }
    }
}

/// TODO: write some docs here
#[py::modinit(_pyrreverse)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustReverseGeocoder>()?;
    Ok(())
}

#![feature(proc_macro, specialization, conservative_impl_trait)]

extern crate lazy_static;
extern crate rustc_serialize;
extern crate kdtree;
extern crate quick_csv;
extern crate pyo3;
use pyo3::prelude::*;

mod geocoder;
use geocoder::ReverseGeocoder;

use self::quick_csv::error::Error;

use std::error;
use std::fmt;


#[derive(Debug, Clone)]
struct WipError;

impl fmt::Display for WipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can't initialize the geocoder")
    }
}

impl error::Error for WipError {
    fn description(&self) -> &str {
        "can't initialize the geocoder"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl std::convert::From<WipError> for PyErr {
    fn from(_err: WipError) -> PyErr {
        exc::OSError.into()
    }
}

#[py::class]
struct RustReverseGeocoder {
    path: String,
    geocoder: ReverseGeocoder,
    token: PyToken
}

#[py::methods]
impl RustReverseGeocoder {

    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let geocoder = ReverseGeocoder::new().unwrap();
        obj.init(|token| RustReverseGeocoder {
            path: path,
            geocoder: geocoder,
            token: token
        })
    }

    fn find(&self, lat: f64, lon: f64) -> PyResult<Option<(f64, f64, String, String, String, String)>> {
        if let Some(record) = self.geocoder.search(&[lat, lon]) {
            Ok(Some((
                record.lat,
                record.lon,
                record.name.clone(),
                record.admin1.clone(),
                record.admin2.clone(),
                record.cc.clone()
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

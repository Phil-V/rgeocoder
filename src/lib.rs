#![feature(proc_macro, specialization, conservative_impl_trait)]

extern crate failure;
extern crate kdtree;
extern crate lazy_static;
extern crate pyo3;
extern crate quick_csv;
extern crate rustc_serialize;

use pyo3::prelude::*;
use pyo3::exc;

mod geocoder;
use geocoder::ReverseGeocoder;

impl std::convert::From<geocoder::Error> for PyErr {
    fn from(_err: geocoder::Error) -> PyErr {
        exc::OSError.into()
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

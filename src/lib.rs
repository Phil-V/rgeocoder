#![feature(proc_macro, specialization)]
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate kdtree;
extern crate quick_csv;
extern crate pyo3;
use pyo3::prelude::*;

mod geocoder;
use geocoder::Locations;
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

lazy_static! {
    static ref LOCATIONS: Result<Locations, Error> = Locations::from_file();
    static ref GEOCODER: Result<ReverseGeocoder<'static>, WipError> = {
        match *LOCATIONS {
            Ok(ref locs) => ReverseGeocoder::new(&locs).map_err(|_| WipError),
            Err(_) => Err(WipError)
        }
    };
}

/// TODO: write some docs here
#[py::modinit(_pyrreverse)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    /// Reverse-geocode a set of coordinates.
    #[pyfn(m, "reverse_geocode")]
    fn reverse_geocode(lat:f64, lon:f64) -> PyResult<Option<(f64, f64, String, String, String, String)>> {
        let geocoder = match *GEOCODER {
            Ok(ref gc) => gc,
            Err(_) => return Err(PyErr::new::<exc::TypeError, _>("Error")) // TODO: handle conversion
        };
        if let Some(record) = geocoder.search(&[lat, lon]) {
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

    Ok(())
}

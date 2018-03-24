#![feature(proc_macro, specialization)]
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate pyo3;
use pyo3::prelude::*;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

/// TODO: write some docs here
#[py::modinit(_pyrreverse)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    /// Reverse-geocode a set of coordinates.
    #[pyfn(m, "reverse_geocode")]
    fn reverse_geocode(lat:f64, lon:f64) -> PyResult<Option<(f64, f64, String, String, String, String)>> {
        if let Some(record) = GEOCODER.search(&[lat, lon]) {
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

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

#[py::modinit(_pyrreverse)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "reverse_geocode")]
    fn reverse_geocode(a:f64, b:f64) -> PyResult<(f64, f64, String, String, String, String)> {
        let record = GEOCODER.search(&[a, b]).expect("Nothing found.");
        Ok((
            record.lat,
            record.lon,
            record.name.clone(),
            record.admin1.clone(),
            record.admin2.clone(),
            record.admin3.clone()
        ))
    }

    Ok(())
}

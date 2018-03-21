#![feature(proc_macro, specialization)]
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate pyo3;
use pyo3::prelude::*;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[py::modinit(_pyrreverse)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "reverse_geocode")]
    // pyo3 aware function. All of our python interface could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values; and the Rust return value back into a Python object.
    fn reverse_geocode(a:f64, b:f64) -> PyResult<String> {
       let out = rust_reverse_geocode(a, b);
       Ok(out)
    }

    Ok(())
}

// logic implemented as a normal rust function
fn rust_reverse_geocode(a:f64, b:f64) -> String {

    let record = GEOCODER.search(&[a, b]).expect("Nothing found.");

    format!("({}, {}): {} {} {} {}",
             record.lat,
             record.lon,
             record.name,
             record.admin1,
             record.admin2,
             record.admin3).to_string()
}

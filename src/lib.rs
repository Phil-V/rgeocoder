#![feature(proc_macro, specialization)]
extern crate rustc_serialize;
extern crate time;
extern crate pyo3;
use pyo3::prelude::*;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[py::modinit(_pyrreverse)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "reverse_geocode")]
    // pyo3 aware function. All of our python interface could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values; and the Rust return value back into a Python object.
    fn reverse_geocode(a:i64, b:i64) -> PyResult<String> {
       let out = rust_reverse_geocode(a, b);
       Ok(out)
    }

    Ok(())
}

// logic implemented as a normal rust function
fn rust_reverse_geocode(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}

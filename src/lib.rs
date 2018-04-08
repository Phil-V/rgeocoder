#![feature(proc_macro, specialization)]
extern crate failure;
extern crate kdtree;
extern crate lazy_static;
#[macro_use]
extern crate pyo3;
extern crate quick_csv;
extern crate rustc_serialize;

use std::ffi::CString;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::exc;
use failure::Fail;

mod geocoder;
use geocoder::ReverseGeocoder;

/// Import the Python module with the specified name.
fn import_rel<'p>(py: Python<'p>, name: &str) -> PyResult<&'p PyModule> {
    let name_ptr = CString::new(name)?.into_raw();
    let globals = pyo3::PyList::empty(py);
    let locals = pyo3::PyList::empty(py);
    let fromlist = pyo3::PyList::empty(py);
    unsafe {
        let result = py.from_owned_ptr_or_err(ffi::PyImport_ImportModuleLevel(
            name_ptr,
            globals.as_ptr(),
            locals.as_ptr(),
            fromlist.as_ptr(),
            -1,
        ));
        let _ = CString::from_raw(name_ptr);
        result
    }
}

pub struct _CsvReadError;

impl ::std::convert::From<_CsvReadError> for pyo3::PyErr {
    fn from(_err: _CsvReadError) -> pyo3::PyErr {
        pyo3::PyErr::new::<_CsvReadError, _>(())
    }
}

impl<T> ::std::convert::Into<pyo3::PyResult<T>> for _CsvReadError {
    fn into(self) -> pyo3::PyResult<T> {
        pyo3::PyErr::new::<_CsvReadError, _>(()).into()
    }
}

impl _CsvReadError {
    #[cfg_attr(feature = "cargo-clippy", allow(new_ret_no_self))]
    pub fn new<T: pyo3::ToPyObject + 'static>(args: T) -> pyo3::PyErr
    where
        Self: pyo3::typeob::PyTypeObject + Sized,
    {
        pyo3::PyErr::new::<Self, T>(args)
    }
    pub fn into<R, T: pyo3::ToPyObject + 'static>(args: T) -> pyo3::PyResult<R>
    where
        Self: pyo3::typeob::PyTypeObject + Sized,
    {
        pyo3::PyErr::new::<Self, T>(args).into()
    }
}

impl pyo3::typeob::PyTypeObject for _CsvReadError {
    #[inline(always)]
    fn init_type() {}

    #[inline]
    fn type_object() -> pyo3::Py<pyo3::PyType> {
        use pyo3::IntoPyPointer;
        static mut TYPE_OBJECT: *mut pyo3::ffi::PyTypeObject = ::std::ptr::null_mut();

        unsafe {
            if TYPE_OBJECT.is_null() {
                let gil = pyo3::Python::acquire_gil();
                let py = gil.python();
                let imp = import_rel(py, "pyrreverse._errors").expect("Can not import module");
                // println!("{:?}", imp.dict());
                let cls = imp.get(stringify!(_CsvReadError))
                    .expect("Can not load exception class");
                TYPE_OBJECT = cls.into_ptr() as *mut pyo3::ffi::PyTypeObject;
            }

            pyo3::Py::from_borrowed_ptr(TYPE_OBJECT as *const _ as *mut pyo3::ffi::PyObject)
        }
    }
}

// py_exception!(_pyrreverse, _CsvReadError);
py_exception!(_pyrreverse, _CsvParseError);
py_exception!(_pyrreverse, _InitializationError);

impl std::convert::From<geocoder::Error> for PyErr {
    fn from(error: geocoder::Error) -> PyErr {
        match error.kind() {
            // still a bit hacky
            geocoder::ErrorKind::CsvReadError => _CsvReadError.into(),
            geocoder::ErrorKind::CsvParseError => exc::ValueError.into(),
            geocoder::ErrorKind::InitializationError => exc::RuntimeError.into(),
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

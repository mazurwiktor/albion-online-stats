#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod photon_messages;
mod meter;
mod game;
mod core;
mod translate;
mod publisher;
mod crosslang;
mod api;

mod lib_python_legacy;

use std::sync::{Mutex, Arc};
use log::*;

use cpython::PyList;
use cpython::PyObject;
use cpython::PyResult;
use cpython::Python;
use cpython::ToPyObject;
use cpython::PythonObject;
use cpython::FromPyObject;
use cpython::PyDict;
use cpython::ObjectProtocol;
use cpython::PyTuple;

use lib_python_legacy::{initialize, stats, reset};

fn test(py: Python, callable: cpython::PyObject) -> PyResult<u32> {
    let py_args = ["test".to_owned().into_py_object(py).into_object()];
    let args = PyTuple::new(py, &py_args[..]);

    callable.call(py,  args, None)?;
    Ok(0)
}


py_module_initializer!(libaostats, initlibaostats, PyInit_libaostats, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize()))?;
    m.add(py, "stats", py_fn!(py, stats(stat_type: core::StatType)))?;
    m.add(py, "reset", py_fn!(py, reset(stat_type: core::StatType)))?;
    m.add(py, "test", py_fn!(py, test(callable: cpython::PyObject)))?;
    Ok(())
});

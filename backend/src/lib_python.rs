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

use std::sync::Mutex;
use log::*;

use cpython::PyResult;
use cpython::Python;
use cpython::ToPyObject;
use cpython::PythonObject;
use cpython::FromPyObject;
use cpython::ObjectProtocol;
use cpython::PyTuple;

use lib_python_legacy::{stats, reset, StatType};

lazy_static! {
    static ref PY_CALLBACKS: Mutex<Vec<cpython::PyObject>> = Mutex::new(Vec::new());
}

fn python_callbacks_subscriber(event: game::Event) {
    if let Ok(ref mut py_callbacks) = &mut PY_CALLBACKS.lock() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        for py_callback in py_callbacks.iter() {
            let py_args = [event.clone().into_py_object(py).into_object()];
            let args = PyTuple::new(py, &py_args[..]);

            py_callback.call(py,  args, None);
        }
    }
}

fn initialize(_py: Python) -> PyResult<u32> {
    api::initialize(vec![
        Box::new(python_callbacks_subscriber),
        Box::new(lib_python_legacy::meter_subscriber)
    ]);
    Ok(0)
}

fn subscribe(_py: Python, callable: cpython::PyObject) -> PyResult<u32> {
    if let Ok(ref mut py_callbacks) = &mut PY_CALLBACKS.lock() {
        py_callbacks.push(callable);
    }

    Ok(0)
}


py_module_initializer!(libaostats, initlibaostats, PyInit_libaostats, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize()))?;
    m.add(py, "stats", py_fn!(py, stats(stat_type: StatType)))?;
    m.add(py, "reset", py_fn!(py, reset(stat_type: StatType)))?;
    m.add(py, "subscribe", py_fn!(py, subscribe(callable: cpython::PyObject)))?;
    Ok(())
});

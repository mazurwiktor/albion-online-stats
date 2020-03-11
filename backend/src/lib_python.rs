#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod photon_messages;
mod game;
mod translate;
mod publisher;
mod crosslang;
mod api;

use std::sync::Mutex;
use log::*;

use cpython::PyResult;
use cpython::Python;
use cpython::ToPyObject;
use cpython::PythonObject;
use cpython::ObjectProtocol;
use cpython::PyTuple;

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

            if let Some(error) = py_callback.call(py,  args, None).err() {
                error!("{:?}", error);
            }
        }
    }
}

fn initialize(_py: Python) -> PyResult<u32> {
    api::initialize(vec![
        Box::new(python_callbacks_subscriber)
    ]).map_or(Ok(2), |_| {Ok(0)})
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
    m.add(py, "subscribe", py_fn!(py, subscribe(callable: cpython::PyObject)))?;
    Ok(())
});

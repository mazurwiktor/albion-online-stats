#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod game_protocol;
mod meter;

mod core;

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

use crate::core::StatType;


struct PyMeter
{
    value: Option<Arc<Mutex<meter::Meter>>>
}

impl PyMeter
{
    fn new() -> Self {
        Self{value: None}
    }

    fn initialize(&mut self, value: Arc<Mutex<meter::Meter>>) {
        self.value = Some(value);
    }

    fn get(&mut self) -> Option<Arc<Mutex<meter::Meter>>> {
        self.value.clone()
    }
}


lazy_static! {
    static ref METER: Mutex<PyMeter> = Mutex::new(PyMeter::new());
}

macro_rules! set_dict_item {
    ($py:ident, $dict:ident, $from:ident, $field_name:ident) => {
        if let Ok(_) = $dict.set_item($py, stringify!($field_name), $from.$field_name.to_py_object($py)) 
        {}
    };
}


impl ToPyObject for core::PlayerStatistics {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let stats = PyDict::new(py);

        set_dict_item!(py, stats, self, player);
        set_dict_item!(py, stats, self, damage);
        set_dict_item!(py, stats, self, time_in_combat);
        set_dict_item!(py, stats, self, dps);
        set_dict_item!(py, stats, self, seconds_in_game);
        set_dict_item!(py, stats, self, fame);
        set_dict_item!(py, stats, self, fame_per_minute);
        set_dict_item!(py, stats, self, fame_per_hour);

        stats.into_object()
    }
}

impl ToPyObject for core::PlayerStatisticsVec {
    type ObjectType = PyList;

    fn into_py_object(self, py: Python) -> Self::ObjectType {
        self.value().into_py_object(py)
    }

    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self.value().into_py_object(py)
    }
}

impl <'source> FromPyObject<'source> for StatType {
    fn extract(py: Python, obj: &'source PyObject) -> PyResult<Self> {
        match obj.extract(py) {
            Ok(n) => {
                match n {
                    1 => Ok(StatType::LastFight),
                    2 => Ok(StatType::Zone),
                    3 => Ok(StatType::Overall),
                    _ => Ok(StatType::Unknown)
                }
            },
            Err(e) => Err(e)
        }
    }
}


pub fn stats(py: Python, stat_type: StatType) -> PyResult<PyList> {
    if let Ok(ref mut py_meter) = METER.lock() {
        if let Some(m) = py_meter.get() {
            if let Ok(ref mut meter) = m.lock() {
                return Ok(core::stats(&meter, stat_type)
                    .into_iter()
                    .filter(|s| s.damage != 0.0 || s.fame != 0.0)
                    .collect::<Vec<meter::PlayerStatistics>>()
                    .into_py_object(py))
            }
        }
    }

    error!("Failed to acquire locks on meter");
    Ok(PyList::new(py, Vec::<PyObject>::new().as_slice()))
}

pub fn reset(_py: Python, stat_type: StatType) -> PyResult<u32> {
    if let Ok(ref mut py_meter) = METER.lock() {
        if let Some(m) = py_meter.get() {
            if let Ok(ref mut meter) = m.lock() {
                core::reset(meter, stat_type);
                return Ok(0);
            }
        }
    }

    error!("Failed to acquire locks on meter");
    Ok(1)
}

pub fn get_players_in_party(py: Python) -> PyResult<PyList> {
    if let Ok(ref mut py_meter) = METER.lock() {
        if let Some(m) = py_meter.get() {
            if let Ok(ref mut meter) = m.lock() {
                return Ok(core::get_players_in_party(&meter).into_py_object(py))
            }
        }
    }

    error!("Failed to acquire locks on meter");
    Ok(PyList::new(py, Vec::<PyObject>::new().as_slice()))
}

fn initialize(_py: Python, skip_non_party_members: bool) -> PyResult<u32> {
    if let Ok(ref mut py_meter) = METER.lock() {
        py_meter.initialize(core::initialize());
        if let Some(m) = py_meter.get() {
            if let Ok(ref mut meter) = m.lock() {
                meter.configure(core::MeterConfig {
                    skip_non_party_members,
                    ..Default::default()
                });
                return Ok(0);
            }
        }
    }
    error!("Failed to initialize meter");
    Ok(1)
}

py_module_initializer!(libaostats, initlibaostats, PyInit_libaostats, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize(skip_non_party_members: bool)))?;
    m.add(py, "stats", py_fn!(py, stats(stat_type: StatType)))?;
    m.add(py, "reset", py_fn!(py, reset(stat_type: StatType)))?;
    m.add(py, "get_players_in_party", py_fn!(py, get_players_in_party()))?;
    Ok(())
});

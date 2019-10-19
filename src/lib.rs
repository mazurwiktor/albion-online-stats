#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod game_protocol;
mod meter;

mod core;

use std::sync::{Mutex, Arc};

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

impl ToPyObject for core::PlayerStatistics {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let stats = PyDict::new(py);

        stats
            .set_item(py, "player", self.player.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "damage", self.damage.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "time_in_combat", self.time_in_combat.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "dps", self.dps.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "seconds_in_game", self.seconds_in_game.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame", self.fame.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame_per_minute", self.fame_per_minute.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame_per_hour", self.fame_per_hour.to_py_object(py))
            .unwrap();

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
    let meter = &mut METER.lock().unwrap();

    if let Some(m) = meter.get() {
        let meter = &mut *m.lock().unwrap();
        return Ok(core::stats(&meter, stat_type).into_py_object(py))
    }

    Ok(PyList::new(py, Vec::<PyObject>::new().as_slice()))
}

pub fn reset(_py: Python, stat_type: StatType) -> PyResult<u32> {
    let meter = &mut METER.lock().unwrap();

    if let Some(m) = meter.get() {
        let meter = &mut *m.lock().unwrap();
        core::reset(meter, stat_type);
    }

    Ok(0)
}

pub fn get_players_in_party(py: Python) -> PyResult<PyList> {
    let meter = &mut METER.lock().unwrap();

    if let Some(m) = meter.get() {
        let meter = &mut *m.lock().unwrap();
        return Ok(core::get_players_in_party(&meter).into_py_object(py))
    }

    Ok(PyList::new(py, Vec::<PyObject>::new().as_slice()))
}

fn initialize(_py: Python, skip_non_party_members: bool) -> PyResult<u32> {
    let meter = &mut METER.lock().unwrap();

    meter.initialize(core::initialize());

    if let Some(m) = meter.get() {
        let meter = &mut *m.lock().unwrap();
        meter.configure(core::MeterConfig {
            skip_non_party_members,
            ..Default::default()
        })
    }

    Ok(0)
}


py_module_initializer!(libmeter, initliblibmeter, PyInit_libmeter, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize(skip_non_party_members: bool)))?;
    m.add(py, "stats", py_fn!(py, stats(stat_type: StatType)))?;
    m.add(py, "reset", py_fn!(py, reset(stat_type: StatType)))?;
    m.add(py, "get_players_in_party", py_fn!(py, get_players_in_party()))?;
    Ok(())
});

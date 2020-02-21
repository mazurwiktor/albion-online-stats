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

use crate::core;
use crate::meter;
use crate::game::Event;

pub use crate::meter::StatType;

lazy_static! {
    static ref METER: Mutex<meter::Meter> = Mutex::new(meter::Meter::new());
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
        set_dict_item!(py, stats, self, items);

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


pub fn stats(py: Python, stat_type: StatType) -> PyResult<PyDict> {
    let stats = PyDict::new(py);
    if let Ok(ref mut meter) = &mut METER.lock() {
        let mut main : Option<core::PlayerStatistics> = None;
        stats.set_item(py, "players", core::stats(&meter, stat_type)
            .into_iter()
            .inspect(|s| {
                if s.main_player_stats {
                    main = Some(s.clone());
                }
            })
            .filter(|s| !s.idle || s.fame != 0.0)
            .collect::<Vec<meter::PlayerStatistics>>()
            .into_py_object(py)).ok();
        stats.set_item(py, "main", main.into_py_object(py)).ok();
        return Ok(stats);
    }

    error!("Failed to acquire locks on meter");
    Ok(stats)
}

pub fn reset(_py: Python, stat_type: StatType) -> PyResult<u32> {
    if let Ok(ref mut meter) = &mut METER.lock() {
        core::reset(meter, stat_type);
        return Ok(0);
    }

    error!("Failed to acquire locks on meter");
    Ok(1)
}

pub fn meter_subscriber(event: Event) {
    if let Ok(ref mut meter) = &mut METER.lock() {
        meter.consume(event); 
    }
}
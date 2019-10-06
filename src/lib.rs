#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod game_protocol;
mod meter;

mod core;

use crate::core::get_players_in_party;
use crate::core::initialize;
use crate::core::reset;
use crate::core::stats;
use crate::core::StatType;

py_module_initializer!(libmeter, initliblibmeter, PyInit_libmeter, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(
        py,
        "initialize",
        py_fn!(py, initialize(skip_non_party_members: bool)),
    )?;
    m.add(py, "stats", py_fn!(py, stats(stat_type: StatType)))?;
    m.add(py, "reset", py_fn!(py, reset(stat_type: StatType)))?;
    m.add(
        py,
        "get_players_in_party",
        py_fn!(py, get_players_in_party()),
    )?;
    Ok(())
});

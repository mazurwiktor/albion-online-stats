#[macro_use]
extern crate cpython;

#[macro_use]
extern crate lazy_static;

mod game_protocol;
mod meter;

mod core;

use crate::core::get_last_fight_session;
use crate::core::get_overall_session;
use crate::core::get_zone_session;
use crate::core::initialize;
use crate::core::new_last_fight_session;
use crate::core::new_zone_session;
use crate::core::reset_sessions;
use crate::core::get_players_in_party;

py_module_initializer!(libmeter, initliblibmeter, PyInit_libmeter, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize(skip_non_party_members: bool)))?;
    m.add(py, "new_zone_session", py_fn!(py, new_zone_session()))?;
    m.add(py, "get_overall_session", py_fn!(py, get_overall_session()))?;
    m.add(
        py,
        "get_last_fight_session",
        py_fn!(py, get_last_fight_session()),
    )?;
    m.add(py, "reset_sessions", py_fn!(py, reset_sessions()))?;
    m.add(
        py,
        "new_last_fight_session",
        py_fn!(py, new_last_fight_session()),
    )?;
    m.add(py, "get_zone_session", py_fn!(py, get_zone_session()))?;
    m.add(py, "get_players_in_party", py_fn!(py, get_players_in_party()))?;
    Ok(())
});

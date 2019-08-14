#[macro_use] extern crate cpython;
#[macro_use] extern crate lazy_static;
extern crate bytes;
extern crate packet_sniffer;
extern crate protocol16;

use std::fs::File;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::sync::Mutex;

use simplelog::*;
use log::*;
use cpython::{Python, PyResult, PyDict, PyList, PyString, PythonObject, PyObject, PyFloat};

use packet_sniffer::UdpPacket;

mod game_protocol;
mod meter;

lazy_static! {
    static ref METER: Mutex<meter::Meter> = Mutex::new(meter::Meter::new());
}

fn get_instance_session(py: Python) -> PyResult<PyList> {
    let meter = &mut METER.lock().unwrap();

    if let Some(session) = meter.get_instance_session() {
        let stats = session.iter().map(|s| {
            let dict = PyDict::new(py);

            dict.set_item(py, "player", PyString::new(py, &s.player)).unwrap();
            dict.set_item(py, "damage", PyFloat::new(py, s.damage.into())).unwrap();
            dict.set_item(py, "time_in_combat", PyFloat::new(py, s.time_in_combat.into())).unwrap();
            dict.set_item(py, "dps", PyFloat::new(py, s.dps.value().into())).unwrap();

            dict.into_object()
        }).collect::<Vec<PyObject>>();

        return Ok(PyList::new(py, stats.as_slice()));
    }

    return Ok(PyList::new(py, Vec::<PyObject>::new().as_slice()));

    // player: String,
    // damage: f32,
    // time_in_combat: f32,
    // dps: DPS
}

fn initialize(_py: Python) -> PyResult<u32> {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("damage-meter.log").unwrap(),
        ),
    ])
    .unwrap();

    thread::spawn(move || {
        let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();

        packet_sniffer::receive(tx);

        loop {
            if let Ok(packet) = rx.recv() {
                let meter = &mut METER.lock().unwrap();
                if packet.destination_port != 5056 && packet.source_port != 5056 {
                    continue;
                }
                let messages = game_protocol::decode(&packet.payload);

                for msg in messages {
                    debug!("Found message {:?}", msg);

                    match msg {
                        game_protocol::Message::Leave(msg) => meter.register_leave(msg.source).unwrap_or(()),
                        game_protocol::Message::NewCharacter(msg) => meter.register_player(&msg.character_name, msg.source),
                        game_protocol::Message::CharacterStats(msg) => meter.register_main_player(&msg.character_name, msg.source),
                        game_protocol::Message::HealthUpdate(msg) => meter.register_damage_dealt(msg.source, msg.value).unwrap_or(()),
                        game_protocol::Message::RegenerationHealthChanged(msg) => {
                            match msg.regeneration_rate {
                                Some(_) => meter.register_combat_leave(msg.source).unwrap_or(()),
                                None => meter.register_combat_enter(msg.source).unwrap_or(()) // TODO: handle death
                            }
                        }
                        _ => {}
                    }
                }
            }
        }        
    });

    Ok(0)
}

py_module_initializer!(libmeter, initliblibmeter, PyInit_libmeter, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize()))?;
    m.add(py, "get_instance_session", py_fn!(py, get_instance_session()))?;
    Ok(())
});

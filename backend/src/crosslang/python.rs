use cpython::PyObject;
use cpython::Python;
use cpython::ToPyObject;
use cpython::PythonObject;
use cpython::PyDict;

use crate::game::Event;
use crate::game::events;
use crate::game::StaticId;
use crate::photon_messages;

impl ToPyObject for Event {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let event = PyDict::new(py);

        match self {
            Event::MainPlayerAppeared(e) => {
                event.set_item(py, "name", "MainPlayerAppeared".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            },
            Event::PlayerAppeared(e) => {
                event.set_item(py, "name", "PlayerAppeared".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            },
            Event::ZoneChange => {
                event.set_item(py, "name", "ZoneChange".into_py_object(py)).unwrap_or(());
            },
            Event::UpdateItems(e) => {
                event.set_item(py, "name", "UpdateItems".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
            Event::DamageDone(e) => {
                event.set_item(py, "name", "DamageDone".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
            Event::EnterCombat(e) => {
                event.set_item(py, "name", "EnterCombat".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
            Event::LeaveCombat(e) => {
                event.set_item(py, "name", "LeaveCombat".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
            Event::UpdateFame(e) => {
                event.set_item(py, "name", "UpdateFame".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
            Event::HealthReceived(e) => {
                event.set_item(py, "name", "HealthReceived".into_py_object(py)).unwrap_or(());
                event.set_item(py, "value", e.into_py_object(py)).unwrap_or(());
            }
        }

        event.into_object()
    }
}

impl ToPyObject for StaticId {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self.inner().into_py_object(py).into_object()
    }
}


impl ToPyObject for events::Player {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let event = PyDict::new(py);

        event.set_item(py, "id", self.id.into_py_object(py)).unwrap_or(());
        event.set_item(py, "name", self.name.clone().into_py_object(py)).unwrap_or(());
        event.into_object()
    }
}

impl ToPyObject for events::Damage {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let event = PyDict::new(py);

        event.set_item(py, "source", self.source.into_py_object(py)).unwrap_or(());
        event.set_item(py, "target", self.target.into_py_object(py)).unwrap_or(());
        event.set_item(py, "value", self.value.into_py_object(py).into_object()).unwrap_or(());
        event.into_object()
    }
}

impl ToPyObject for events::Fame {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let event = PyDict::new(py);

        event.set_item(py, "source", self.source.into_py_object(py)).unwrap_or(());
        event.set_item(py, "value", self.value.into_py_object(py).into_object()).unwrap_or(());
        event.into_object()
    }
}

impl ToPyObject for events::Items {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let event = PyDict::new(py);

        event.set_item(py, "source", self.source.into_py_object(py)).unwrap_or(());
        event.set_item(py, "value", self.value.clone().into_py_object(py)).unwrap_or(());
        event.into_object()
    }
}

macro_rules! set_dict_item {
    ($py:ident, $dict:ident, $from:ident, $field_name:ident) => {
        if let Ok(_) = $dict.set_item($py, stringify!($field_name), $from.$field_name.to_py_object($py)) 
        {}
    };
}

impl ToPyObject for photon_messages::Items {
    type ObjectType = PyDict;

    fn into_py_object(self, py: Python) -> Self::ObjectType {
        self.to_py_object(py)
    }

    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let items = PyDict::new(py);

        set_dict_item!(py, items, self, weapon);
        set_dict_item!(py, items, self, offhand);
        set_dict_item!(py, items, self, helmet);
        set_dict_item!(py, items, self, armor);
        set_dict_item!(py, items, self, boots);
        set_dict_item!(py, items, self, bag);
        set_dict_item!(py, items, self, cape);
        set_dict_item!(py, items, self, mount);
        set_dict_item!(py, items, self, potion);
        set_dict_item!(py, items, self, food);
        
        items
    }
}

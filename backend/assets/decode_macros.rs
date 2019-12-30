macro_rules! decode_number {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Short(v) => Some(*v as usize),
                Value::Integer(v) => Some(*v as usize),
                Value::Byte(v) => Some(*v as usize),
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            error!("Index {} not found in {}", $index, $name);
            None
        }
    };
}

macro_rules! decode_string {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::String(v) => Some(v.clone()),
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

#[allow(unused_macros)]
macro_rules! decode_string_vec {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Array(arr) => {
                    let mut ret = vec![];
                    for v in arr {
                        if let Value::String(s) = v {
                            ret.push(s.clone());
                        }
                    }

                    Some(ret)
                }
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! decode_number_vec {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Array(arr) => {
                    let mut ret = vec![];
                    for v in arr {
                        match v {
                            Value::Short(v) => {
                                ret.push(*v as u32);
                            },
                            Value::Byte(v) => {
                                ret.push(*v as u32);
                            },
                            _ => {}
                        }
                    }

                    Some(ret)
                },
                Value::ByteArray(v) => {
                    Some(v.iter().map(|b| *b as u32).collect::<Vec<u32>>())
                },
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! decode_float {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Float(v) => Some(*v as f32),
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

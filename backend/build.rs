use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize};

#[derive(PartialEq, Clone, Deserialize)]
enum ParamType
{
    Number,
    Float,
    String,
    Items,
    StringList
}

impl ParamType {
    fn rust_type(&self) -> &'static str {
        match self {
            ParamType::Number => "usize",
            ParamType::Float => "f32",
            ParamType::String => "String",
            ParamType::Items => "Items",
            ParamType::StringList => "Vec<String>",
        }
    }

    fn parse_macro(&self) -> &'static str {
        match self {
            ParamType::Number => "decode_number!",
            ParamType::Float => "decode_float!",
            ParamType::String => "decode_string!",
            ParamType::Items => "decode_number_vec!",
            ParamType::StringList => "decode_string_vec!",
        }
    }
}

#[derive(Clone, Deserialize)]
struct Param
{
    name: String,
    id: u32,
    param_type: ParamType,
    optional: Option<bool>
}

#[derive(Clone, Deserialize)]
struct Message
{
    name: String,
    code: u32,
    params: Vec<Param>
}

#[derive(Clone, Deserialize)]
struct Messages
{
    events: Vec<Message>,
    responses: Vec<Message>,
}

fn main() {
    save_file("itemdb.rs", &generate_itemdb());
    save_file("messages.rs", &generate_messages());
}

fn generate_messages() -> String {
    let mut out = String::new();

    out.push_str(r"
        use log::*;
        use crate::game_protocol::Items;
        use photon_decode::Parameters;
        use photon_decode::Value;
    ");

    out.push_str(include_str!("assets/decode_macros.rs"));
    let messages: Messages = serde_json::from_str(include_str!("assets/messages.json")).expect("Missing assets/messages.json");

    for msg in &[&messages.events[..], &messages.responses[..]].concat() {
        let mut struct_params = String::new();
        for param in &msg.params {
            if param.optional.is_some() {
                struct_params.push_str(&format!("pub {}: Option<{}>,\n", param.name, param.param_type.rust_type()));
            } else {
                struct_params.push_str(&format!("pub {}: {},\n", param.name, param.param_type.rust_type()));
            }           
        }

        out.push_str(&format!(r###"
        #[derive(Debug)]
        pub struct {} {{
            {}
        }}
        "###, msg.name, struct_params));


        let mut parse_body = String::new();
        for param in &msg.params {
            if param.param_type == ParamType::Items {
                parse_body.push_str(&format!("let {} = {}(val, {}, \"{}::{0}\")?;\n", "item_array", param.param_type.parse_macro(), param.id, msg.name));
                parse_body.push_str(&format!("let items = Items::from(&item_array);\n"));
            } else {
                if param.optional.is_some() {
                    parse_body.push_str(&format!("let {} = {}(val, {}, \"{}::{0}\");\n", param.name, param.param_type.parse_macro(), param.id, msg.name));
                } else {
                    parse_body.push_str(&format!("let {} = {}(val, {}, \"{}::{0}\")?;\n", param.name, param.param_type.parse_macro(), param.id, msg.name));
                }
                
            }
            
        }
        let mut param_names = String::new();
        for param in &msg.params {
            param_names.push_str(&format!("{}, ", param.name));
        }

        out.push_str(&format!(r###"
        impl {0} {{
            pub fn parse(val: Parameters) -> Option<Message> {{
                info!("{0} parameters: {{:?}}", val);
                {1}
        
                Some(Message::{0}({0} {{ {2} }}))
            }}
        }}
        "###, msg.name, parse_body, param_names));
    }

    out.push_str("#[derive(Debug)]\n");
    out.push_str("pub enum Message {\n");
    for msg in &[&messages.events[..], &messages.responses[..]].concat() {
        out.push_str(&format!(r###"
                {0}({0}),"###, msg.name));
    }
    out.push_str("}");

    let mut event_matches = String::new();
    let mut responses_matches = String::new();

    for msg in &messages.events {
        event_matches.push_str(&format!("Some(photon_decode::Value::Short({})) => {}::parse(parameters),\n", msg.code, msg.name));
    }

    for msg in &messages.responses {
        responses_matches.push_str(&format!("Some(photon_decode::Value::Short({})) => {}::parse(parameters),\n", msg.code, msg.name));
    }

    out.push_str(&format!(r###"
    pub fn into_game_message(photon_message: photon_decode::Message) -> Option<Message> {{
        debug!("Raw photon : {{:?}}", photon_message);
        match photon_message {{
            photon_decode::Message::Event(photon_decode::EventData{{
                code: 1,
                parameters
            }}) => {{
                match parameters.get(&252u8) {{
                    {}
                    _ => None
                }}
            }},
            photon_decode::Message::Request(photon_decode::OperationRequest{{
                code: 1,
                parameters
            }}) => {{
                match parameters.get(&253u8) {{
                    _ => None
                }}
            }},
            photon_decode::Message::Response(photon_decode::OperationResponse{{
                code: 1,
                parameters,
                return_code: _,
                debug_message: _
            }}) => {{
                match parameters.get(&253u8) {{
                    {}
                    _ => None
                }}
            }},
            _ => None
        }}
    }}
    "###, event_matches, responses_matches));
    out
}

fn generate_itemdb() -> String {
    let mut out = String::new();

    out.push_str("use std::collections::HashMap;\n");
    out.push_str("lazy_static! {\n");
    out.push_str("pub static ref ITEMDB: HashMap<u32, &'static str> = {[\n");

    include_str!("assets/item_ids.txt").split('\n').filter_map(|line| {
        let v: Vec<&str> = line.split(',').collect();
        let id : u32 = v.get(0)?.parse().ok()?;
        let item  = v.get(1)?.to_owned();
        Some((id, item))
    }).for_each(|(id, item)| {
        out.push_str(&format!("({}, \"{}\"),\n", id, item))
    });
    out.push_str("].iter().cloned().collect()\n");
    out.push_str("};\n");
    out.push_str("}");

    out
}

fn save_file(file_name: &str, content: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(file_name);
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

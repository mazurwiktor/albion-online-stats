use std::io::Cursor;
use bytes::Buf;

use log::*;

static CMD_HEADER_LENGTH: u32 = 12;
static SIGNIFIER_BYTE_LENGTH: usize = 1;

static LOG_OUT: u8 = 4;
static SEND_UNRELIABLE: u8 = 7;
static SEND_RELIABLE: u8 = 6;

static REQUEST_CONSTANT: usize = 10000;
static RESPONSE_CONSTANT: usize = 1000;

static MSG_TYPE_REQUEST: u8 = 2;
static MSG_TYPE_RESPONSE: u8 = 3;
static MSG_TYPE_EVENT: u8 = 4;

use super::message::Message;
use super::packet::Packet;

pub fn decode(payload: &[u8]) -> Vec<Message>
{
    let mut messages = vec![];

    let mut cursor = Cursor::new(payload);
    cursor.advance(3);
    
    let cmd_cnt = photon_protocol::deserialize_byte(&mut cursor).unwrap();
    cursor.advance(8);

    for _ in 0..cmd_cnt {
        let cmd_type = photon_protocol::deserialize_byte(&mut cursor).unwrap();
        cursor.advance(3);
        let cmd_length = photon_protocol::deserialize_integer(&mut cursor).unwrap();
        cursor.advance(4);

        if cmd_type == LOG_OUT {
            break;
        } else if cmd_type == SEND_UNRELIABLE {
            cursor.advance(4);
            if let Some(msg) = on_message(&mut cursor, cmd_length - 4) {
                messages.push(msg);
            }
        } else if cmd_type == SEND_RELIABLE {
            if let Some(msg) = on_message(&mut cursor, cmd_length) {
                messages.push(msg);
            }

        } else {
            cursor.advance((cmd_length - CMD_HEADER_LENGTH) as usize);
        }
    }

    messages
}

fn on_message(cursor: &mut Cursor<&[u8]>, msg_len: u32)  -> Option<Message> {
    let mut message = None;

    let init = cursor.bytes().len();
    cursor.advance(SIGNIFIER_BYTE_LENGTH);

    let msg_type = photon_protocol::deserialize_byte(cursor).unwrap();
    let operation_length = msg_len - CMD_HEADER_LENGTH - 2;

    let mut payload = Cursor::new(&cursor.bytes()[0..operation_length as usize]);

    if msg_type == MSG_TYPE_EVENT {
        if let Some(event_data) = photon_protocol::deserialize_event_data(&mut payload) {
            if event_data.code != 2 && event_data.parameters.get(&252u8).is_some() {
                if let photon_protocol::Value::Short(code) = event_data.parameters.get(&252u8)? {
                    let packet = Packet{code: *code as usize, parameters: event_data.parameters};
                    debug!("EVENT: [{}] {:?}", packet.code, packet);
                    message = packet.decode();
                }
            }
        }
    } else if msg_type == MSG_TYPE_REQUEST {
        if let Some(request) = photon_protocol::deserialize_operation_request(&mut payload) {
            let code = request.code as usize + REQUEST_CONSTANT;
            let packet = Packet{code, parameters: request.parameters};
            debug!("REQUEST: [{}] {:?}", packet.code, packet);

            message = packet.decode();
        }
    } else if msg_type == MSG_TYPE_RESPONSE {
        if let Some(response) = photon_protocol::deserialize_operation_response(&mut payload) {
            let code = response.code as usize + RESPONSE_CONSTANT;
            let packet = Packet{code, parameters: response.parameters};
            debug!("RESPONSE: [{}] {:?}", packet.code, packet);

            message = packet.decode();
        }
    }

    cursor.advance(operation_length as usize);
    let last = cursor.bytes().len();
    assert!(init - last == msg_len as usize - CMD_HEADER_LENGTH as usize);

    message
}
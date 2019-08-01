use std::io::Cursor;
use bytes::Buf;

static CMD_HEADER_LENGTH: u32 = 12;
static SIGNIFIER_BYTE_LENGTH: usize = 1;

static LOG_OUT: u8 = 4;
static SEND_UNRELIABLE: u8 = 7;
static SEND_RELIABLE: u8 = 6;

static MSG_TYPE_REQUEST: u8 = 2;
static MSG_TYPE_RESPONSE: u8 = 3;
static MSG_TYPE_EVENT: u8 = 4;


pub fn decode(payload: &[u8])
{
    let mut cursor = Cursor::new(payload);
    cursor.advance(3);
    
    let cmd_cnt = protocol16::deserialize_byte(&mut cursor).unwrap();
    cursor.advance(8);

    for _ in 0..cmd_cnt {
        let cmd_type = protocol16::deserialize_byte(&mut cursor).unwrap();
        cursor.advance(3);
        let cmd_length = protocol16::deserialize_integer(&mut cursor).unwrap();
        cursor.advance(4);

        if cmd_type == LOG_OUT {
            break;
        } else if cmd_type == SEND_UNRELIABLE {
            cursor.advance(4);
            on_message(&mut cursor, cmd_length - 4);
        } else if cmd_type == SEND_RELIABLE {
            on_message(&mut cursor, cmd_length);
        } else {
            cursor.advance((cmd_length - CMD_HEADER_LENGTH) as usize);
        }
    }
}

fn on_message(cursor: &mut Cursor<&[u8]>, msg_len: u32) {
    let init = cursor.bytes().len();
    cursor.advance(SIGNIFIER_BYTE_LENGTH);

    let msg_type = protocol16::deserialize_byte(cursor).unwrap();
    let operation_length = msg_len - CMD_HEADER_LENGTH - 2;

    let mut payload = Cursor::new(&cursor.bytes()[0..operation_length as usize]);

    if msg_type == MSG_TYPE_EVENT {
        if let Some(event_data) = protocol16::deserialize_event_data(&mut payload) {
            if event_data.code != 2 {
                println!("{:?}", event_data);
            }
        }
    }

    cursor.advance(operation_length as usize);
    let last = cursor.bytes().len();
    assert!(init - last == msg_len as usize - CMD_HEADER_LENGTH as usize);
}
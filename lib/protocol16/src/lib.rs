// import io
// import struct

// from collections import namedtuple

// from . import type_codes

// EventData = namedtuple("EventData", ['Code', 'Parameters'])
// OperationResponse = namedtuple("OperationResponse", ['OperationCode', 'ReturnCode', 'DebugMessage', 'Parameters'])
// OperationRequest = namedtuple('OperationRequest', ['OperationCode', 'Parameters'])


// def deserialize_byte(byte_stream):
//     return struct.unpack("B", byte_stream.read(1))[0]


// def deserialize_double(byte_stream):
//     return struct.unpack(">d", byte_stream.read(8))[0]

// def deserialize_float(byte_stream):
//     return round(struct.unpack(">f", byte_stream.read(4))[0], 2)


// def deserialize_integer(byte_stream):
//     return struct.unpack(">I", byte_stream.read(4))[0]


// def deserialize_short(byte_stream):
//     return struct.unpack(">h", byte_stream.read(2))[0]


// def deserialize_long(byte_stream):
//     return struct.unpack(">q", byte_stream.read(8))[0]


// def deserialize_dictionary(byte_stream):
//     key_type_code = deserialize_byte(byte_stream)
//     value_type_code = deserialize_byte(byte_stream)
//     dict_size = deserialize_short(byte_stream)

//     result = {}

//     for _ in range(0, dict_size):
//         key = deserialize(
//             byte_stream, deserialize_byte(byte_stream) if (
//                 key_type_code == 0 or key_type_code == 42) else key_type_code)
//         value = deserialize(
//             byte_stream, deserialize_byte(byte_stream) if (
//                 value_type_code == 0 or value_type_code == 42) else value_type_code)
//         result[key] = value

//     return result


// def deserialize_string(byte_stream):
//     size = deserialize_short(byte_stream)
//     if not size:
//         return ""
    
//     return byte_stream.read(size).decode('utf-8', errors='replace')


// def deserialize_string_array(byte_stream):
//     size = deserialize_short(byte_stream)
    
//     result = []
    
//     if not size:
//         return result
    
//     for _ in range(0, size):
//         result.append(deserialize_string(byte_stream))
    
//     return result


// def deserialize_parameter_table(byte_stream):
//     size = deserialize_short(byte_stream)

//     result = {}

//     for _ in range(0, size):
//         key = deserialize_byte(byte_stream)
//         value_type_code = deserialize_byte(byte_stream)
//         value = deserialize(byte_stream, value_type_code)

//         result[key] = value

//     return result


// def deserialize_event_data(byte_stream):
//     code = deserialize_byte(byte_stream)
    
//     return EventData(code, deserialize_parameter_table(byte_stream))


// def deserialize_integer_array(byte_stream):
//     return None


// def deserialize_boolean(byte_stream):
//     return struct.unpack(">?", byte_stream.read(1))[0]


// def deserialize_operation_response(byte_stream):
//     operation_code = deserialize_byte(byte_stream)
//     return_code = deserialize_short(byte_stream)
//     debug_message = deserialize(byte_stream, deserialize_byte(byte_stream))
//     parameters = deserialize_parameter_table(byte_stream)

//     return OperationResponse(operation_code, return_code, debug_message, parameters)


// def deserialize_operation_request(byte_stream):
//     operation_code = deserialize_byte(byte_stream)
//     parameters = deserialize_parameter_table(byte_stream)

//     return OperationRequest(operation_code, parameters)


// def deserialize_byte_array(byte_stream):
//     size = deserialize_integer(byte_stream)
//     if not size:
//         return []
    
//     return list(byte_stream.read(size))


// def deserialize_object_array(byte_stream):
//     size = deserialize_short(byte_stream)
//     result = []
    
//     if not size:
//         return result
    
//     for _ in range(0, size):
//         typed_code = deserialize_byte(byte_stream)
//         result.append(deserialize(byte_stream, typed_code))
    
//     return result


// def deserialize_array(byte_stream):
//     size = deserialize_short(byte_stream)
//     result = []
    
//     if not size:
//         return result
    
//     typed_code = deserialize_byte(byte_stream)

//     if typed_code == type_codes.BYTE_ARRAY:
//         return deserialize_byte_array(byte_stream)

//     if typed_code == type_codes.STRING_ARRAY:
//         return deserialize_string_array(byte_stream)

//     for _ in range(0, size):
//         result.append(deserialize(byte_stream, typed_code))

//     return result


// deserializers = {
//     0: lambda _ : None,
//     42: lambda _ : None,
//     type_codes.DICTIONARY:  deserialize_dictionary,
//     type_codes.STRING_ARRAY:  deserialize_string_array,
//     type_codes.BYTE:  deserialize_byte,
//     type_codes.DOUBLE: deserialize_double,
//     type_codes.EVENT_DATA: deserialize_event_data,
//     type_codes.FLOAT: deserialize_float,
//     type_codes.INTEGER: deserialize_integer,
//     type_codes.SHORT: deserialize_short,
//     type_codes.LONG: deserialize_long,
//     type_codes.INTEGER_ARRAY: deserialize_integer_array,
//     type_codes.BOOLEAN: deserialize_boolean,
//     type_codes.OPERATION_RESPONSE: deserialize_operation_response,
//     type_codes.OPERATION_REQUEST: deserialize_operation_request,
//     type_codes.STRING: deserialize_string,
//     type_codes.BYTE_ARRAY: deserialize_byte_array,
//     type_codes.ARRAY: deserialize_array,
//     type_codes.OBJECT_ARRAY: deserialize_object_array
// }

// def deserialize(byte_stream, typed_code):
//     if typed_code in deserializers:
//         return deserializers[typed_code](byte_stream)
    
//     return None

#[cfg(test)]
mod tests
{
// def test_deserialize_dictionary():
//     byte_stream = io.BytesIO(bytes([68, 115, 115, 0, 2, 0, 8, 116, 101, 115, 116, 75, 101, 121, 49, 0, 10, 116, 101, 115, 116, 86, 97, 108, 117, 101, 49, 0, 8, 116, 101, 115, 116, 75, 101, 121, 50, 0, 10, 116, 101, 115, 116, 86, 97, 108, 117, 101, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result["testKey1"] == "testValue1"
//     assert result["testKey2"] == "testValue2"


// def test_deserialize_string_array():
//     byte_stream = io.BytesIO(bytes([121, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0] == "test1"
//     assert result[1] == "test2"


// def test_deserialize_byte():
//     byte_stream = io.BytesIO(bytes([98, 6]))
//     typed_code = struct.unpack("B", byte_stream.read(1))[0]

//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 6


// def test_deserialize_double():
//     byte_stream = io.BytesIO(bytes([100, 64, 147, 74, 51, 51, 51, 51, 51, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 1234.55


// def test_deserialize_event_data():
//     byte_stream = io.BytesIO(bytes([101, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result.Code == 100
//     assert result.Parameters[0] == "test1"
//     assert result.Parameters[1] == "test2"


// def test_deserialize_float():
//     byte_stream = io.BytesIO(bytes([102, 68, 154, 81, 154, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 1234.55


// def test_deserialize_integer():
//     byte_stream = io.BytesIO(bytes([105, 0, 0, 4, 210, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 1234


// def test_deserialize_short():
//     byte_stream = io.BytesIO(bytes([107, 4, 210, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 1234


// def test_deserialize_long():
//     byte_stream = io.BytesIO(bytes([108, 0, 0, 0, 0, 0, 0, 4, 210, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == 1234


// def test_deserialize_integer_array():
//     byte_stream = io.BytesIO(bytes([121, 0, 2, 105, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0] == 0
//     assert result[1] == 1


// def test_deserialize_boolean():
//     byte_stream = io.BytesIO(bytes([111, 1]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == True


// def test_deserialize_operation_response():
//     byte_stream = io.BytesIO(bytes([112, 100, 0, 100, 42, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result.OperationCode == 100
//     assert result.ReturnCode == 100
//     assert result.Parameters[0] == "test1"
//     assert result.Parameters[1] == "test2"


// def test_deserialize_operation_request():
//     byte_stream = io.BytesIO(bytes([113, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result.OperationCode == 100
//     assert result.Parameters[0] == "test1"
//     assert result.Parameters[1] == "test2"


// def test_deserialize_string():
//     byte_stream = io.BytesIO(bytes([115, 0, 12, 116, 101, 115, 116, 95, 109, 101, 115, 115, 97, 103, 101, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result == "test_message"


// def test_deserialize_byte_array():
//     byte_stream = io.BytesIO(bytes([120, 0, 0, 0, 2, 6, 7, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0] == 6
//     assert result[1] == 7


// def test_deserialize_array_dictionary():
//     byte_stream = io.BytesIO(bytes([121, 0, 1, 68, 105, 115, 0, 1, 0, 0, 0, 0, 0, 5, 116, 101, 115, 116, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0][0] == "test1"


// def test_deserialize_array_byte_array():
//     byte_stream = io.BytesIO(bytes([121, 0, 1, 120, 0, 0, 0, 4, 0, 2, 4, 8, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0] == 0
//     assert result[1] == 2
//     assert result[2] == 4
//     assert result[3] == 8


// def test_deserialize_array_array():
//     byte_stream = io.BytesIO(bytes([121, 0, 1, 121, 0, 3, 105, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0][0] == 1
//     assert result[0][1] == 2
//     assert result[0][2] == 3


// def test_deserialize_object_array():
//     byte_stream = io.BytesIO(bytes([122, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))

//     typed_code = struct.unpack("B", byte_stream.read(1))[0]
//     result = deseliarizer.deserialize(byte_stream, typed_code)

//     assert result
//     assert result[0] == "test1"
//     assert result[1] == "test2"


}
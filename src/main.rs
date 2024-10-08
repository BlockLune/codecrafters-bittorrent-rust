use serde_bencode;
use serde_json::{self, Value as JsonValue};
use std::env;
use std::str;

fn bencode_to_json(value: &serde_bencode::value::Value) -> Result<JsonValue, String> {
    match value {
        serde_bencode::value::Value::Bytes(bytes) => match str::from_utf8(&bytes) {
            Ok(s) => Ok(JsonValue::String(s.to_string())),
            Err(e) => Err(format!("Error converting bytes: {}", e)),
        },
        serde_bencode::value::Value::Int(integer) => {
            Ok(JsonValue::Number(serde_json::Number::from(*integer)))
        }
        serde_bencode::value::Value::List(list) => {
            let json_list: Result<Vec<_>, _> = list.iter().map(bencode_to_json).collect();
            json_list.map(JsonValue::Array)
        }
        serde_bencode::value::Value::Dict(dict) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in dict {
                match str::from_utf8(&key) {
                    Ok(key_str) => {
                        let json_value = bencode_to_json(value)?;
                        json_map.insert(key_str.to_string(), json_value);
                    }
                    Err(e) => return Err(format!("Error converting key bytes: {}", e)),
                }
            }
            Ok(JsonValue::Object(json_map))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        match serde_bencode::from_str::<serde_bencode::value::Value>(&encoded_value) {
            Ok(value) => match bencode_to_json(&value) {
                Ok(json_value) => {
                    println!("{}", json_value)
                }
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}

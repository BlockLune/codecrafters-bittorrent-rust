use serde_json;
use std::env;

fn decode_bencoded_integer(encoded_value: &str) -> Result<(serde_json::Value, usize), String> {
    let err_message = format!("Failed to parse an integer from {}", encoded_value);

    let begin_index = encoded_value.find("i").ok_or_else(|| err_message.clone())?;
    if begin_index != 0 {
        return Err(err_message.clone());
    }

    let end_index = encoded_value.find('e').ok_or_else(|| err_message.clone())?;
    let int_str = &encoded_value[1..end_index];

    int_str
        .parse::<i64>()
        .map(|integer| {
            (
                serde_json::Value::Number(serde_json::Number::from(integer)),
                end_index,
            )
        })
        .map_err(|_| err_message)
}

fn decode_bencoded_string(encoded_value: &str) -> Result<(serde_json::Value, usize), String> {
    let err_message = format!("Failed to parse a string from {}", encoded_value);

    if !encoded_value.chars().next().unwrap().is_digit(10) {
        return Err(err_message.clone());
    }

    let colon_index = encoded_value.find(':').ok_or_else(|| err_message.clone())?;

    let number_str = &encoded_value[..colon_index];
    let number = number_str.parse::<i64>().map_err(|_| err_message.clone())?;
    let end_index = colon_index + 1 + number as usize;
    let string = &encoded_value[colon_index + 1..end_index];
    Ok((serde_json::Value::String(string.to_string()), end_index))
}

fn decode_bencoded_list(encoded_value: &str) -> Result<(serde_json::Value, usize), String> {
    let err_message = format!("Failed to parse a list from {}", encoded_value);

    if encoded_value.chars().next().unwrap() != 'l' {
        return Err(err_message.clone());
    }

    todo!()
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.chars().next().unwrap().is_digit(10) {
        return decode_bencoded_string(encoded_value).unwrap().0;
    } else if encoded_value.starts_with("i") {
        return decode_bencoded_integer(encoded_value).unwrap().0;
    } else if encoded_value.starts_with("l") {
    }
    panic!("Unhandled encoded value: {}", encoded_value)
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

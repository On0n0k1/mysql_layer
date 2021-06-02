use serde::{Serialize, Deserialize};

// use wasm_bindgen::prelude::*;
// use js_sys::{Object, Reflect, Promise};
// use wasm_bindgen::prelude::*;
// use web_sys::console;
use base64::decode;

use crate::lambda::{
    message::Message,
};

/// Allowing snake case and camel case warnings so the serialize parse aws events.
#[derive(Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub struct requestContext{
    pub http: http,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub struct http{
    pub method: String,
    pub path: String,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types, non_snake_case)]
pub struct event{
    pub version: String,
    pub requestContext: requestContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub isBase64Encoded: bool,
}

// Change name to LambdaEvent later
#[derive(Serialize, Deserialize, Clone)]
pub struct Full{
    pub event: event,
}

impl Full{
    pub fn new(message: String) -> Result<Self, String> {
        let message = Message::new_json(&message[..]);

        let full = message.get_value::<Full>();
        let full = match full{
            None => {
                let message = format!("Failed at deserializing body(Full message). \n");
                println!("{}", message);
                // console::log_1(&(format!("Failed at deserializing body(Full message). \n"))[..].into());
                return Err(format!("{{ \"body\": \"{}\" }}", message));
            },
            Some(mut value) => {
                let body = value.event.body.clone();
                match body {
                    None => value,
                    Some(body) => {
                        if value.event.isBase64Encoded {
                            let decoded_body = Self::decode_body(body)?;
                            value.event.body = Some(format!("{}", decoded_body));
                            value.event.isBase64Encoded = false;
                            value
                        } else {
                            value
                        }
                    }
                }
            }
        };
        Ok(full)
    }

    pub fn decode_body(body: String) -> Result<String, String> {
        println!("Starting decoding");
        // console::log_1(&String::from("Starting decoding").into());
        let value = &decode(&body[..]);
        match value {
            Err(err) => {
                println!("Failed at decoding body(u8). err = {}\n", err);
                // console::log_1(&(format!("Failed at decoding body(u8). err = {}\n", err))[..].into());
                return Err(String::from("Failed at decoding body(u8)"));
            },
            Ok(value) => {
                let value = std::str::from_utf8(value);
                match value{
                    Err(err) => {
                        println!("Failed at decoding body(utf8). err = {}\n", err);
                        // console::log_1(&(format!("Failed at decoding body(utf8). err = {}\n", err))[..].into());
                        return Err(String::from("Failed at decoding body(utf8)"));
                    },
                    Ok(value)=> {
                        return Ok(String::from(value))
                    }

                }
            }
        }
    }
}
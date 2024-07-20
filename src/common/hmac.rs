use hmac::{Hmac, Mac};
use itertools::Itertools;
use serde_json::Value;
use sha2::Sha256;
use std::collections::HashMap;

const SIGN_KEY: &[u8] = b"8844b676f3268c082a56021d9f47a206";

pub fn sign_data<T>(data: &T) -> String
where
    T: serde::ser::Serialize,
{
    // data.Serialize();
    let t = serde_json::to_value(&data).unwrap();
    let t1: HashMap<String, Value> = serde_json::from_value(t).unwrap();
    let t2: HashMap<String, String> = t1
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                },
            )
        })
        .collect();

    let mut keys = t2.keys().sorted();
    let mut s = match keys.next() {
        Some(key) => format!("{}={}", key, t2[key]),
        None => return String::new(),
    };

    for key in keys {
        s.push_str(format!("&{}={}", key, t2[key]).as_str());
    }

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(SIGN_KEY).unwrap();
    mac.update(s.as_bytes());
    let result = mac.finalize();
    return hex::encode(result.into_bytes());
}

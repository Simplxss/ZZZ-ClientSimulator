use std::collections::HashMap;

const SDK_DOMAIN: &str = "https://nap-sdk.mihoyo.com/";

#[derive(serde::Deserialize)]
struct Response<T> {
    retcode: i32,
    message: String,
    data: T,
}

#[derive(serde::Deserialize)]
struct FetchQrcode {
    pub url: String,
}

pub fn fetch_qrcode() -> Result<String, String> {
    let mut data = HashMap::new();
    data.insert("app_id", "7".to_string());
    data.insert("device", "device_id".to_string());
    let res = reqwest::blocking::Client::new()
        .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/panda/qrcode/fetch").as_str())
        .form(&data)
        .send()
        .expect("Failed to send request");

    let json = serde_json::from_str::<Response<FetchQrcode>>(
        res.text().expect("Decode request text error").as_str(),
    )
    .expect("Parsing JSON failed");

    if json.retcode != 0 {
        return Err(format!("Failed to get regions, retcode: {}", json.retcode));
    }

    return Ok(json.data.url);
}

#[derive(serde::Deserialize)]
pub struct Raw {
    pub uid: i32,
    pub token: String,
}

#[derive(serde::Deserialize)]
pub struct Payload {
    pub raw: Raw,
}

#[derive(serde::Deserialize)]
pub struct QueryQrcodeStatus {
    pub stat: String,
    pub payload: Payload,
}

pub fn query_qrcode_status(ticket: &str) -> Result<QueryQrcodeStatus, String> {
    let mut data = HashMap::new();
    data.insert("app_id", "7".to_string());
    data.insert("device", "device_id".to_string());
    data.insert("ticket", ticket.to_string());
    let res = reqwest::blocking::Client::new()
        .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/panda/qrcode/query").as_str())
        .form(&data)
        .send()
        .expect("Failed to send request");

    let json = serde_json::from_str::<Response<QueryQrcodeStatus>>(
        res.text().expect("Decode request text error").as_str(),
    )
    .expect("Parsing JSON failed");

    if json.retcode != 0 {
        return Err(format!("Failed to get regions, retcode: {}", json.retcode));
    }

    return Ok(json.data);
}

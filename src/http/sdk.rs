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
    let mut headers = reqwest::header::HeaderMap::new();
    // "x-rpc-channel_id": "1",
    // "x-rpc-channel_version": "2.24.0.94",
    // 'x-rpc-client_type': '9',
    // "x-rpc-device_fp": device['pc'].device_fp,
    // "x-rpc-device_id": device['pc'].device_id,
    // "x-rpc-device_model": device['pc'].device_model,
    // 'x-rpc-device_name': device['pc'].device_name,
    // "x-rpc-game_biz": "hk4e_cn",
    // "x-rpc-language": "zh-cn",
    // "x-rpc-lifecycle_id": device['pc'].lifecycle_id,
    // 'x-rpc-mdk_version': '2.24.0.94',
    // 'x-rpc-sdk_version': "2.24.0.94",
    // 'x-rpc-sys_version': 'Windows 10',
    // "Content-Type": "application/json"
    headers.insert(
        "x-rpc-channel_id",
        reqwest::header::HeaderValue::from_static("1")
    );
    headers.insert(
        "x-rpc-channel_version",
        reqwest::header::HeaderValue::from_static("2.24.0.94")
    );
    headers.insert(
        "x-rpc-client_type",
        reqwest::header::HeaderValue::from_static("9")
    );

    let mut data = HashMap::new();
    data.insert("app_id", "7".to_string());
    data.insert("device", "device_id".to_string());

    let res = reqwest::blocking::Client::new()
        .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/panda/qrcode/fetch").as_str())
        .headers(headers)
        .form(&data)
        .send()
        .expect("Failed to send request");

    let json = res
        .json::<Response<FetchQrcode>>()
        .expect("Parsing JSON failed");

    if json.retcode != 0 {
        return Err(format!("Failed to fetch qrcode, retcode: {}, message: {}", json.retcode, json.message));
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

    let json = res
        .json::<Response<QueryQrcodeStatus>>()
        .expect("Parsing JSON failed");

    if json.retcode != 0 {
        return Err(format!("Failed to query qrcode, retcode: {}, message: {}", json.retcode, json.message));
    }

    return Ok(json.data);
}

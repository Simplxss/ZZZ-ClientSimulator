#![allow(dead_code)]

use rand::Rng;
use regex::Regex;
use std::{thread::sleep, time::Duration};

const SDK_DOMAIN: &str = "https://nap-sdk.mihoyo.com/";
const GAME_API_DOMAIN: &str = "https://gameapi-account.mihoyo.com/";

struct Device {
    device_fp: String,
    device_id: String,
    device_model: String,
    device_name: String,
    lifecycle_id: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AccountData {
    uid: String,
    token: String,
}

pub struct Sdk {
    device: Device,
    headers: reqwest::header::HeaderMap,

    account: Option<AccountData>,

    // callback function
    qrcode_callback: Option<fn(&str) -> ()>,
    captcha_callback: Option<fn() -> String>,
    geetest_callback: Option<fn(&str, &str) -> (String, String)>,
}

#[derive(serde::Deserialize)]
struct Response<T> {
    retcode: i32,
    message: String,
    data: Option<T>,
}

#[derive(serde::Serialize)]
struct FetchQrcodeRequest {
    app_id: String,
    device: String,
}

#[derive(serde::Deserialize)]
struct FetchQrcodeResponse {
    url: String,
}

#[derive(serde::Deserialize)]
struct Raw {
    token: String,
    is_bbs: String,
    uid: String,
    mid: String,
    is_v2_token: bool,
}

#[derive(serde::Deserialize)]
struct Payload {
    proto: String,
    raw: String,
    ext: String,
}

#[derive(serde::Serialize)]
struct QueryQrcodeStatusRequest {
    app_id: String,
    device: String,
    ticket: String,
}

#[derive(serde::Deserialize)]
struct QueryQrcodeStatusResponse {
    stat: String,
    payload: Payload,
}

#[derive(serde::Deserialize)]
struct Geetest {
    challenge: String,
    gt: String,
    new_captcha: i32,
    success: i32,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct CheckRiskyRequest {
    action_type: String,
    api_name: String,
    username: Option<String>,
    mobile: Option<String>,
}

#[derive(serde::Deserialize)]
struct CheckRiskyResponse {
    id: String,
    action: String,
    geetest: Option<Geetest>,
}

#[derive(serde::Serialize)]
struct PasswordLoginRequest {
    account: String,
    password: String,
    is_crypto: bool,
}

#[derive(serde::Deserialize)]
struct PasswordLoginResponse {
    account: Account,
    device_grant_required: bool,
    safe_moblie_required: bool,
    realperson_required: bool,
    reactivate_required: bool,
    realname_operation: String,
}

#[derive(serde::Serialize)]
struct SendCaptchaRequest {
    mobile: String,
    area: String,
}

#[derive(serde::Deserialize)]
struct SendCaptchaResponse {
    action: String,
}

#[derive(serde::Serialize)]
struct SubmitCaptchaRequest {
    mobile: String,
    captcha: String,
    action: String,
    area: String,
}

#[derive(serde::Deserialize)]
struct Account {
    uid: String,
    name: String,
    email: String,
    mobile: String,
    is_email_verify: String,
    realname: String,
    identity_card: String,
    token: String, // game_token
    safe_mobile: String,
    facebook_name: String,
    google_name: String,
    twitter_name: String,
    game_center_name: String,
    apple_name: String,
    sony_name: String,
    tap_name: String,
    country: String,
    reactivate_ticket: String,
    area_code: String,
    device_grant_ticket: String,
    steam_name: String,
    unmasked_email: String,
    unmasked_email_type: i32,
    cx_name: String,
}

#[derive(serde::Deserialize)]
struct SubmitCaptchaResponse {
    account: Account,
    realperson_required: bool,
    realname_operation: String,
    reactivate_required: bool,
}

#[derive(serde::Serialize)]
struct AuthData {
    uid: String,
    guest: bool,
    token: String,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct LoginGameRequest {
    data: String,
    app_id: i32,
    channel_id: i32,
    device: String,
    sign: Option<String>,
}

#[derive(serde::Deserialize)]
struct LoginGameResponse {
    combo_id: String,
    open_id: String,
    combo_token: String,
    data: String,
    heartbeat: bool,
    account_type: i32,
    fatigue_remind: Option<()>,
}

fn rand_hex_string<R: Rng + ?Sized>(rng: &mut R, len: usize) -> String {
    const HEX_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut s = String::new();
    for _ in 0..len {
        s.push(HEX_CHARS[rng.gen_range(0..16)]);
    }
    return s;
}

impl Sdk {
    pub fn new(
        qrcode_callback: Option<fn(&str)>,
        captcha_callback: Option<fn() -> String>,
        geetest_callback: Option<fn(&str, &str) -> (String, String)>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let device = Device {
            device_fp: rand_hex_string(&mut rng, 13),
            device_id: rand_hex_string(&mut rng, 53),
            device_model: "iMac".to_string(),
            device_name: "Simulator".to_string(),
            lifecycle_id: uuid::Uuid::new_v4().to_string(),
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "UnityPlayer/2019.4.40f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)",
            ),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("*/*"),
        );
        headers.insert(
            reqwest::header::ACCEPT_ENCODING,
            reqwest::header::HeaderValue::from_static("deflate, gzip"),
        );
        headers.insert(
            "x-rpc-client_type",
            reqwest::header::HeaderValue::from_static("3"),
        );
        headers.insert(
            "x-rpc-sys_version",
            reqwest::header::HeaderValue::from_static("Windows 10"),
        );
        headers.insert(
            "x-rpc-device_id",
            reqwest::header::HeaderValue::from_str(&device.device_id).unwrap(),
        );
        headers.insert(
            "x-rpc-device_model",
            reqwest::header::HeaderValue::from_str(&device.device_model).unwrap(),
        );
        headers.insert(
            "x-rpc-device_name",
            reqwest::header::HeaderValue::from_str(&device.device_name).unwrap(),
        );
        headers.insert(
            "x-rpc-mdk_version",
            reqwest::header::HeaderValue::from_static("2.23.0.0"),
        );
        headers.insert(
            "x-rpc-channel_version",
            reqwest::header::HeaderValue::from_static("2.23.0.0"),
        );
        headers.insert(
            "x-rpc-channel_id",
            reqwest::header::HeaderValue::from_static("1"),
        );
        headers.insert(
            "x-rpc-sub_channel_id",
            reqwest::header::HeaderValue::from_static("1"),
        );
        headers.insert(
            "x-rpc-language",
            reqwest::header::HeaderValue::from_static("zh-cn"),
        );
        headers.insert(
            "x-rpc-game_biz",
            reqwest::header::HeaderValue::from_static("nap_cn"),
        );
        headers.insert(
            "x-rpc-combo_version",
            reqwest::header::HeaderValue::from_static("2.23.0"),
        );
        headers.insert(
            "x-rpc-payment_version",
            reqwest::header::HeaderValue::from_static("2.23.0"),
        );
        headers.insert(
            "x-rpc-goods_third_party",
            reqwest::header::HeaderValue::from_static("unsupported"),
        );
        headers.insert(
            "x-rpc-device_fp",
            reqwest::header::HeaderValue::from_str(&device.device_fp).unwrap(),
        );
        headers.insert(
            "x-rpc-lifecycle_id",
            reqwest::header::HeaderValue::from_str(&device.lifecycle_id).unwrap(),
        );
        headers.insert(
            "X-Unity-Version",
            reqwest::header::HeaderValue::from_static("2019.4.40f1"),
        );

        Sdk {
            device,
            headers,
            account: Option::None,
            qrcode_callback,
            captcha_callback,
            geetest_callback,
        }
    }

    async fn fetch_qrcode(&self) -> Result<String, String> {
        let data = FetchQrcodeRequest {
            app_id: "12".to_string(),
            device: self.device.device_id.clone(),
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/panda/qrcode/fetch").as_str())
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<FetchQrcodeResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to fetch qrcode, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        return Ok(json.data.unwrap().url);
    }

    async fn query_qrcode_status(&self, ticket: &str) -> Result<QueryQrcodeStatusResponse, String> {
        let data = QueryQrcodeStatusRequest {
            app_id: "12".to_string(),
            device: self.device.device_id.clone(),
            ticket: ticket.to_string(),
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/panda/qrcode/query").as_str())
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<QueryQrcodeStatusResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to query qrcode, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        return Ok(json.data.unwrap());
    }

    pub async fn qr_login(&mut self) -> Result<(), String> {
        let qrcode_url = self.fetch_qrcode().await.expect("Failed to fetch qrcode");
        (self.qrcode_callback.expect("qrcode callback is null"))(&qrcode_url);

        let re = Regex::new(r"&ticket=([^&]*)").unwrap();
        let ticket = re.captures(&qrcode_url).unwrap().get(1).unwrap().as_str();

        loop {
            let result = self
                .query_qrcode_status(ticket)
                .await
                .expect("Failed to check qrcode status");
            match result.stat.as_str() {
                "Init" => {}
                "Scanned" => {}
                "Confirmed" => {
                    let raw = serde_json::from_str::<Raw>(&result.payload.raw).unwrap();
                    self.account = Option::Some(AccountData {
                        uid: raw.uid,
                        token: raw.token, //stoken
                    });
                    return Ok(());
                }
                _ => return Err(format!("Unknown status: {}", result.stat)),
            }
            sleep(Duration::from_secs(1));
        }
    }

    async fn check_risky(
        &self,
        action_type: &str,
        api_name: &str,
        username: Option<&str>,
        mobile: Option<&str>,
    ) -> Result<String, String> {
        let data = CheckRiskyRequest {
            action_type: action_type.to_string(),
            api_name: api_name.to_string(),
            username: username.and_then(|s| Option::Some(s.to_string())),
            mobile: mobile.and_then(|s| Option::Some(s.to_string())),
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}", GAME_API_DOMAIN, "account/risky/api/check").as_str())
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<CheckRiskyResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to check risky, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        let data = json.data.unwrap();
        let (mut challenge, mut seccode, mut validate) =
            (String::new(), String::new(), String::new());
        match data.action.as_str() {
            "ACTION_NONE" => {}
            "ACTION_GEETEST" => {
                let geetest = data.geetest.unwrap();
                challenge = geetest.challenge;
                (seccode, validate) = (self.geetest_callback.expect("geetest callback is null"))(
                    challenge.as_str(),
                    geetest.gt.as_str(),
                );
            }
            _ => {
                return Err(format!(
                    "Failed to check risky, action: {}, id: {}",
                    data.action, data.id
                ));
            }
        }
        return Ok(format!(
            "id={};c={};s={};v={}",
            data.id, challenge, seccode, validate
        ));
    }

    pub async fn password_login(&mut self, account: &str, password: &str) -> Result<(), String> {
        const ACTION_TYPE: &str = "login";
        const LOGIN_API_NAME: &str = "/shield/api/login";
        let risky = match self
            .check_risky(
                ACTION_TYPE,
                LOGIN_API_NAME,
                Option::Some(account),
                Option::None,
            )
            .await
        {
            Ok(risky) => risky,
            Err(e) => return Err(e),
        };

        let mut headers = self.headers.clone();
        headers.insert(
            "x-rpc-risky",
            reqwest::header::HeaderValue::from_str(&risky).unwrap(),
        );

        let data = PasswordLoginRequest {
            account: account.to_string(),
            password: crate::common::rsa::password_encrypt(password).unwrap(),
            is_crypto: true,
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}{}", SDK_DOMAIN, "nap_cn/mdk", LOGIN_API_NAME).as_str())
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<PasswordLoginResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to login, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        let d = json.data.unwrap();

        self.account = Option::Some(AccountData {
            uid: d.account.uid,
            token: d.account.token,
        });

        return Ok(());
    }

    async fn send_captcha(&self, area: &str, mobile: &str) -> Result<(), String> {
        const ACTION_TYPE: &str = "login";
        const SEND_CAPTCHA_API_NAME: &str = "/shield/api/loginCaptcha";
        let risky = match self
            .check_risky(
                ACTION_TYPE,
                SEND_CAPTCHA_API_NAME,
                Option::None,
                Option::Some(mobile),
            )
            .await
        {
            Ok(risky) => risky,
            Err(e) => return Err(e),
        };

        let mut headers = self.headers.clone();
        headers.insert(
            "x-rpc-risky",
            reqwest::header::HeaderValue::from_str(&risky).unwrap(),
        );

        let data = SendCaptchaRequest {
            mobile: mobile.to_string(),
            area: area.to_string(),
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}{}", SDK_DOMAIN, "nap_cn/mdk", SEND_CAPTCHA_API_NAME).as_str())
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<SendCaptchaResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to send captcha, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        return Ok(());
    }

    async fn submit_captcha(
        &self,
        area: &str,
        mobile: &str,
        captcha: &str,
    ) -> Result<SubmitCaptchaResponse, String> {
        let data = SubmitCaptchaRequest {
            mobile: mobile.to_string(),
            captcha: captcha.to_string(),
            action: "Login".to_string(),
            area: area.to_string(),
        };

        let res = match reqwest::Client::new()
            .post(format!("{}{}", SDK_DOMAIN, "nap_cn/mdk/shield/api/loginCaptcha").as_str())
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<SubmitCaptchaResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to submit captcha, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }
        return Ok(json.data.unwrap());
    }

    pub async fn mobile_login(&mut self, area: &str, mobile: &str) -> Result<(), String> {
        self.send_captcha(area, mobile).await?;

        let captcha = (self.captcha_callback.expect("captcha callback is null"))();

        let d = self.submit_captcha(area, mobile, &captcha).await?;
        self.account = Option::Some(AccountData {
            uid: d.account.uid,
            token: d.account.token,
        });

        return Ok(());
    }

    pub fn save_token(&self) -> Result<String, String> {
        if self.account.is_none() {
            return Err("Account is not logged in".to_string());
        }
        let account = self.account.as_ref().unwrap();
        let data = AccountData {
            uid: account.uid.clone(),
            token: account.token.clone(),
        };
        return Ok(serde_json::to_string(&data).unwrap());
    }

    pub fn load_token(&mut self, data: &str) -> Result<(), String> {
        let account = match serde_json::from_str::<AccountData>(data) {
            Ok(account) => account,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };
        self.account = Option::Some(account);
        return Ok(());
    }

    pub async fn login_combo(&self) -> Result<(String, String), String> {
        if self.account.is_none() {
            return Err("Account is not logged in".to_string());
        }
        let account = self.account.as_ref().unwrap();
        let auth_data = AuthData {
            uid: account.uid.clone(),
            guest: false,
            token: account.token.clone(), // stoken_v2 or game_token
        };
        let inr_data = serde_json::to_string(&auth_data).unwrap();

        let mut data = LoginGameRequest {
            data: inr_data,
            app_id: 12,
            channel_id: 1,
            device: self.device.device_id.clone(),
            sign: Option::None,
        };

        data.sign = Option::Some(crate::common::hmac::sign_data(&data));

        let res = match reqwest::Client::new()
            .post(format!("{}{}", SDK_DOMAIN, "nap_cn/combo/granter/login/v2/login").as_str())
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request: {}", e)),
        };

        let json = match res.json::<Response<LoginGameResponse>>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse json: {}", e)),
        };

        if json.retcode != 0 {
            return Err(format!(
                "Failed to get combo token, retcode: {}, message: {}",
                json.retcode, json.message
            ));
        }

        let data = json.data.unwrap();

        return Ok((data.open_id, data.combo_token));
    }
}

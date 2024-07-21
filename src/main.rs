use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread::sleep,
    time::Duration,
};

use game::session;
use qrcode::{render::unicode, QrCode};

mod common;
mod game;
mod http;

const GATE_NAME: &str = "cn";
const VERSION: &str = "CNPRODWin1.0.0";
const LANGUAGE: u32 = 2;
const CHANNEL_ID: u32 = 1;
const SUB_CHANNEL_ID: u32 = 1;
const PLATFORM: u32 = 3;

const BIZ: &str = "nap_cn";
const DISPATCH_SEED: &str = "195fdb867197c041";
const RSA_VER: u32 = 3;

const DEVICE: &str = "Macbook Pro";

#[tokio::main]
async fn main() {
    let mut sdk = http::sdk::Sdk::new(
        Option::Some(|qrcode_url: &str| {
            let qrcode = QrCode::new(qrcode_url).unwrap();
            let image = qrcode
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("{}", image);
        }),
        Option::Some(|| {
            let mut captcha = String::new();
            std::io::stdin()
                .read_line(&mut captcha)
                .expect("error: unable to read user input");
            return captcha.trim().to_string();
        }),
        Option::Some(|challenge: &str, gt: &str| {
            println!("challenge: {}", challenge);
            println!("gt: {}", gt);
            let mut validate = String::new();
            std::io::stdin()
                .read_line(&mut validate)
                .expect("error: unable to read user input");
            validate = validate.trim().to_string();
            return (format!("{}|jordan", validate), validate);
        }),
    );

    let mut login_type: String = String::new();
    std::io::stdin()
        .read_line(&mut login_type)
        .expect("error: unable to read user input");
    let login_type: i32 = login_type.trim().parse().expect("Invalid login type");
    match login_type {
        0 => {
            let mut token = String::new();
            std::io::stdin()
                .read_line(&mut token)
                .expect("error: unable to read user input");
            sdk.load_token(token.trim()).expect("Load token failed");
        }
        1 => {
            sdk.qr_login().await.expect("QR login failed");
        }
        2 => {
            let (mut account, mut password) = (String::new(), String::new());
            std::io::stdin()
                .read_line(&mut account)
                .expect("error: unable to read user input");
            std::io::stdin()
                .read_line(&mut password)
                .expect("error: unable to read user input");
            sdk.password_login(account.trim(), password.trim())
                .await
                .expect("Password login failed");
        }
        3 => {
            let (mut area_code, mut phone) = (String::new(), String::new());
            std::io::stdin()
                .read_line(&mut area_code)
                .expect("error: unable to read user input");
            std::io::stdin()
                .read_line(&mut phone)
                .expect("error: unable to read user input");
            sdk.mobile_login(area_code.trim(), phone.trim())
                .await
                .expect("Captcha login failed");
        }
        _ => panic!("Invalid login type"),
    }
    println!("{}", sdk.save_token().expect("Save token failed"));

    let (account_uid, combo_token) = sdk.login_combo().await.expect("Game login failed");

    let dispatch_info = http::gate::get_regions(
        GATE_NAME,
        VERSION,
        LANGUAGE,
        CHANNEL_ID,
        SUB_CHANNEL_ID,
        PLATFORM,
    )
    .await
    .expect("Failed to get regions");
    for region in dispatch_info.region_list {
        println!("{}: {}", region.title, region.retcode);
        if region.biz == BIZ {
            let region_info = http::gate::get_region(
                &region.dispatch_url,
                VERSION,
                RSA_VER,
                LANGUAGE,
                PLATFORM,
                DISPATCH_SEED,
                CHANNEL_ID,
                SUB_CHANNEL_ID,
            )
            .await
            .expect("Failed to get region");

            println!("{}: {}", region_info.title, region_info.retcode);
            if region_info.retcode == 0 {
                let gateway = region_info.gateway.unwrap();
                println!("{}: {}", gateway.ip, gateway.port);
                let client_secret_key = region_info.client_secret_key.unwrap();

                let addr = SocketAddr::new(
                    IpAddr::V4(gateway.ip.parse::<Ipv4Addr>().unwrap()),
                    gateway.port,
                );

                let mut session = match session::Session::new(
                    addr,
                    RSA_VER,
                    &client_secret_key,
                    &account_uid,
                    &combo_token,
                    DEVICE,
                )
                .await
                {
                    Ok(v) => v,
                    Err(e) => panic!("Failed to create session: {}", e),
                };

                let _ = session.get_token().await;

                break;
            }
        }
    }

    sleep(Duration::from_secs(60));
}

use std::{thread::sleep, time::Duration};

use qrcode::{render::unicode, QrCode};

mod http;

const GATE_NAME: &str = "cn";
const VERSION: &str = "CNPRODWin1.0.0";
const LANGUAGE: i32 = 2;
const CHANNEL_ID: i32 = 1;
const SUB_CHANNEL_ID: i32 = 1;
const PLATFORM: i32 = 3;

const BIZ: &str = "nap_cn";
const DISPATCH_SEED: &str = "195fdb867197c041";
const RSA_VER: i32 = 3;

fn main() {
    // let mut sdk = http::sdk::Sdk::new(
    //     Option::None,
    //     Option::Some(|qrcode_url: &str| {
    //         let qrcode = QrCode::new(qrcode_url.clone()).unwrap();
    //         let image = qrcode
    //             .render::<unicode::Dense1x2>()
    //             .dark_color(unicode::Dense1x2::Light)
    //             .light_color(unicode::Dense1x2::Dark)
    //             .build();
    //         println!("{}", image);
    //     }),
    //     Option::None,
    //     Option::None,
    // );
    // sdk.qr_login().expect("QR login failed");
    // // sdk.password_login("account", "password").expect("Password login failed");

    let sdk = http::sdk::Sdk::new(
        Option::Some(account),
        Option::None,
        Option::None,
        Option::None,
    );
    // sdk.login_game().expect("Game login failed");

    let regions = http::gate::get_regions(
        GATE_NAME,
        VERSION,
        LANGUAGE,
        CHANNEL_ID,
        SUB_CHANNEL_ID,
        PLATFORM,
    )
    .expect("Failed to get regions");
    for region in regions {
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
            .expect("Failed to get region");

            println!("{}: {}", region_info.title, region_info.retcode);
            if region_info.retcode == 0 {
                println!("{}: {}", region_info.gateway.ip, region_info.gateway.port);
                break;
            }
        }
    }
}

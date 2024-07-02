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
    let url = http::sdk::fetch_qrcode().expect("Failed to fetch qrcode");
    println!("{}", url);
    use regex::Regex;
    let re = Regex::new(r"&ticket=([^&]*)").unwrap();
    let ticket = re.captures(&url).unwrap().get(1).unwrap().as_str();

    loop {
        let result = http::sdk::query_qrcode_status(ticket).expect("Failed to check qrcode status");
        if result.stat == "Confirmed" {
            // todo
            break;
        }
    }

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
                // todo
            }
        }
    }
    println!("Hello, world!");
}

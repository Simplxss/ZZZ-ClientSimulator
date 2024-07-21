#![allow(dead_code)]

use crate::common::rsa;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct RegionSimpleInfo {
    pub area: i32,
    pub biz: String,
    pub dispatch_url: String,
    pub env: i32,
    pub is_recommend: bool,
    pub name: String,
    pub retcode: i32,
    pub title: String,
}

#[derive(serde::Deserialize)]
pub struct DispatchInfo {
    pub region_list: Vec<RegionSimpleInfo>,
    pub retcode: i32,
}

pub async fn get_regions(
    dispatch_name: &str,
    version: &str,
    language: u32,
    channel_id: u32,
    sub_channel_id: u32,
    platform: u32,
) -> Result<DispatchInfo, String> {
    let mut domain = HashMap::new();
    domain.insert("cn", "https://globaldp-prod-cn01.juequling.com/");
    domain.insert("os", "");

    let mut params = HashMap::new();
    params.insert("version", version.to_string());
    params.insert("language", language.to_string());
    params.insert("channel_id", channel_id.to_string());
    params.insert("sub_channel_id", sub_channel_id.to_string());
    params.insert("platform", platform.to_string());

    let url = match reqwest::Url::parse_with_params(
        format!("{}{}", domain[dispatch_name], "query_dispatch").as_str(),
        &params,
    ) {
        Ok(url) => url,
        Err(e) => return Err(format!("Failed to parse url: {}", e)),
    };
    let res = match reqwest::get(url).await {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to send request: {}", e)),
    };

    return match res.json::<DispatchInfo>().await {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Failed to parse json: {}", e)),
    };
}

#[derive(serde::Deserialize)]
pub struct DesignData {
    pub base_url: String,
    pub data_revision: String,
    pub md5_files: String,
}

#[derive(serde::Deserialize)]
pub struct GameRes {
    pub audio_revision: String,
    pub base_url: String,
    pub branch: String,
    pub md5_files: String,
    pub res_revision: String,
}

#[derive(serde::Deserialize)]
pub struct SilenceData {
    pub base_url: String,
    pub md5_files: String,
    pub silence_revision: String,
}

#[derive(serde::Deserialize)]
pub struct CdnConfExt {
    pub design_data: DesignData,
    pub game_res: GameRes,
    pub silence_data: SilenceData,
}

#[derive(serde::Deserialize)]
pub struct Gateway {
    pub ip: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct FuncSwitch {
    #[serde(rename = "Close_Medium_Package_Download")]
    pub close_medium_package_download: i32,
    #[serde(rename = "Disable_Audio_Download")]
    pub disable_audio_download: i32,
    #[serde(rename = "Disable_Frequent_attempts")]
    pub disable_frequent_attempts: i32,
    #[serde(rename = "Hide_Download_complete_resources")]
    pub hide_download_complete_resources: i32,
    #[serde(rename = "Hide_Download_resources_popups")]
    pub hide_download_resources_popups: i32,
    #[serde(rename = "Hide_download_progress")]
    pub hide_download_progress: i32,
    #[serde(rename = "Medium_Package_Play")]
    pub medium_package_play: i32,
    #[serde(rename = "Play_The_Music")]
    pub play_the_music: i32,
    #[serde(rename = "disableAnimAllocatorOpt")]
    pub disable_anim_allocator_opt: i32,
    #[serde(rename = "disableAsyncSRPSubmit")]
    pub disable_async_srp_submit: i32,
    #[serde(rename = "disableAsyncUploadJob")]
    pub disable_async_upload_job: i32,
    #[serde(rename = "disableExecuteAsync")]
    pub disable_execute_async: i32,
    #[serde(rename = "disableLoadSceneParallel")]
    pub disable_load_scene_parallel: i32,
    #[serde(rename = "disableMetalPSOCreateAsync")]
    pub disable_metal_pso_create_async: i32,
    #[serde(rename = "disableObjectInstanceCache")]
    pub disable_object_instance_cache: i32,
    #[serde(rename = "disableSRPHelper")]
    pub disable_srp_helper: i32,
    #[serde(rename = "disableSRPInstancing")]
    pub disable_srp_instancing: i32,
    #[serde(rename = "disableSkinMeshStrip")]
    pub disable_skin_mesh_strip: i32,
    #[serde(rename = "disableStepPreloadMonster")]
    pub disable_step_preload_monster: i32,
    #[serde(rename = "disableTexStreamingVisbilityOpt")]
    pub disable_tex_streaming_visbility_opt: i32,
    #[serde(rename = "disableiOSGPUBufferOpt")]
    pub disable_ios_gpu_buffer_opt: i32,
    #[serde(rename = "disableiOSShaderHibernation")]
    pub disable_ios_shader_hibernation: i32,
    #[serde(rename = "enableGachaMobileConsole")]
    pub enable_gacha_mobile_console: i32,
    #[serde(rename = "enableNoticeMobileConsole")]
    pub enable_notice_mobile_console: i32,
    #[serde(rename = "enableOperationLog")]
    pub enable_operation_log: i32,
    #[serde(rename = "enableiOSShaderWarmupOnStartup")]
    pub enable_ios_shader_warmup_on_startup: i32,
    #[serde(rename = "isKcp")]
    pub is_kcp: i32,
    pub open_hotfix_popups: i32,
}

#[derive(serde::Deserialize)]
pub struct RegionExt {
    pub exchange_url: String,
    pub feedback_url: String,
    pub func_switch: FuncSwitch,
    #[serde(rename = "mtrNap")]
    pub mtr_nap: String,
    #[serde(rename = "mtrSdk")]
    pub mtr_sdk: String,
    pub pgc_webview_method: i32,
    #[serde(rename = "urlCheckNap")]
    pub url_check_nap: String,
    #[serde(rename = "urlCheckSdk")]
    pub url_check_sdk: String,
}

#[derive(serde::Deserialize)]
pub struct RegionInfo {
    pub cdn_conf_ext: Option<CdnConfExt>,
    pub client_secret_key: Option<String>,
    pub env: i32,
    pub gateway: Option<Gateway>,
    pub msg: Option<String>,
    pub region_ext: Option<RegionExt>,
    pub region_name: String,
    pub retcode: i32,
    pub stop_begin_time: Option<i32>,
    pub stop_end_time: Option<i32>,
    pub stop_jump_url: Option<String>,
    pub title: String,
}

#[derive(serde::Deserialize)]
struct QueryGateway {
    content: String,
    sign: String,
}

pub async fn get_region(
    dispatch_url: &str,
    version: &str,
    rsa_ver: u32,
    language: u32,
    platform: u32,
    dispatch_seed: &str,
    channel_id: u32,
    sub_channel_id: u32,
) -> Result<RegionInfo, String> {
    let mut params = HashMap::new();
    params.insert("version", version.to_string());
    params.insert("rsa_ver", rsa_ver.to_string());
    params.insert("language", language.to_string());
    params.insert("platform", platform.to_string());
    params.insert("seed", dispatch_seed.to_string());
    params.insert("channel_id", channel_id.to_string());
    params.insert("sub_channel_id", sub_channel_id.to_string());

    let url = match reqwest::Url::parse_with_params(dispatch_url, &params) {
        Ok(url) => url,
        Err(e) => return Err(format!("Failed to parse url: {}", e)),
    };
    let res = match reqwest::get(url).await {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to send request: {}", e)),
    };

    let json = match res.json::<QueryGateway>().await {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to parse QueryGateway: {}", e)),
    };

    let content = match rsa::decrypt_content(&json.content, rsa_ver) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to decrypt content: {}", e)),
    };

    match rsa::rsa_verify_sign(&content, &json.sign, rsa_ver) {
        Ok(true) => (),
        Ok(false) => return Err("Failed to verify sign".to_string()),
        Err(e) => return Err(format!("Failed to verify sign: {}", e)),
    };

    return match serde_json::from_str::<RegionInfo>(&content) {
        Ok(region_info) => Ok(region_info),
        Err(e) => Err(format!("Failed to parse RegionInfo: {}", e)),
    };
}

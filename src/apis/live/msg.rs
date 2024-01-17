//! 发送弹幕API

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::blocking::{Client, Response};
use serde::Deserialize;

use crate::credential::Credential;

#[derive(Debug, Clone)]
pub struct LiveMessageFormContent {
    /// 弹幕内容
    pub msg: String,
    /// 弹幕颜色 (hex color string)
    pub color: i32,
    /// 字体大小 (e.g., 25)
    pub fontsize: i32,
    /// 模式 (1: 飘屏, 4: 底部, 5: 顶部)
    pub mode: i32,
    /// 气泡
    pub bubble: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SendLiveMessageResponse {
    Success {
        code: i32,
        message: String,
        data: SendLiveMessageData,
    },
    Failure {
        code: i32,
        message: String,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct SendLiveMessageData {
    pub mode_info: SendLiveMessageModeInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SendLiveMessageModeInfo {
    pub mode: i32,
    pub show_player_type: i32,
    pub extra: String,
}

#[allow(clippy::too_many_arguments)]
pub fn send_live_message(
    client: &Client,
    room_id: i32,  // 直播间id
    msg: &str,     // 弹幕内容
    color: i32,    // 弹幕颜色
    fontsize: i32, // 字体大小
    mode: i32,     // 弹幕模式 (1 - 飘屏, 4 - 底部, 5 - 顶部)
    bubble: i32,   // 气泡 (1)
    credential: &Credential,
) -> std::result::Result<SendLiveMessageResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/msg/send";
    let rnd = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let form = [
        ("color", &*color.to_string()),
        ("fontsize", &*fontsize.to_string()),
        ("mode", &*mode.to_string()),
        ("msg", msg),
        ("rnd", &*rnd),
        ("roomid", &*room_id.to_string()),
        ("bubble", &*bubble.to_string()),
        ("csrf", &credential.bili_jct),
        ("csrf_token", &credential.bili_jct),
    ];
    let request = client
        .post(API_URL)
        .header("cookie", credential.to_cookie_str())
        .form(&form)
        .header("User-Agent", crate::apis::USER_AGENT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .build()?;
    client.execute(request).and_then(Response::json)
}

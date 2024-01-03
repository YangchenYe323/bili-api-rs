//! 发送弹幕API

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::blocking::{Client, Response};
use serde::Deserialize;

#[derive(Debug)]
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

#[derive(Debug, Deserialize)]
pub struct SendLiveMessageResponse {
    pub code: i32,
    pub message: String,
    pub msg: String,
    pub data: SendLiveMessageData,
}

#[derive(Debug, Deserialize)]
pub struct SendLiveMessageData {
    pub mode_info: SendLiveMessageModeInfo,
}

#[derive(Debug, Deserialize)]
pub struct SendLiveMessageModeInfo {
    pub mode: i32,
    pub show_player_type: i32,
    pub extra: String,
}

#[allow(clippy::too_many_arguments)]
pub fn send_live_message(
    client: &Client,
    room_id: i32,     // 直播间id
    msg: &str,        // 弹幕内容
    color: i32,       // 弹幕颜色
    fontsize: i32,    // 字体大小
    mode: i32,        // 弹幕模式 (1 - 飘屏, 4 - 底部, 5 - 顶部)
    bubble: i32,      // 气泡 (1)
    csrf: &str,       // cookie bili_jct字段
    raw_cookie: &str, // SESSDATA
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
        ("csrf", csrf),
        ("csrf_token", csrf),
    ];
    let request = client.post(API_URL).header("cookie", raw_cookie).form(&form).build()?;
    client.execute(request).and_then(Response::json)
}

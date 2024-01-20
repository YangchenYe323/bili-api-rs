//! 发送弹幕API

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{credential::Credential, utils::handle_api_response};

pub const DEFAULT_FONTSIZE: i32 = 25;
pub const WHITE: i32 = 0xffffff;

/// 弹幕模式
/// 0: 飘屏
/// 4: 底部
/// 5: 顶部
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum LiveMessageMode {
  #[default]
  Float = 0,
  Bottom = 4,
  Top = 5,
}

impl LiveMessageMode {
  pub fn as_str(self) -> &'static str {
    match self {
      LiveMessageMode::Float => "0",
      LiveMessageMode::Bottom => "4",
      LiveMessageMode::Top => "5",
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveMessageConfig {
  /// 房间id
  roomid: i32,
  /// 弹幕内容
  msg: String,
  /// 弹幕颜色 (hex color string)
  color: i32,
  /// 字体大小 (e.g., 25)
  fontsize: i32,
  /// 模式 (1: 飘屏, 4: 底部, 5: 顶部)
  mode: LiveMessageMode,
  /// 气泡 (意义不明)
  bubble: i32,
  /// 当前时间戳，由callee提供
  pub(crate) rnd: String,
  /// CSRF: bili_jct
  pub(crate) csrf: String,
  /// CSRF token: bili_jct
  pub(crate) csrf_token: String,
}

impl Default for LiveMessageConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl LiveMessageConfig {
  pub fn new() -> LiveMessageConfig {
    Self {
      roomid: 0,
      msg: String::new(),
      color: WHITE,
      fontsize: DEFAULT_FONTSIZE,
      mode: LiveMessageMode::Float,
      bubble: 0,
      rnd: String::new(),
      csrf: String::new(),
      csrf_token: String::new(),
    }
  }

  pub fn with_roomid_and_msg(room_id: i32, msg: String) -> LiveMessageConfig {
    Self {
      roomid: room_id,
      msg,
      color: WHITE,
      fontsize: DEFAULT_FONTSIZE,
      mode: LiveMessageMode::Float,
      bubble: 0,
      rnd: String::new(),
      csrf: String::new(),
      csrf_token: String::new(),
    }
  }

  pub fn msg(&mut self, msg: String) -> &mut LiveMessageConfig {
    self.msg = msg;
    self
  }

  pub fn color(&mut self, color: i32) -> &mut Self {
    self.color = color;
    self
  }

  pub fn fontsize(&mut self, fontsize: i32) -> &mut Self {
    self.fontsize = fontsize;
    self
  }

  pub fn mode(&mut self, mode: LiveMessageMode) -> &mut Self {
    self.mode = mode;
    self
  }

  pub fn bubble(&mut self, bubble: i32) -> &mut Self {
    self.bubble = bubble;
    self
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SendLiveMessageResponse {
  data: SendLiveMessageData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SendLiveMessageData {
  pub mode_info: SendLiveMessageModeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SendLiveMessageModeInfo {
  pub mode: i32,
  pub show_player_type: i32,
  pub extra: String,
}

pub fn send_live_message(
  client: &Client,
  mut config: LiveMessageConfig,
  credential: &Credential,
) -> crate::Result<SendLiveMessageResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/msg/send";
  let rnd = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string();

  config.rnd = rnd;
  config.csrf = credential.bili_jct.clone();
  config.csrf_token = credential.bili_jct.clone();

  let request = client
    .post(API_URL)
    .header("cookie", credential.to_cookie_str())
    .form(&config)
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;

  handle_api_response(client.execute(request)?)
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use crate::{
    credential::extract_credential::{get_credential_for_test_or_abort, get_fake_credential},
    error::REQUEST_ERROR,
    utils::assert_error_code,
  };

  use super::{send_live_message, LiveMessageConfig};

  #[test]
  pub fn test_send_msg() {
    let agent = reqwest::blocking::Client::new();
    let credential = get_credential_for_test_or_abort();
    let config = LiveMessageConfig::with_roomid_and_msg(1029, "加油".to_string());

    // 正常发送弹幕成功
    let result = send_live_message(&agent, config, &credential);
    assert!(result.is_ok());
    std::thread::sleep(Duration::from_millis(500));

    // 用户登录错误
    let invalid_cred = get_fake_credential();
    let config = LiveMessageConfig::with_roomid_and_msg(1029, "加油".to_string());
    assert_error_code(
      send_live_message(&agent, config, &invalid_cred),
      REQUEST_ERROR,
    );

    std::thread::sleep(Duration::from_millis(500));

    // 参数错误(fontsize为0)
    let mut config = LiveMessageConfig::with_roomid_and_msg(1029, "加油".to_string());
    config.fontsize(0);
    let result = send_live_message(&agent, config, &credential);
    assert_error_code(result, REQUEST_ERROR);
  }
}

//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/user/info.md

use serde::{Deserialize, Serialize};

use crate::{credential::Credential, utils::handle_api_response, Client};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MyInfoResponse {
  pub data: MyInfoData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MyInfoData {
  pub mid: i64,
  pub name: String,
  pub sex: String,
  pub face: String,
  pub sign: String,
  pub rank: i32,
  pub level: i32,
  pub jointime: i32,
  pub moral: i32,
  pub silence: i32,
  pub email_status: i32,
  pub tel_status: i32,
  pub identification: i32,
  pub vip: VipStatus,
  pub birthday: i64,
  pub is_tourist: i32,
  pub is_fake_account: i32,
  pub pin_prompting: i32,
  pub is_deleted: i32,
  pub coins: f32,
  pub following: i32,
  pub follower: i32,
  pub pendant: Pendant,
  pub nameplate: Nameplate,
  pub official: Official,
  pub level_exp: LevelExp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VipStatus {
  #[serde(rename = "type")]
  pub r#type: i32,
  pub status: i32,
  pub due_date: i64,
  pub theme_type: i32,
  pub label: VipLabel,
  pub avatar_subscript: i32,
  pub nickname_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VipLabel {
  pub path: String,
  pub text: String,
  pub label_theme: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Pendant {
  pub pid: i32,
  pub name: String,
  pub image: String,
  pub expire: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Nameplate {
  pub nid: i32,
  pub name: String,
  pub image: String,
  pub image_small: String,
  pub level: String,
  pub condition: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Official {
  pub role: i32,
  pub title: String,
  pub desc: String,
  #[serde(rename = "type")]
  pub r#type: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LevelExp {
  pub current_level: i32,
  pub current_min: i32,
  pub current_exp: i32,
  pub next_exp: i32,
}

pub fn get_my_info(client: &Client, credential: &Credential) -> crate::Result<MyInfoResponse> {
  const API_URL: &str = "https://api.bilibili.com/x/space/myinfo";

  let request = client
    .get(API_URL)
    .header("Cookie", format!("SESSDATA={}", credential.sessdata))
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;

  handle_api_response(client.execute(request)?)
}

#[cfg(test)]
mod tests {
  use super::get_my_info;
  use crate::{
    credential::extract_credential::{get_credential_for_test_or_abort, get_fake_credential},
    utils::assert_error_code,
    Client,
  };

  #[test]
  fn test_get_my_info_success() {
    let agent = Client::new();
    let cred = get_credential_for_test_or_abort();

    let result = get_my_info(&agent, &cred);

    assert!(result.is_ok())
  }

  #[test]
  fn test_get_my_info_unlogged_in() {
    let agent = Client::new();
    let cred = get_fake_credential();
    let result = get_my_info(&agent, &cred);
    assert_error_code(result, -400);
  }
}

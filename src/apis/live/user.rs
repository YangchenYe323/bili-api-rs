//! Please refer to `https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/live/user.md`
//! for API documentation.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::{credential::Credential, utils::handle_api_response};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct GetMedalForUserResponse {
  pub data: GetMedalForUserData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMedalForUserData {
  pub count: i32,
  pub items: Vec<MedalItem>,
  pub page_info: PageInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MedalItem {
  pub can_deleted: bool,
  pub day_limit: i32,
  pub guard_level: i32,
  pub guard_medal_title: String,
  pub intimacy: i32,
  pub is_lighted: i32,
  pub level: i32,
  pub medal_name: String,
  pub medal_color_border: i32,
  pub medal_color_start: i32,
  pub medal_color_end: i32,
  pub medal_id: i32,
  pub next_intimacy: i32,
  pub today_feed: i32,
  pub roomid: i32,
  pub status: i32,
  pub target_id: i64,
  pub target_name: String,
  pub uname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageInfo {
  pub total_page: i32,
  pub cur_page: i32,
}

pub fn get_medal_for_user(
  client: &Client,
  page_size: i32,
  num_page: i32,
  credential: &Credential,
) -> crate::Result<GetMedalForUserResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/app-ucenter/v1/user/GetMyMedals";
  let url = format!("{}?page={}&page_size={}", API_URL, num_page, page_size);
  let request = client
    .get(url)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;

  handle_api_response(client.execute(request)?)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WearMedalResponse {}

pub fn wear_medal(
  client: &Client,
  medal_id: i32,
  credential: &Credential,
) -> crate::Result<WearMedalResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/fansMedal/wear";
  let params = [
    ("medal_id", &*medal_id.to_string()),
    ("csrf", &credential.bili_jct),
    ("csrf_token", &credential.bili_jct),
  ];

  let request = client
    .post(API_URL)
    .header("cookie", credential.to_cookie_str())
    .form(&params)
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;

  handle_api_response(client.execute(request)?)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiveCheckinResponse;

pub fn live_checkin(
  client: &Client,
  credential: &Credential,
) -> crate::Result<LiveCheckinResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign";
  let request = client
    .get(API_URL)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;

  handle_api_response(client.execute(request)?)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MonthlyLiveCheckinInfoResponse {
  pub data: MonthlyLiveCheckinInfoData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MonthlyLiveCheckinInfoData {
  pub text: String,
  pub special_text: String,
  pub status: i32,
  pub all_days: i32,
  pub cur_month: i32,
  pub cur_year: i32,
  pub cur_day: i32,
  pub cur_date: String,
  pub had_sign_days: i32,
  pub new_task: i32,
  pub sign_days_list: Vec<i32>,
  pub sign_bonus_days_list: Vec<i32>,
}

pub fn get_monthly_live_checkin_info(
  client: &Client,
  credential: &Credential,
) -> crate::Result<MonthlyLiveCheckinInfoResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/WebGetSignInfo";
  let request = client
    .get(API_URL)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;
  handle_api_response(client.execute(request)?)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LastMonthLiveCheckInInfoResponse {
  pub data: LastMonthLiveCheckInData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LastMonthLiveCheckInData {
  pub days: i32,
  pub month: i32,
  pub had_sign_days: i32,
  pub sign_days_list: Vec<i32>,
  pub sign_bonus_days_list: Vec<i32>,
}

pub fn get_last_month_live_checkin_info(
  client: &Client,
  credential: &Credential,
) -> crate::Result<LastMonthLiveCheckInInfoResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/sign/getLastMonthSignDays";
  let request = client
    .get(API_URL)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;
  handle_api_response(client.execute(request)?)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetInfoByUserResponse {
  pub data: GetInfoByUserData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetInfoByUserData {
  pub property: UserLiveRoomProperty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserLiveRoomProperty {
  pub bubble: i32,
  pub bubble_color: String,
  pub danmu: UserDanmuProperty,
  pub uname_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserDanmuProperty {
  pub color: i32,
  pub length: i32,
  pub mode: i32,
  pub room_id: i32,
}

pub fn get_live_info_by_user(
  client: &Client,
  room_id: i32,
  credential: &Credential,
) -> crate::Result<GetInfoByUserResponse> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByUser";
  let url = format!("{}?room_id={}", API_URL, room_id);
  let request = client
    .get(url)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;
  handle_api_response(client.execute(request)?)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    credential::extract_credential::get_credential_for_test_or_abort,
    error::{REQUEST_ERROR, ROOM_NOT_EXIST},
    utils::assert_error_code,
  };

  #[test]
  fn test_get_medal_for_user() {
    let agent = Client::new();
    let credential = get_credential_for_test_or_abort();
    let invalid_credential = Credential::new("123".to_string(), "456".to_string());
    // Success scenario
    assert!(get_medal_for_user(&agent, 10, 1, &credential).is_ok());
    // Failure scenario
    assert_error_code(
      get_medal_for_user(&agent, 10, 1, &invalid_credential),
      REQUEST_ERROR,
    );
  }

  #[test]
  fn test_get_monthly_live_checkin_info() {
    let agent = Client::new();
    let credential = get_credential_for_test_or_abort();
    let invalid_credential = Credential::new("123".to_string(), "456".to_string());
    // Success scenario
    assert!(get_monthly_live_checkin_info(&agent, &credential).is_ok());
    // Failure scenario
    assert_error_code(
      get_monthly_live_checkin_info(&agent, &invalid_credential),
      REQUEST_ERROR,
    );
  }

  #[test]
  fn test_get_last_month_live_checkin_info() {
    let agent = Client::new();
    let credential = get_credential_for_test_or_abort();
    let invalid_credential = Credential::new("123".to_string(), "456".to_string());
    // Success scenario
    assert!(get_last_month_live_checkin_info(&agent, &credential).is_ok());
    // Failure scenario
    assert_error_code(
      get_last_month_live_checkin_info(&agent, &invalid_credential),
      -101,
    );
  }

  #[test]
  fn test_get_live_info_by_user() {
    let agent = Client::new();
    let credential = get_credential_for_test_or_abort();
    let invalid_credential = Credential::new("123".to_string(), "456".to_string());
    // Success scenario
    assert!(get_live_info_by_user(&agent, 1029, &credential).is_ok());
    // Failure scenario
    assert_error_code(
      get_live_info_by_user(&agent, 1029, &invalid_credential),
      REQUEST_ERROR,
    );
    assert_error_code(
      get_live_info_by_user(&agent, 44444444, &credential),
      ROOM_NOT_EXIST,
    );
  }
}

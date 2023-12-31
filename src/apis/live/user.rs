//! Please refer to `https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/live/user.md`
//! for API documentation.

// Code is generated with the assistance of GPT-4

use serde::Deserialize;

use crate::{BiliClient, BiliApiRequest, BiliApiResponse};

#[derive(Debug, Deserialize)]
pub struct GetMedalForUserResponse {
  pub code: i32,
  pub message: String,
  pub ttl: i32,
  pub data: GetMedalForUserData,
}

#[derive(Debug, Deserialize)]
pub struct GetMedalForUserData {
  pub count: i32,
  pub items: Vec<MedalItem>,
  pub page_info: PageInfo,
}

#[derive(Debug, Deserialize)]
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
  pub target_id: i32,
  pub target_name: String,
  pub uname: String,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
  pub total_page: i32,
  pub cur_page: i32,
}

pub fn get_medal_for_user<T: BiliClient>(client: T, page_size: u32, num_page: u32, raw_cookie: &str) -> std::result::Result<GetMedalForUserResponse, <T::Request as BiliApiRequest>::Error> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/app-ucenter/v1/user/GetMyMedals";
  let url = format!("{}?page={}&page_size={}", API_URL, num_page, page_size);
  let request = client.create_request("GET", &url).set_header("cookie", raw_cookie);
  let response = request.execute()?;
  Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct WearMedalResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
    // data field is often empty
}

pub fn wear_medal<T: BiliClient>(
  client: T, 
  medal_id: i32, 
  csrf: &str, 
  csrf_token: &str,
  raw_cookie: &str,
) -> std::result::Result<WearMedalResponse, <T::Request as BiliApiRequest>::Error> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/fansMedal/wear";
  let params = [("medal_id", &*medal_id.to_string()), ("csrf", csrf), ("csrf_token", csrf_token)];
  let response = client.create_request("POST", API_URL)
                      .set_header("cookie", raw_cookie).execute_post_form(&params)?;
  Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct LiveCheckinResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
}

pub fn live_checkin<T: BiliClient>(
  client: T,
  raw_cookie: &str
) -> std::result::Result<LiveCheckinResponse, <T::Request as BiliApiRequest>::Error> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign";
  let request = client.create_request("GET", API_URL)
                      .set_header("cookie", raw_cookie);
  let response = request.execute()?;
  Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct MonthlyLiveCheckinInfoResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
    pub data: MonthlyLiveCheckinInfoData,
}

#[derive(Debug, Deserialize)]
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

pub fn get_monthly_live_checkin_info<T: BiliClient>(
  client: T,
  raw_cookie: &str
) -> std::result::Result<MonthlyLiveCheckinInfoResponse, <T::Request as BiliApiRequest>::Error> {
  const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/WebGetSignInfo";
  let request = client.create_request("GET", API_URL)
                      .set_header("cookie", raw_cookie);
  let response = request.execute()?;
  Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct LastMonthLiveCheckInInfoResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
    pub data: LastMonthLiveCheckInData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LastMonthLiveCheckInData {
    pub days: i32,
    pub month: i32,
    pub had_sign_days: i32,
    pub sign_days_list: Vec<i32>,
    pub sign_bonus_days_list: Vec<i32>,
}

pub fn get_last_month_live_checkin_info<T: BiliClient>(
  client: T,
  raw_cookie: &str
) -> std::result::Result<LastMonthLiveCheckInInfoResponse, <T::Request as BiliApiRequest>::Error> {
  const API_URL: &str = "https://api.live.bilibili.com/sign/getLastMonthSignDays";
  let request = client.create_request("GET", API_URL)
                      .set_header("cookie", raw_cookie);
  let response = request.execute()?;
  Ok(response.deserialize_json().unwrap())
}

#[test]
fn test() {
  let agent = ureq::agent();
  println!("{:?}", get_last_month_live_checkin_info(agent, "ABCDSEF"));
}
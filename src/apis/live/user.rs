//! Please refer to `https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/live/user.md`
//! for API documentation.

use reqwest::blocking::{Client, Response};
use serde::Deserialize;

use crate::credential::Credential;

#[derive(Debug, Deserialize)]
#[serde(untagged)]

pub enum GetMedalForUserResponse {
    Success {
        code: i32,
        data: GetMedalForUserData,
        ttl: i32,
    },
    Failure {
        code: i32,
        message: String,
        ttl: i32,
    },
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
    pub target_id: i64,
    pub target_name: String,
    pub uname: String,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub total_page: i32,
    pub cur_page: i32,
}

pub fn get_medal_for_user(
    client: &Client,
    page_size: i32,
    num_page: i32,
    credential: &Credential,
) -> std::result::Result<GetMedalForUserResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/app-ucenter/v1/user/GetMyMedals";
    let url = format!("{}?page={}&page_size={}", API_URL, num_page, page_size);
    let request = client
        .get(url)
        .header("cookie", credential.to_cookie_str())
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[derive(Debug, Deserialize)]
pub struct WearMedalResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
    // data field is often empty
}

pub fn wear_medal(
    client: &Client,
    medal_id: i32,
    credential: &Credential,
) -> std::result::Result<WearMedalResponse, reqwest::Error> {
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
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[derive(Debug, Deserialize)]
pub struct LiveCheckinResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
}

pub fn live_checkin(
    client: &Client,
    credential: &Credential,
) -> std::result::Result<LiveCheckinResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign";
    let request = client
        .get(API_URL)
        .header("cookie", credential.to_cookie_str())
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonthlyLiveCheckinInfoResponse {
    Success {
        code: i32,
        ttl: i32,
        data: MonthlyLiveCheckinInfoData,
    },
    Failure {
        code: i32,
        ttl: i32,
        message: String,
    },
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

pub fn get_monthly_live_checkin_info(
    client: &Client,
    credential: &Credential,
) -> std::result::Result<MonthlyLiveCheckinInfoResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/WebGetSignInfo";
    let request = client
        .get(API_URL)
        .header("cookie", credential.to_cookie_str())
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LastMonthLiveCheckInInfoResponse {
    Success {
        code: i32,
        ttl: i32,
        data: LastMonthLiveCheckInData,
    },
    Failure {
        code: i32,
        ttl: i32,
        message: String,
    },
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

pub fn get_last_month_live_checkin_info(
    client: &Client,
    credential: &Credential,
) -> std::result::Result<LastMonthLiveCheckInInfoResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/sign/getLastMonthSignDays";
    let request = client
        .get(API_URL)
        .header("cookie", credential.to_cookie_str())
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetInfoByUserResponse {
    Success {
        code: i32,
        ttl: i32,
        data: GetInfoByUserData,
    },
    Failure {
        code: i32,
        ttl: i32,
        message: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct GetInfoByUserData {
    pub property: UserLiveRoomProperty,
}

#[derive(Debug, Deserialize)]
pub struct UserLiveRoomProperty {
    pub bubble: i32,
    pub bubble_color: String,
    pub danmu: UserDanmuProperty,
    pub uname_color: String,
}

#[derive(Debug, Deserialize)]
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
) -> std::result::Result<GetInfoByUserResponse, reqwest::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByUser";
    let url = format!("{}?room_id={}", API_URL, room_id);
    let request = client
        .get(url)
        .header("cookie", credential.to_cookie_str())
        .header("User-Agent", crate::apis::USER_AGENT).build()?;
    client.execute(request).and_then(Response::json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_medal_for_user() {
        let agent = reqwest::blocking::Client::new();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let bili_jct = std::env::var("BILI_JCT").expect("Please set valid bili_jct for testing");
        let credential = Credential::new(sessdata, bili_jct);
        let invalid_credential = Credential::new("123".to_string(), "456".to_string());
        // Success scenario
        get_medal_for_user(&agent, 10, 1, &credential).unwrap();
        // Failure scenario
        get_medal_for_user(&agent, 10, 1, &invalid_credential).unwrap();
    }

    #[test]
    fn test_get_monthly_live_checkin_info() {
        let agent = reqwest::blocking::Client::new();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let bili_jct = std::env::var("BILI_JCT").expect("Please set valid bili_jct for testing");
        let credential = Credential::new(sessdata, bili_jct);
        let invalid_credential = Credential::new("123".to_string(), "456".to_string());
        // Success scenario
        assert!(matches!(
            get_monthly_live_checkin_info(&agent, &credential).unwrap(),
            MonthlyLiveCheckinInfoResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_monthly_live_checkin_info(&agent, &invalid_credential).unwrap(),
            MonthlyLiveCheckinInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_last_month_live_checkin_info() {
        let agent = reqwest::blocking::Client::new();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let bili_jct = std::env::var("BILI_JCT").expect("Please set valid bili_jct for testing");
        let credential = Credential::new(sessdata, bili_jct);
        let invalid_credential = Credential::new("123".to_string(), "456".to_string());
        // Success scenario
        assert!(matches!(
            get_last_month_live_checkin_info(&agent, &credential).unwrap(),
            LastMonthLiveCheckInInfoResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_last_month_live_checkin_info(&agent, &invalid_credential).unwrap(),
            LastMonthLiveCheckInInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_live_info_by_user() {
        let agent = reqwest::blocking::Client::new();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let bili_jct = std::env::var("BILI_JCT").expect("Please set valid bili_jct for testing");
        let credential = Credential::new(sessdata, bili_jct);
        let invalid_credential = Credential::new("123".to_string(), "456".to_string());
        // Success scenario
        assert!(matches!(
            get_live_info_by_user(&agent, 1029, &credential).unwrap(),
            GetInfoByUserResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_live_info_by_user(&agent, 1029, &invalid_credential).unwrap(),
            GetInfoByUserResponse::Failure { .. }
        ));
        assert!(matches!(
            get_live_info_by_user(&agent, 44444444, &credential).unwrap(),
            GetInfoByUserResponse::Failure { .. }
        ));
    }
}

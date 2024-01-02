//! Please refer to `https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/live/user.md`
//! for API documentation.

use serde::Deserialize;

use crate::{BiliApiRequest, BiliApiResponse, BiliClient};

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

pub fn get_medal_for_user<T: BiliClient>(
    client: &T,
    page_size: i32,
    num_page: i32,
    raw_cookie: &str,
) -> std::result::Result<GetMedalForUserResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/app-ucenter/v1/user/GetMyMedals";
    let url = format!("{}?page={}&page_size={}", API_URL, num_page, page_size);
    let request = client
        .create_request("GET", &url)
        .set_header("cookie", raw_cookie);
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
    client: &T,
    medal_id: i32,
    csrf: &str,
    csrf_token: &str,
    raw_cookie: &str,
) -> std::result::Result<WearMedalResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/fansMedal/wear";
    let params = [
        ("medal_id", &*medal_id.to_string()),
        ("csrf", csrf),
        ("csrf_token", csrf_token),
    ];
    let response = client
        .create_request("POST", API_URL)
        .set_header("cookie", raw_cookie)
        .execute_post_form(&params)?;
    Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct LiveCheckinResponse {
    pub code: i32,
    pub ttl: i32,
    pub message: String,
}

pub fn live_checkin<T: BiliClient>(
    client: &T,
    raw_cookie: &str,
) -> std::result::Result<LiveCheckinResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign";
    let request = client
        .create_request("GET", API_URL)
        .set_header("cookie", raw_cookie);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
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

pub fn get_monthly_live_checkin_info<T: BiliClient>(
    client: &T,
    raw_cookie: &str,
) -> std::result::Result<MonthlyLiveCheckinInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/WebGetSignInfo";
    let request = client
        .create_request("GET", API_URL)
        .set_header("cookie", raw_cookie);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
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

pub fn get_last_month_live_checkin_info<T: BiliClient>(
    client: &T,
    raw_cookie: &str,
) -> std::result::Result<LastMonthLiveCheckInInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/sign/getLastMonthSignDays";
    let request = client
        .create_request("GET", API_URL)
        .set_header("cookie", raw_cookie);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
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

pub fn get_live_info_by_user<T: BiliClient>(
    client: &T,
    room_id: i32,
    raw_cookie: &str,
) -> std::result::Result<GetInfoByUserResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByUser";
    let url = format!("{}?room_id={}", API_URL, room_id);
    let request = client.create_request("GET", &url);
    let response = request.set_header("cookie", raw_cookie).execute()?;
    Ok(response.deserialize_json().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_medal_for_user() {
        let agent = ureq::agent();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let raw_cookie = format!("SESSDATA={}", sessdata);
        // Success scenario
        get_medal_for_user(&agent, 10, 1, &raw_cookie).unwrap();
        // Failure scenario
        get_medal_for_user(&agent, 10, 1, "123").unwrap();
    }

    #[test]
    fn test_get_monthly_live_checkin_info() {
        let agent = ureq::agent();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let raw_cookie = format!("SESSDATA={}", sessdata);
        // Success scenario
        assert!(matches!(
            get_monthly_live_checkin_info(&agent, &raw_cookie).unwrap(),
            MonthlyLiveCheckinInfoResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_monthly_live_checkin_info(&agent, "123").unwrap(),
            MonthlyLiveCheckinInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_last_month_live_checkin_info() {
        let agent = ureq::agent();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let raw_cookie = format!("SESSDATA={}", sessdata);
        // Success scenario
        assert!(matches!(
            get_last_month_live_checkin_info(&agent, &raw_cookie).unwrap(),
            LastMonthLiveCheckInInfoResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_last_month_live_checkin_info(&agent, "123").unwrap(),
            LastMonthLiveCheckInInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_live_info_by_user() {
        let agent = ureq::agent();
        let sessdata = std::env::var("SESSDATA").expect("Please set valid SESSDATA for testing");
        let raw_cookie = format!("SESSDATA={}", sessdata);
        // Success scenario
        assert!(matches!(
            get_live_info_by_user(&agent, 1029, &raw_cookie).unwrap(),
            GetInfoByUserResponse::Success { .. }
        ));
        // Failure scenario
        assert!(matches!(
            get_live_info_by_user(&agent, 1029, "123").unwrap(),
            GetInfoByUserResponse::Failure { .. }
        ));
        assert!(matches!(
            get_live_info_by_user(&agent, 44444444, &raw_cookie).unwrap(),
            GetInfoByUserResponse::Failure { .. }
        ));
    }
}

//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/user/info.md

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
  apis::misc::sign::wbi::do_wbi_signature, credential::Credential, utils::handle_api_response,
  Client,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserInfoResponse {
  pub data: UserInfoData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserInfoData {
  pub mid: i64,
  pub name: String,
  pub sex: String,
  pub face: String,
  pub face_nft: i32,
  pub face_nft_type: i32,
  pub sign: String,
  pub rank: i32,
  pub level: i32,
  pub jointime: i32,
  pub moral: i32,
  pub silence: i32,
  pub coins: f32,
  pub fans_badge: bool,
  pub fans_medal: Option<FansMedal>,
  pub official: Official,
  pub vip: InfoVipStatus,
  pub pendant: InfoPendant,
  pub nameplate: Nameplate,
  pub user_honour_info: UserHonourInfo,
  pub is_followed: bool,
  pub top_photo: String,
  pub theme: Option<Theme>,
  #[serde(deserialize_with = "crate::utils::non_empty_json_obj")]
  pub sys_notice: Option<SysNotice>,
  pub live_room: Option<LiveRoom>,
  pub birthday: String,
  pub school: Option<School>,
  pub profession: Option<Profession>,
  pub tags: Option<Vec<String>>,
  pub series: Option<Series>,
  pub is_senior_member: i32,
  pub mcn_info: Option<MCNInfo>,
  pub gaia_res_type: i32,
  pub gaia_data: Option<GaiaData>,
  pub is_risk: bool,
  pub elec: Option<Elec>,
  pub contract: Option<Contract>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FansMedal {
  pub show: bool,
  pub wear: bool,
  pub medal: Medal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Medal {
  pub uid: i64,
  pub target_id: i64,
  pub medal_id: i64,
  pub level: i32,
  pub medal_name: String,
  pub medal_color: i32,
  pub intimacy: i32,
  pub next_intimacy: i32,
  pub day_limit: i32,
  pub today_feed: Option<i32>,
  pub medal_color_start: i32,
  pub medal_color_end: i32,
  pub medal_color_border: i32,
  pub is_lighted: i32,
  pub light_status: i32,
  pub wearing_status: i32,
  pub score: i32,
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
pub struct InfoVipStatus {
  #[serde(rename = "type")]
  pub r#type: i32,
  pub status: i32,
  pub due_date: i64,
  pub vip_pay_type: i32,
  pub theme_type: i32,
  pub label: InfoVipLabel,
  pub avatar_subscript: i32,
  pub nickname_color: String,
  pub role: i32,
  pub avatar_subscript_url: String,
  pub tv_vip_status: i32,
  pub tv_vip_pay_type: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InfoVipLabel {
  pub path: String,
  pub text: String,
  pub label_theme: String,
  pub text_color: String,
  pub bg_style: i32,
  pub bg_color: String,
  pub border_color: String,
  pub use_img_label: bool,
  pub img_label_uri_hans: String,
  pub img_label_uri_hant: String,
  pub img_label_uri_hans_static: String,
  pub img_label_uri_hant_static: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InfoPendant {
  pub pid: i32,
  pub name: String,
  pub image: String,
  pub expire: i64,
  pub image_enhance: String,
  pub image_enhance_frame: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserHonourInfo {
  pub mid: i64,
  pub colour: Option<String>,
  pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Theme {
  // Define fields as per API documentation if available
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SysNotice {
  pub id: i32,
  pub content: String,
  pub notice_type: i32,
  pub icon: String,
  pub text_color: String,
  pub bg_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiveRoom {
  #[serde(rename = "roomStatus")]
  pub room_status: i32,
  #[serde(rename = "liveStatus")]
  pub live_status: i32,
  pub url: String,
  pub title: String,
  pub cover: String,
  pub watched_show: Option<WatchedShow>,
  pub roomid: i32,
  #[serde(rename = "roundStatus")]
  pub round_status: i32,
  pub broadcast_type: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WatchedShow {
  pub switch: bool,
  pub num: i32,
  pub text_small: String,
  pub text_large: String,
  pub icon: String,
  pub icon_location: String,
  pub icon_web: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct School {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Profession {
  pub name: String,
  pub department: String,
  pub title: String,
  pub is_show: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Series {
  pub user_upgrade_status: i32,
  pub show_upgrade_window: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MCNInfo {
  // Define fields as per API documentation if available
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GaiaData {
  // Define fields as per API documentation if available
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Elec {
  pub show_info: ShowInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ShowInfo {
  pub show: bool,
  pub state: i32,
  pub title: String,
  pub icon: String,
  pub jump_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Contract {
  pub is_display: bool,
  pub is_follow_display: bool,
}

pub fn get_user_info(
  client: &Client,
  credential: &Credential,
  mid: i64,
) -> crate::Result<UserInfoResponse> {
  const API_URL: &str = "https://api.bilibili.com/x/space/wbi/acc/info";

  let mut params = BTreeMap::new();
  let mid = mid.to_string();
  params.insert("mid", &*mid);

  // Perform wbi signature
  let (wbi, _) = do_wbi_signature(client, credential, &params)?;
  params.insert("wts", wbi.wts());
  params.insert("w_rid", wbi.w_rid());

  let request = client
    .get(API_URL)
    .header("Cookie", format!("SESSDATA={}", credential.sessdata))
    .header("User-Agent", crate::apis::USER_AGENT)
    .query(&params)
    .build()?;

  handle_api_response(client.execute(request)?)
}

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
  use super::{get_my_info, get_user_info};
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

  #[test]
  fn test_get_user_info() {
    let agent = Client::new();
    let cred = get_credential_for_test_or_abort();
    let mid = 12734361i64;
    let res = get_user_info(&agent, &cred, mid);

    assert!(res.is_ok());
  }
}

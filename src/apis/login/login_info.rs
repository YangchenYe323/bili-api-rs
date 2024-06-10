//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/login/login_info.md

use serde::Deserialize;

use crate::{credential::Credential, utils::handle_api_response, Client};

#[derive(Deserialize, Debug)]
pub struct NavResponse {
  pub data: NavData,
}

#[derive(Deserialize, Debug)]
pub struct NavData {
  #[serde(rename = "isLogin")]
  pub is_login: bool,
  pub email_verified: u64,
  pub face: String,
  pub level_info: LevelInfo,
  pub mid: u64,
  #[serde(rename = "mobile_verified")]
  pub mobile_verified: u64,
  pub money: f64,
  pub moral: u64,
  pub official: Official,
  #[serde(rename = "officialVerify")]
  pub official_verify: OfficialVerify,
  pub pendant: Pendant,
  pub scores: u64,
  pub uname: String,
  #[serde(rename = "vipDueDate")]
  pub vip_due_date: u64,
  #[serde(rename = "vipStatus")]
  pub vip_status: u64,
  #[serde(rename = "vipType")]
  pub vip_type: u64,
  #[serde(rename = "vip_pay_type")]
  pub vip_pay_type: u64,
  #[serde(rename = "vip_theme_type")]
  pub vip_theme_type: u64,
  #[serde(rename = "vip_label")]
  pub vip_label: VipLabel,
  #[serde(rename = "vip_avatar_subscript")]
  pub vip_avatar_subscript: u64,
  #[serde(rename = "vip_nickname_color")]
  pub vip_nickname_color: String,
  pub wallet: Wallet,
  #[serde(rename = "has_shop")]
  pub has_shop: bool,
  #[serde(rename = "shop_url")]
  pub shop_url: String,
  #[serde(rename = "allowance_count")]
  pub allowance_count: u64,
  #[serde(rename = "answer_status")]
  pub answer_status: u64,
  #[serde(rename = "is_senior_member")]
  pub is_senior_member: u64,
  #[serde(rename = "wbi_img")]
  pub wbi_img: WbiImg,
  #[serde(rename = "is_jury")]
  pub is_jury: bool,
}

#[derive(Deserialize, Debug)]
pub struct LevelInfo {
  #[serde(rename = "current_level")]
  pub current_level: u64,
  #[serde(rename = "current_min")]
  pub current_min: u64,
  #[serde(rename = "current_exp")]
  pub current_exp: u64,
  #[serde(rename = "next_exp")]
  pub next_exp: serde_json::Value, // Can be num or str
}

#[derive(Deserialize, Debug)]
pub struct Official {
  pub role: i32,
  pub title: String,
  pub desc: String,
  #[serde(rename = "type")]
  pub r#type: i32,
}

#[derive(Deserialize, Debug)]
pub struct OfficialVerify {
  #[serde(rename = "type")]
  pub r#type: i32,
  pub desc: String,
}

#[derive(Deserialize, Debug)]
pub struct Pendant {
  pub pid: u64,
  pub name: String,
  pub image: String,
  pub expire: serde_json::Value, // Placeholder, as no details are provided
}

#[derive(Deserialize, Debug)]
pub struct VipLabel {
  pub path: String,
  pub text: String,
  #[serde(rename = "label_theme")]
  pub label_theme: String,
}

#[derive(Deserialize, Debug)]
pub struct Wallet {
  pub mid: u64,
  #[serde(rename = "bcoin_balance")]
  pub bcoin_balance: f64,
  #[serde(rename = "coupon_balance")]
  pub coupon_balance: f64,
  #[serde(rename = "coupon_due_time")]
  pub coupon_due_time: f64, // Placeholder, as no details are provided
}

#[derive(Deserialize, Debug)]
pub struct WbiImg {
  #[serde(rename = "img_url")]
  pub img_url: String,
  #[serde(rename = "sub_url")]
  pub sub_url: String,
}

pub fn fetch_nav_info(client: &Client, credential: &Credential) -> crate::Result<NavResponse> {
  const API_URL: &str = "https://api.bilibili.com/x/web-interface/nav";

  let request = client
    .get(API_URL)
    .header("cookie", credential.to_cookie_str())
    .header("User-Agent", crate::apis::USER_AGENT)
    .build()?;
  handle_api_response(client.execute(request)?)
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::*;
  use crate::credential::extract_credential::get_credential_for_test_or_abort;
  #[test]
  fn test_fetch_nav_info() {
    let agent = Client::new();
    let cred = get_credential_for_test_or_abort();

    let result = fetch_nav_info(&agent, &cred);

    assert!(result.is_ok());

    let result = result.unwrap();
    let webimg = result.data.wbi_img;
    let img_url = webimg.img_url;
    let sub_url = webimg.sub_url;

    println!("{}", img_url);
    println!("{}", sub_url);

    let img_url = url::Url::parse(&img_url).unwrap();
    let sub_url = url::Url::parse(&sub_url).unwrap();

    println!("{:?}\n{:?}", img_url, sub_url);

    let img_key = Path::new(img_url.path())
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap();
    let sub_key = Path::new(sub_url.path())
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap();

    println!("{}\n{}", img_key, sub_key);
  }
}

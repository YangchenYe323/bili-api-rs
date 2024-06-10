use std::{collections::BTreeMap, path::Path};

use crate::{
  apis::login::login_info::{fetch_nav_info, WbiImg},
  credential::Credential,
  Client,
};

#[derive(Debug)]
pub struct Wbi {
  wts: String,
  w_rid: String,
}

impl Wbi {
  pub fn wts(&self) -> &str {
    &self.wts
  }

  pub fn w_rid(&self) -> &str {
    &self.w_rid
  }
}

/// Perform an API call to the https://api.bilibili.com/x/web-interface/nav endpoint and return both a [WbiImg]
/// for caching and the computed [Wbi] for the current request parameters.
pub fn do_wbi_signature(
  client: &Client,
  credential: &Credential,
  original_params: &BTreeMap<&str, &str>,
) -> crate::Result<(Wbi, WbiImg)> {
  let nav_info = fetch_nav_info(client, credential)?;
  let wbi_img = nav_info.data.wbi_img;

  let wbi = do_wbi_signature_with_wbi_img(&wbi_img, original_params);
  Ok((wbi, wbi_img))
}

/// Perform wbi signature for a given API request with a cached
/// [WbiImg] object
pub fn do_wbi_signature_with_wbi_img(
  wbi_img: &WbiImg,
  original_params: &BTreeMap<&str, &str>,
) -> Wbi {
  let raw_wbi_key = extract_raw_wbi_key(wbi_img);
  let mixin_key = gen_mixin_key(&raw_wbi_key);

  compute_wbi_signature(original_params, &mixin_key)
}

/// Extract img_key and sub_key from img_url and sub_url.
/// E.g.,
/// https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png -> 7cd084941338484aae1ad9425b84077c
///
/// Should only be used on urls that we know satisfy the invariant
fn wbi_extract_file_stem(url: &str) -> Vec<u8> {
  let url = url::Url::parse(url).unwrap();
  let path = Path::new(url.path());
  path.file_stem().unwrap().as_encoded_bytes().to_vec()
}

fn wbi_encode_url_string(params: &BTreeMap<&str, &str>, mixin_key: &str) -> String {
  let mut buffer = String::new();

  // Append query parameters
  let mut first = true;
  for (&k, &v) in params {
    if first {
      buffer += &format!("{}={}", k, urlencoding::encode(v));
      first = false;
    } else {
      buffer += &format!("&{}={}", k, urlencoding::encode(v));
    }
  }

  // Append mixin key
  buffer += mixin_key;

  buffer
}

fn extract_raw_wbi_key(webimg: &WbiImg) -> Vec<u8> {
  let mut img_key = wbi_extract_file_stem(&webimg.img_url);
  let sub_key = wbi_extract_file_stem(&webimg.sub_url);
  img_key.extend(sub_key);
  img_key
}

fn gen_mixin_key(raw_wbi_key: impl AsRef<[u8]>) -> String {
  const MIXIN_KEY_ENC_TAB: [u8; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29,
    28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25,
    54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
  ];
  let raw_wbi_key = raw_wbi_key.as_ref();
  let mut mixin_key = {
    let binding = MIXIN_KEY_ENC_TAB
      .iter()
      // 此步操作即遍历 MIXIN_KEY_ENC_TAB，取出 raw_wbi_key 中对应位置的字符
      .map(|n| raw_wbi_key[*n as usize])
      // 并收集进数组内
      .collect::<Vec<u8>>();
    unsafe { String::from_utf8_unchecked(binding) }
  };
  let _ = mixin_key.split_off(32); // 截取前 32 位字符
  mixin_key
}

fn compute_wbi_signature(original_params: &BTreeMap<&str, &str>, mixin_key: &str) -> Wbi {
  use std::time::SystemTime;
  let mut params = original_params.clone();
  let wts = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("SystemTime before UNIX EPOCH")
    .as_secs();
  let wts = wts.to_string();
  params.insert("wts", &wts);

  let encoded_param_str = wbi_encode_url_string(&params, mixin_key);
  let w_rid = format!("{:02x}", md5::compute(&encoded_param_str));
  Wbi { wts, w_rid }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_mixin_key() {
    let raw_wbi_key = "7cd084941338484aae1ad9425b84077c4932caff0ff746eab6f01bf08b70ac45";
    let mixin_key = gen_mixin_key(raw_wbi_key);
    assert_eq!("ea1db124af3c7062474693fa704f4ff8", &mixin_key);
  }

  #[test]
  fn test_urlencoding_sanity() {
    let space = urlencoding::encode(" ");
    let chinese = urlencoding::encode("中");

    // space should be encoded as %20
    assert_eq!("%20", space);
    // chinese characters should go to uppercase letters
    assert_eq!("%E4%B8%AD", chinese);
  }

  #[test]
  fn test_url_encoding_simple() {
    let mut params = BTreeMap::new();
    params.insert("foo", "114");
    params.insert("bar", "514");
    params.insert("zab", "1919810");
    params.insert("wts", "1702204169");
    let mixin_key = "ea1db124af3c7062474693fa704f4ff8";

    let actual = wbi_encode_url_string(&params, mixin_key);
    assert_eq!(
      "bar=514&foo=114&wts=1702204169&zab=1919810ea1db124af3c7062474693fa704f4ff8",
      actual
    );
  }

  #[test]
  fn test_url_encoding_chinese() {
    let mut params = BTreeMap::new();
    params.insert("foo", "one one four");
    params.insert("bar", "五一四");
    params.insert("baz", "1919810");

    let actual = wbi_encode_url_string(&params, "");
    assert_eq!(
      "bar=%E4%BA%94%E4%B8%80%E5%9B%9B&baz=1919810&foo=one%20one%20four",
      actual
    );
  }

  #[test]
  fn test_wbi_signature() {
    let mut params = BTreeMap::new();
    params.insert("foo", "114");
    params.insert("bar", "514");
    params.insert("zab", "1919810");
    params.insert("wts", "1702204169");
    let mixin_key = "ea1db124af3c7062474693fa704f4ff8";
    let encoded_param_str = wbi_encode_url_string(&params, mixin_key);
    let w_rid = format!("{:02x}", md5::compute(&encoded_param_str));

    assert_eq!("8f6f2b5b3d485fe1886cec6a0be8c5d4", w_rid);
  }
}

//! Utilities
use serde::{de::DeserializeOwned, Deserializer};
use serde_json::Value;

use crate::error::ApiError;

/// Handles Bilibili business logic with regard to response:
/// 1. If code != 0 or message is not a normal success indicator, throw an BiliResponseError
///    It is possible that code == 0 but message indicates failure, for example when user live message is
///    blocked due to blocked word.
/// 2. Otherwise try deserialize to the corresponding type and throw Deserialize Error if failed.
pub fn handle_api_response<T: DeserializeOwned>(
  response: reqwest::blocking::Response,
) -> crate::Result<T> {
  let value: Value = response.json().expect("Response has to be JSON formatted");

  // uncomment this line when debugging :)
  // println!("{}", serde_json::to_string_pretty(&value).unwrap());

  let (code, message) = {
    // We know that the response has to be
    let obj = value.as_object().expect("Response has to be JSON object");
    let code = obj
      .get("code")
      .expect("Response has to contain code field")
      .as_i64()
      .expect("Response code has to be number");
    let message = obj
      .get("message")
      .expect("Response has to contain message field")
      .as_str()
      .expect("Response message has to be string")
      .to_string();
    (code, message)
  };
  match (code, message) {
    (0, _) => serde_json::from_value(value).map_err(crate::Error::from),
    (code, x) => Err(crate::Error::Api(ApiError::new(code as i32, x))),
  }
}

#[cfg(test)]
pub fn assert_error_code<T: std::fmt::Debug>(result: crate::Result<T>, code: i32) {
  match result {
    Err(crate::Error::Api(err)) => {
      assert_eq!(code, err.code());
    }
    it => {
      panic!("Expect Api Error, got {:?}", it);
    }
  }
}

/// A helper utility that deserializes empty object as None
pub fn non_empty_json_obj<'de, D: Deserializer<'de>, T: DeserializeOwned>(
  d: D,
) -> Result<Option<T>, D::Error> {
  use serde::Deserialize;
  let o: Option<Value> = Option::deserialize(d)?;

  let Some(val) = o else {
    return Ok(None);
  };

  if val.is_object() {
    let obj = val.as_object().unwrap();
    if obj.is_empty() {
      return Ok(None);
    }
  }

  match serde_json::from_value(val) {
    Ok(res) => Ok(Some(res)),
    Err(_e) => Ok(None),
  }
}

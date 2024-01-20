use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
  pub sessdata: String,
  pub bili_jct: String,
}

impl Credential {
  pub fn new(sessdata: String, bili_jct: String) -> Self {
    Self { sessdata, bili_jct }
  }

  pub fn to_cookie_str(&self) -> String {
    format!("SESSDATA={}", self.sessdata)
  }
}

#[cfg(test)]
pub mod extract_credential {
  use super::Credential;

  /// This utility function is used for getting a valid user credential in test environment.
  /// Ways to supply credential in decreasing precedence are:
  /// 1. Supply both SESSDATA and BILI_JCT on environment variables, e.g., SESSDATA=xxxx BILI_JCT=xxxx cargo test
  /// 2. Put a cookies.json file of format {sessdata: "", bili_jct: ""} in the project's work directory.
  pub(crate) fn get_credential_for_test_or_abort() -> Credential {
    if let Some(cred) = get_credential_from_env() {
      cred
    } else if let Some(cred) = get_credential_from_file() {
      cred
    } else {
      panic!("Failed to get user credential")
    }
  }

  pub(crate) fn get_fake_credential() -> Credential {
    Credential {
      sessdata: "123".to_string(),
      bili_jct: "456".to_string(),
    }
  }

  fn get_credential_from_env() -> Option<Credential> {
    let sessdata = std::env::var("SESSDATA").ok()?;
    let bili_jct = std::env::var("BILI_JCT").ok()?;
    Some(Credential { sessdata, bili_jct })
  }

  fn get_credential_from_file() -> Option<Credential> {
    let cookie = std::fs::OpenOptions::new()
      .read(true)
      .write(false)
      .create(false)
      .open("./cookies.json")
      .ok()?;
    let cookie = serde_json::from_reader(cookie).ok()?;
    Some(cookie)
  }
}

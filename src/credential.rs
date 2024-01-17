use serde::{Serialize, Deserialize};

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

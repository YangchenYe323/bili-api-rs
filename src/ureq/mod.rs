use crate::{BiliApiRequest, BiliApiResponse, BiliClient};

impl BiliApiRequest for ureq::Request {
    type Response = ureq::Response;

    type Error = ureq::Error;

    fn set_header(self, header: &str, value: &str) -> Self {
        self.set(header, value)
    }

    fn execute(self) -> std::result::Result<Self::Response, Self::Error> {
        self.call()
    }

    fn execute_post_form(self, form: &[(&str, &str)]) -> std::result::Result<Self::Response, Self::Error> {
      self.send_form(form)
    }
}

impl BiliApiResponse for ureq::Response {
    type Error = std::io::Error;

    fn deserialize_json<T: serde::de::DeserializeOwned>(
        self,
    ) -> std::result::Result<T, Self::Error> {
        self.into_json()
    }
}

impl BiliClient for ureq::Agent {
  type Request = ureq::Request;

  fn create_request(&self, method: &str, url: &str) -> Self::Request {
    self.request(method, url)
  }
}

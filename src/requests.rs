use reqwest::header::{self, HeaderMap, HeaderValue};

pub(crate) struct Requester {
    token: String,
}
impl Requester {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
    pub(crate) async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/json")
                .expect("Failed to convert header to header value"),
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))
                .expect("Failed to convert header to header value"),
        );

        let client = reqwest::Client::new();
        match client
            .get(format!("https://the-one-api.dev/v2/{}", url))
            .headers(headers)
            .send()
            .await
        {
            Ok(response) => response.text().await,
            Err(e) => Err(e),
        }
    }
}

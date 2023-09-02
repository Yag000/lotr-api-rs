use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::item::ItemType;

pub struct Sort {
    sort_type: SortOrder,
    sort_by: ItemType,
}

impl Sort {
    pub fn new(sort_type: SortOrder, sort_by: ItemType) -> Self {
        Self { sort_type, sort_by }
    }

    fn get_url(&self) -> String {
        let mut url = String::from("sort=");
        url.push_str(format!("{}:{}", self.sort_by.get_url(), self.sort_type.get_url()).as_str());
        url
    }
}

pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    fn get_url(&self) -> &str {
        match self {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc",
        }
    }
}

pub struct RequestBuilder {
    request: Request,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            request: Request::new(),
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item_type(mut self, item: ItemType) -> Self {
        self.request.item_type = item;
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.request.id = Some(id);
        self
    }

    pub fn second_item(mut self, second_item: ItemType) -> Self {
        self.request.second_item = Some(second_item);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.request.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.request.offset = Some(offset);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.request.page = Some(page);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.request.sort = Some(sort);
        self
    }

    pub fn build(self) -> Request {
        self.request
    }
}

pub struct Request {
    pub(crate) item_type: ItemType,
    id: Option<String>,
    pub(crate) second_item: Option<ItemType>,
    limit: Option<u32>,
    offset: Option<u32>,
    page: Option<u32>,
    sort: Option<Sort>,
}

impl Request {
    fn new() -> Self {
        Self {
            item_type: ItemType::Book,
            id: None,
            second_item: None,
            limit: None,
            offset: None,
            page: None,
            sort: None,
        }
    }
    pub fn get_url(&self) -> String {
        let mut url = self.item_type.get_url().to_string();
        if let Some(id) = &self.id {
            url.push_str(&format!("/{}", id));
        }
        if let Some(second_item) = &self.second_item {
            url.push_str(&format!("/{}", second_item.get_url()));
        }
        if let Some(limit) = self.limit {
            url.push_str(&format!("?limit={}", limit));
        }
        if let Some(offset) = self.offset {
            url.push_str(&format!("?offset={}", offset));
        }
        if let Some(page) = self.page {
            url.push_str(&format!("?page={}", page));
        }
        if let Some(sort) = &self.sort {
            url.push_str(&format!("?{}", sort.get_url()));
        }
        url
    }
}

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

    pub(crate) async fn get_from_request(
        &self,
        request: Request,
    ) -> Result<String, reqwest::Error> {
        let url = request.get_url();
        self.get(&url).await
    }
}

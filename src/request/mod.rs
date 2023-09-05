use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{client::Error, Filter, ItemType, Sort};

use self::pagination::Pagination;

pub mod attributes;
pub mod filter;
pub mod pagination;
pub mod sort;

/// This trait is implemented by all structs that can be used to make a request to the API.
/// It is used to get the url for the request.
pub trait GetUrl {
    /// Returns the url that represents the struct's part of the request.
    fn get_url(&self) -> String;
}

pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new(item_type: ItemType) -> Self {
        Self {
            request: Request::new(item_type),
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.request.id = Some(id);
        self
    }

    pub fn secondary_item_type(mut self, secondary_item_type: ItemType) -> Self {
        self.request.secondary_item_type = Some(secondary_item_type);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Result<Self, Error> {
        if sort.get_item_type() != self.request.get_item_type() {
            return Err(Error::new(
                "The sort attribute must be of the same type as the request".to_string(),
            ));
        }
        self.request.sort = Some(sort);
        Ok(self)
    }

    pub fn filter(mut self, filter: Filter) -> Result<Self, Error> {
        if filter.get_item_type() != self.request.get_item_type() {
            return Err(Error::new(
                "The filter attribute must be of the same type as the request".to_string(),
            ));
        }
        self.request.filter = Some(filter);
        Ok(self)
    }

    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.request.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> Request {
        self.request
    }
}

pub struct Request {
    item_type: ItemType,
    id: Option<String>,
    secondary_item_type: Option<ItemType>,
    sort: Option<Sort>,
    filter: Option<Filter>,
    pagination: Option<Pagination>,
}

impl Request {
    pub(crate) fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            id: None,
            secondary_item_type: None,
            sort: None,
            filter: None,
            pagination: None,
        }
    }

    pub(crate) fn get_item_type(&self) -> ItemType {
        if let Some(secondary_item_type) = &self.secondary_item_type {
            secondary_item_type.clone()
        } else {
            self.item_type.clone()
        }
    }
}

impl GetUrl for Request {
    fn get_url(&self) -> String {
        let mut url = String::from(self.item_type.get_url());
        if let Some(id) = &self.id {
            url.push_str(&format!("/{}", id));
        }
        if let Some(secondary_item_type) = &self.secondary_item_type {
            url.push_str(&format!("/{}", secondary_item_type.get_url()));
        }

        let mut aditional_url = vec![];
        if let Some(sort) = &self.sort {
            aditional_url.push(sort.get_url());
        }
        if let Some(filter) = &self.filter {
            aditional_url.push(filter.get_url());
        }
        if let Some(pagination) = &self.pagination {
            aditional_url.push(pagination.get_url());
        }

        if !aditional_url.is_empty() {
            url.push('?');
            url.push_str(&aditional_url.join("&"));
        }

        url
    }
}

/// Wrapper for the [`reqwest::Client`] struct that contains the token
/// and the actual url that is used to make the request.
/// It is used to make requests to the API.
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
            Ok(response) => {
                let response = response.error_for_status()?;
                response.text().await
            }
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

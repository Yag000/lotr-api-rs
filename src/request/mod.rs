use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{Filter, ItemType, Sort};

use self::pagination::{AddPagination, Pagination};

pub mod filter;
pub mod pagination;
pub mod sort;

pub enum Request {
    Filter(FilterReq),
    Sort(SortReq),
    Get(GetReq),
    /// This corresponds to requests of the type `book/{id}/chapter` etc...
    Specific(SpecificReq),
}

impl Request {
    pub fn get_url(&self) -> String {
        match self {
            Self::Filter(filter_req) => filter_req.get_url(),
            Self::Sort(sort_req) => sort_req.get_url(),
            Self::Get(get_req) => get_req.get_url(),
            Self::Specific(specific_req) => specific_req.get_url(),
        }
    }

    pub(crate) fn get_item_type(&self) -> ItemType {
        match self {
            Self::Filter(filter_req) => filter_req.get_req.item_type.clone(),
            Self::Sort(sort_req) => sort_req.get_req.item_type.clone(),
            Self::Get(get_req) => get_req.item_type.clone(),
            Self::Specific(specific_req) => specific_req.second_item.clone(),
        }
    }
}

pub struct FilterReq {
    get_req: GetReq,
    filter: Filter,
    pagination: Option<Pagination>,
}

impl FilterReq {
    pub fn new(get_req: GetReq, filter: Filter) -> Self {
        Self {
            get_req,
            filter,
            pagination: None,
        }
    }
}

impl AddPagination for FilterReq {
    fn add_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
}

impl FilterReq {
    pub(crate) fn get_url(&self) -> String {
        let mut url = self.get_req.get_url();
        url.push_str(&format!("?{}", self.filter.get_url()));
        if let Some(pagination) = &self.pagination {
            url.push_str(&pagination.get_url());
        }
        url
    }
}

pub struct SortReq {
    get_req: GetReq,
    sort: Sort,
    pagination: Option<Pagination>,
}

impl SortReq {
    pub fn new(sort: Sort) -> Self {
        let item_type: ItemType = sort.sort_by.clone().into();
        let get_req = GetReq::new(item_type);
        Self {
            get_req,
            sort,
            pagination: None,
        }
    }

    /// Create a new [`SortReq`] from a [`GetReq`] and a [`Sort`].
    pub fn from_get_req(get_req: GetReq, sort: Sort) -> Self {
        Self {
            get_req,
            sort,
            pagination: None,
        }
    }
}

impl AddPagination for SortReq {
    fn add_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
}

impl SortReq {
    pub(crate) fn get_url(&self) -> String {
        let mut url = self.get_req.get_url();
        url.push_str(&format!("?{}", self.sort.get_url()));
        if let Some(pagination) = &self.pagination {
            url.push_str(&pagination.get_url());
        }
        url
    }
}

pub struct GetReq {
    item_type: ItemType,
    id: Option<String>,
    pagination: Option<Pagination>,
}

impl AddPagination for GetReq {
    fn add_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
}

impl GetReq {
    pub fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            id: None,
            pagination: None,
        }
    }
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub(crate) fn get_url(&self) -> String {
        let mut url = String::from(self.item_type.get_url());
        if let Some(id) = &self.id {
            url.push_str(&format!("/{}", id));
        }
        if let Some(pagination) = &self.pagination {
            url.push_str(&pagination.get_url());
        }
        url
    }
}

pub struct SpecificReq {
    get_req: GetReq,
    second_item: ItemType,
    pagination: Option<Pagination>,
}

impl SpecificReq {
    pub fn new(get_req: GetReq, second_item: ItemType) -> Self {
        Self {
            get_req,
            second_item,
            pagination: None,
        }
    }

    pub(crate) fn get_url(&self) -> String {
        let mut url = self.get_req.get_url();
        url.push_str(&format!("/{}", self.second_item.get_url()));
        if let Some(pagination) = &self.pagination {
            url.push_str(&pagination.get_url());
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

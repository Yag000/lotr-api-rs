use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{Filter, ItemType, Sort};

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

/// This is a builder for [`Request`], it will allow you to build a [`Request`]
/// with the desired parameters, and then call the `build` function to get the [`Request`]
/// with the parameters you set.
///
/// It allows you to have a more rich and complex request that the defaults provided
/// by the Client struct.
///
/// # Example
/// ```
/// use lotr_api_wrapper::{RequestBuilder, ItemType, Attribute, SortOrder, Sort, CharacterAttribute};
///
/// let request = RequestBuilder::new()
///    .item_type(ItemType::Book)
///    .sort(Sort::new(SortOrder::Ascending, Attribute::Character(CharacterAttribute::Hair)))
///    .limit(10)
///    .page(2)
///    .offset(5)
///    .build();
///
/// assert_eq!(request.get_url(), "character?limit=10?page=2?offset=5?sort=hair:asc");
/// ```
impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the item type for the request, it will override any previous
    /// item type unless a sort is set, in which case nothing happens
    /// since the sort item type will override the item type.
    /// This allows us to always have a valid request.
    ///
    /// # Example
    /// ```
    /// use lotr_api_wrapper::{RequestBuilder, ItemType, Attribute, BookAttribute, Sort, SortOrder};
    ///
    /// let request = RequestBuilder::new()
    ///   .item_type(ItemType::Book)
    ///   .build();
    ///
    /// let request2 = RequestBuilder::new()
    ///  .item_type(ItemType::Character)
    ///  .sort(Sort::new(SortOrder::Ascending, Attribute::Book(BookAttribute::Name)))
    ///  .item_type(ItemType::Character) // Even with this line, the item type will still be book
    ///  .build();
    ///
    ///
    ///  assert_eq!(request.get_url(), "book");
    ///  assert_eq!(request2.get_url(), "book?sort=name:asc");
    pub fn item_type(mut self, item: ItemType) -> Self {
        if self.request.sort.is_none() {
            self.request.item_type = item;
        }
        self
    }

    // Used to set the id when requesting for an specific item
    pub fn id(mut self, id: String) -> Self {
        self.request.id = Some(id);
        self
    }

    /// Used when creating requests about another type :
    /// For example:
    ///
    /// ```
    /// use lotr_api_wrapper::{RequestBuilder, ItemType};
    ///
    /// let request = RequestBuilder::new()
    ///     .item_type(ItemType::Character)
    ///     .id("a_character_id".to_string())
    ///     .second_item(ItemType::Quote)
    ///     .build();
    ///
    /// assert_eq!(request.get_url(), "character/a_character_id/quote")
    /// ```
    ///
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

    /// Adds a sort to the request, it will override any previous
    /// item type in order to guarantee that the request is valid
    pub fn sort(mut self, sort: Sort) -> Self {
        let item_type: ItemType = sort.sort_by.clone().into();
        self.request.item_type = item_type;
        self.request.sort = Some(sort);
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        //TODO: add some logic similar to sort, maybe introduce results
        self.request.filter = Some(filter);
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
    filter: Option<Filter>,
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
            filter: None,
        }
    }
    pub fn get_url(&self) -> String {
        let mut url;
        if let Some(sort) = &self.sort {
            let item_type: ItemType = sort.sort_by.clone().into();
            url = String::from(item_type.get_url());
        } else {
            url = String::from(self.item_type.get_url());
        }

        if let Some(id) = &self.id {
            url.push_str(&format!("/{}", id));
        }
        if let Some(second_item) = &self.second_item {
            url.push_str(&format!("/{}", second_item.get_url()));
        }
        if let Some(limit) = self.limit {
            url.push_str(&format!("?limit={}", limit));
        }
        if let Some(page) = self.page {
            url.push_str(&format!("?page={}", page));
        }
        if let Some(offset) = self.offset {
            url.push_str(&format!("?offset={}", offset));
        }
        if let Some(sort) = &self.sort {
            url.push_str(&format!("?{}", sort.get_url()));
        }
        if let Some(filter) = &self.filter {
            url.push_str(&format!("?{}", filter.get_url()));
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

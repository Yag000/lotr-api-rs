//! This module contains the structs that are used to make a request to the API.
//! Here we define the [`Request`] struct and the [`RequestBuilder`] struct, which
//! are the center of the custom request system.

use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{Error, ItemType};

use self::{filter::Filter, pagination::Pagination, sort::Sort};

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

/// This struct is used to build a [`Request`].
///
/// # Example
///
/// ```
/// use lotr_api::{Request, RequestBuilder, ItemType,
///     sort::{Sort, SortOrder},
///     request::GetUrl,
///     attribute::{Attribute, BookAttribute}};
///
/// let request = RequestBuilder::new(ItemType::Book)
///    .sort(Sort::new(
///    SortOrder::Ascending,
///    Attribute::Book(BookAttribute::Name),
///    ))
///    .build()
///    .unwrap();
///
/// assert_eq!(request.get_url(), "book?sort=name:asc");
/// ```
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new(item_type: ItemType) -> Self {
        Self {
            request: Request::new(item_type),
        }
    }

    /// Sets the id of the request. This is used to get a specific item.
    pub fn id(mut self, id: String) -> Self {
        self.request.id = Some(id);
        self
    }

    /// Sets the secondary item type of the request. If you wish
    /// to get a secondary item type, you need to set the id of the request.\
    /// If not the `build` function will return an error.
    ///
    /// # Example
    /// ```
    /// use lotr_api::{ItemType, Request, RequestBuilder,
    ///     request::GetUrl};
    ///
    /// let request = RequestBuilder::new(ItemType::Character)
    ///     .id("123".to_string())
    ///     .secondary_item_type(ItemType::Quote)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(request.get_url(), "character/123/quote");
    ///   ```
    ///
    pub fn secondary_item_type(mut self, secondary_item_type: ItemType) -> Self {
        self.request.secondary_item_type = Some(secondary_item_type);
        self
    }

    /// Sets the sort of the request. If you wish to sort the results
    /// of the request, the `sort_by` attribute of the `Sort` struct
    /// must be of the same type as the item type of the request ( or the
    /// secondary item type if it is set).
    ///
    /// # Example
    /// ```
    /// use lotr_api::{ItemType, Request, RequestBuilder,
    ///     attribute::{Attribute, BookAttribute},
    ///     request::GetUrl,
    ///     sort::{Sort, SortOrder}};
    ///
    /// let request = RequestBuilder::new(ItemType::Book)
    ///     .sort(Sort::new(
    ///         SortOrder::Ascending,
    ///         Attribute::Book(BookAttribute::Name),
    ///     ))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(request.get_url(), "book?sort=name:asc");
    /// ```
    /// Failing to match the item type of the request results in an error.
    /// ```
    /// use lotr_api::{ItemType, Request, RequestBuilder,
    ///     attribute::{Attribute, BookAttribute},
    ///     request::GetUrl,
    ///     sort::{ Sort, SortOrder}};
    ///
    ///  let request = RequestBuilder::new(ItemType::Character)
    ///     .id("123".to_string())
    ///     .secondary_item_type(ItemType::Quote)
    ///     .sort(Sort::new(
    ///         SortOrder::Ascending,
    ///         Attribute::Book(BookAttribute::Name),
    ///     ))
    ///     .build();
    ///
    /// assert!(request.is_err());
    /// ```
    ///
    pub fn sort(mut self, sort: Sort) -> Self {
        self.request.sort = Some(sort);
        self
    }

    /// Sets the filter of the request. If you wish to filter the results
    /// of the request, the `filter_by` attribute of the `Filter` struct
    /// must be of the same type as the item type of the request ( or the
    /// secondary item type if it is set).
    ///
    /// # Example
    /// ```
    /// use lotr_api::{ItemType, Request, RequestBuilder,
    ///     attribute::{Attribute, BookAttribute},
    ///     request::GetUrl,
    ///     filter::{Filter, Operator}};
    ///
    /// let request = RequestBuilder::new(ItemType::Book)
    ///     .filter(Filter::Match(
    ///         Attribute::Book(BookAttribute::Name),
    ///         Operator::Eq,
    ///         vec!["The Fellowship of the Ring".to_string()],
    ///     ))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(request.get_url(), "book?name=The Fellowship of the Ring");
    /// ```
    ///
    /// Failing to match the item type of the request results in an error.
    ///
    /// ```
    /// use lotr_api::{ItemType, Request, RequestBuilder,
    ///     attribute::{Attribute, BookAttribute},
    ///     request::GetUrl,
    ///     filter::{Filter, Operator}};
    ///
    /// let request = RequestBuilder::new(ItemType::Character)
    ///     .id("123".to_string())
    ///     .secondary_item_type(ItemType::Quote)
    ///     .filter(Filter::Match(
    ///         Attribute::Book(BookAttribute::Name),
    ///         Operator::Eq,
    ///         vec!["The Fellowship of the Ring".to_string()],
    ///     ))
    ///     .build();
    ///
    /// assert!(request.is_err());
    /// ```
    pub fn filter(mut self, filter: Filter) -> Self {
        self.request.filter = Some(filter);
        self
    }

    /// Sets the pagination of the request.
    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.request.pagination = Some(pagination);
        self
    }

    /// Builds the request. If the request is invalid, an error is returned.
    ///
    /// # Errors
    ///
    /// A request is invalid if:
    /// - The secondary item type is set but the id is not.
    /// - The sort is set but the item type of the sort does not match the item type of the request.
    /// - The filter is set but the item type of the filter does not match the item type of the request.
    pub fn build(self) -> Result<Request, Error> {
        let item_type = self.request.get_item_type();
        if let Some(sort) = &self.request.sort {
            if sort.get_item_type() != item_type {
                return Err(Error::InvalidSort);
            }
        }
        if let Some(filter) = &self.request.filter {
            if filter.get_item_type() != item_type {
                return Err(Error::InvalidFilter);
            }
        }
        // Every secondary item type needs an id.
        if self.request.secondary_item_type.is_some() && self.request.id.is_none() {
            return Err(Error::InvalidSecondaryItemType);
        }

        Ok(self.request)
    }
}

/// This struct represents a request to the API.
/// It should be created with the [`RequestBuilder`].
#[derive(Debug, Clone, PartialEq, Eq)]
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
        let mut url = self.item_type.get_url();
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

#[cfg(test)]
mod tests {

    use crate::{
        attribute::{Attribute, BookAttribute, QuoteAttribute},
        filter::Operator,
        request::sort::SortOrder,
    };

    use super::*;

    #[test]
    fn test_simple_request_url() {
        let request = RequestBuilder::new(ItemType::Book).build().unwrap();
        assert_eq!(request.get_url(), "book");
    }

    #[test]
    fn test_request_with_id_url() {
        let request = RequestBuilder::new(ItemType::Book)
            .id("123".to_string())
            .build()
            .unwrap();
        assert_eq!(request.get_url(), "book/123");
    }

    #[test]
    fn test_request_with_secondary_item_type_url() {
        let request = RequestBuilder::new(ItemType::Book)
            .secondary_item_type(ItemType::Chapter)
            .build();
        assert!(request.is_err());

        let request = RequestBuilder::new(ItemType::Character)
            .id("123".to_string())
            .secondary_item_type(ItemType::Quote)
            .build()
            .unwrap();

        assert_eq!(request.get_url(), "character/123/quote");
    }

    #[test]
    fn test_request_with_sort_url() {
        let request = RequestBuilder::new(ItemType::Book)
            .sort(Sort::new(
                SortOrder::Ascending,
                Attribute::Book(BookAttribute::Name),
            ))
            .build()
            .unwrap();

        assert_eq!(request.get_url(), "book?sort=name:asc");
    }

    #[test]
    fn test_request_with_filter_url() {
        let request = RequestBuilder::new(ItemType::Book)
            .filter(Filter::Match(
                Attribute::Book(BookAttribute::Name),
                Operator::Eq,
                vec!["The Fellowship of the Ring".to_string()],
            ))
            .build()
            .unwrap();

        assert_eq!(request.get_url(), "book?name=The Fellowship of the Ring");
    }

    #[test]
    fn test_request_with_pagination_url() {
        let request = RequestBuilder::new(ItemType::Book)
            .pagination(Pagination::new(10, 10, 2))
            .build()
            .unwrap();

        assert_eq!(request.get_url(), "book?limit=10&offset=10&page=2");
    }

    #[test]
    fn test_full_request_url() {
        let request = RequestBuilder::new(ItemType::Character)
            .id("123".to_string())
            .secondary_item_type(ItemType::Quote)
            .sort(Sort::new(
                SortOrder::Ascending,
                Attribute::Quote(QuoteAttribute::Dialog),
            ))
            .filter(Filter::Match(
                Attribute::Quote(QuoteAttribute::Dialog),
                Operator::Eq,
                vec!["Deagol!".to_string()],
            ))
            .pagination(Pagination::new(10, 10, 2))
            .build()
            .unwrap();

        assert_eq!(
            request.get_url(),
            "character/123/quote?sort=dialog:asc&dialog=Deagol!&limit=10&offset=10&page=2"
        );
    }
}

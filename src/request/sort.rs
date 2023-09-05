use crate::attribute::Attribute;

use super::GetUrl;

/// This struct contains the data for the sorting of the API.
///
/// # Example
/// ```
/// use lotr_api::{
///     attribute::{Attribute, BookAttribute},
///     sort::{Sort, SortOrder},
///     request::{GetUrl}};
///
/// let sort = Sort::new(SortOrder::Ascending, Attribute::Book(BookAttribute::Name));
/// assert_eq!(sort.get_url(), "sort=name:asc");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sort {
    pub(crate) sort_type: SortOrder,
    pub(crate) sort_by: Attribute,
}

impl GetUrl for Sort {
    fn get_url(&self) -> String {
        let mut url = String::from("sort=");
        url.push_str(format!("{}:{}", self.sort_by.get_url(), self.sort_type.get_url()).as_str());
        url
    }
}

impl Sort {
    pub fn new(sort_type: SortOrder, sort_by: Attribute) -> Self {
        Self { sort_type, sort_by }
    }

    pub(crate) fn get_item_type(&self) -> crate::ItemType {
        self.sort_by.get_item_type()
    }
}

/// Define the sort order.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl GetUrl for SortOrder {
    fn get_url(&self) -> String {
        match self {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc",
        }
        .to_string()
    }
}

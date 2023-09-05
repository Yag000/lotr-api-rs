/// This struct contains the date for the pagination of the API.
///
/// # Example
///
/// ```
/// use lotr_api_wrapper::{GetUrl, request::pagination::Pagination};
///
/// let pagination = Pagination::new(10, 2, 1);
///
/// assert_eq!(pagination.get_url(), "limit=10&offset=2&page=1");
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pagination {
    limit: u32,
    offset: u32,
    page: u32,
}

impl Pagination {
    pub fn new(limit: u32, offset: u32, page: u32) -> Self {
        Self {
            limit,
            offset,
            page,
        }
    }

    pub fn get_url(&self) -> String {
        let mut values = vec![];

        if self.limit != 0 {
            values.push(format!("limit={}", self.limit));
        }
        if self.offset != 0 {
            values.push(format!("offset={}", self.offset));
        }
        if self.page != 0 {
            values.push(format!("page={}", self.page));
        }

        if values.is_empty() {
            String::new()
        } else {
            values.join("&")
        }
    }
}

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
        let mut url = String::new();
        if self.limit != 0 {
            url.push_str(&format!("?limit={}", self.limit));
        }
        if self.offset != 0 {
            url.push_str(&format!("?offset={}", self.offset));
        }
        if self.page != 0 {
            url.push_str(&format!("?page={}", self.page));
        }
        url
    }
}

pub trait AddPagination {
    fn add_pagination(self, pagination: Pagination) -> Self;
}

use crate::attribute::Attribute;

pub struct Sort {
    pub(crate) sort_type: SortOrder,
    pub(crate) sort_by: Attribute,
}

impl Sort {
    pub fn new(sort_type: SortOrder, sort_by: Attribute) -> Self {
        Self { sort_type, sort_by }
    }

    pub(crate) fn get_url(&self) -> String {
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

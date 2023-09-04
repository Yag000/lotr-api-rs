use crate::Attribute;

/// A filter that can be used to filter the results of a request.
///
/// # Examples
/// ```
/// use lotr_api_wrapper::{Filter, Attribute, Operation, ItemType, RequestBuilder, BookAttribute};
/// let filter = Filter::Match(Attribute::Book(BookAttribute::Name),Operation::Eq, vec!["The Fellowship of the Ring".to_string()]);
///
/// let request = RequestBuilder::new()
///     .item_type(ItemType::Book)
///     .filter(filter)
///     .build();
///
/// assert_eq!(request.get_url(), "book?name=The Fellowship of the Ring");
/// ```
pub enum Filter {
    Match(Attribute, Operation, Vec<String>),
    Exists(String, bool),
}

impl Filter {
    pub(crate) fn get_url(&self) -> String {
        match self {
            Filter::Match(attribute, operation, values) => {
                let mut url = String::from(attribute.get_url());
                url.push_str(operation.get_url());
                url.push_str(&values.join(","));
                url
            }
            Filter::Exists(attribute, exists) => {
                let mut url = String::from(attribute);
                url.push_str("=");
                if !exists {
                    url.push_str("?");
                }
                url.push_str(&exists.to_string());
                url
            }
        }
    }
}

pub enum Operation {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl Operation {
    fn get_url(&self) -> &str {
        match self {
            Operation::Eq => "=",
            Operation::Ne => "!=",
            Operation::Gt => ">",
            Operation::Lt => "<",
            Operation::Gte => ">=",
            Operation::Lte => "<=",
        }
    }
}

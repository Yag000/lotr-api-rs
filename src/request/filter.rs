use crate::attribute::Attribute;

/// A filter that can be used to filter the results of a request.
///
/// # Examples
/// ```
/// use lotr_api_wrapper::{Filter,  Operator, ItemType, RequestBuilder, attribute::{Attribute, BookAttribute}};
/// let filter = Filter::Match(Attribute::Book(BookAttribute::Name),Operator::Eq, vec!["The Fellowship of the Ring".to_string()]);
///
/// let request = RequestBuilder::new()
///     .item_type(ItemType::Book)
///     .filter(filter)
///     .build();
///
/// assert_eq!(request.get_url(), "book?name=The Fellowship of the Ring");
/// ```
pub enum Filter {
    Match(Attribute, Operator, Vec<String>),
    Exists(String, bool),
}

impl Filter {
    pub(crate) fn get_url(&self) -> String {
        match self {
            Filter::Match(attribute, operation, values) => {
                let mut url = attribute.get_url();
                url.push_str(operation.get_url());
                url.push_str(&values.join(","));
                url
            }
            Filter::Exists(attribute, exists) => {
                let mut url = String::new();
                if !exists {
                    url.push('!');
                }
                url.push_str(attribute);
                url
            }
        }
    }
}

pub enum Operator {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl Operator {
    fn get_url(&self) -> &str {
        match self {
            Operator::Eq => "=",
            Operator::Ne => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attribute::{Attribute, BookAttribute, MovieAttribute},
        Filter, Operator,
    };

    #[test]
    fn test_match_eq() {
        let filter_eq = Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Eq,
            vec!["The Fellowship of the Ring".to_string()],
        );
        assert_eq!(
            filter_eq.get_url(),
            "name=The Fellowship of the Ring".to_string()
        );
    }

    #[test]
    fn test_match_ne() {
        let filter_ne = Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Ne,
            vec!["The Fellowship of the Ring".to_string()],
        );
        assert_eq!(
            filter_ne.get_url(),
            "name!=The Fellowship of the Ring".to_string()
        );
    }

    #[test]
    fn test_exists() {
        let filter_include = Filter::Exists("name".to_string(), true);
        assert_eq!(filter_include.get_url(), "name".to_string());
    }

    #[test]
    fn test_dont_exist() {
        let filter_exclude = Filter::Exists("name".to_string(), false);
        assert_eq!(filter_exclude.get_url(), "!name".to_string());
    }

    #[test]
    fn test_include_and_exclude() {
        let filter = Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Eq,
            vec![
                "The Fellowship Of The Ring".to_string(),
                "The Two Towers".to_string(),
                "The Return Of The King".to_string(),
            ],
        );

        assert_eq!(
            filter.get_url(),
            "name=The Fellowship Of The Ring,The Two Towers,The Return Of The King".to_string()
        );

        let filter = Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Ne,
            vec![
                "The Fellowship Of The Ring".to_string(),
                "The Two Towers".to_string(),
                "The Return Of The King".to_string(),
            ],
        );

        assert_eq!(
            filter.get_url(),
            "name!=The Fellowship Of The Ring,The Two Towers,The Return Of The King".to_string()
        );
    }

    #[test]
    fn test_operations() {
        let tests = vec![
            (
                Filter::Match(
                    Attribute::Movie(MovieAttribute::BudgetInMillions),
                    Operator::Gt,
                    vec!["10".to_string()],
                ),
                "budgetInMillions>10",
            ),
            (
                Filter::Match(
                    Attribute::Movie(MovieAttribute::BudgetInMillions),
                    Operator::Gte,
                    vec!["10".to_string()],
                ),
                "budgetInMillions>=10",
            ),
            (
                Filter::Match(
                    Attribute::Movie(MovieAttribute::BudgetInMillions),
                    Operator::Lt,
                    vec!["10".to_string()],
                ),
                "budgetInMillions<10",
            ),
            (
                Filter::Match(
                    Attribute::Movie(MovieAttribute::BudgetInMillions),
                    Operator::Lte,
                    vec!["10".to_string()],
                ),
                "budgetInMillions<=10",
            ),
        ];

        for (filter, expected) in tests {
            assert_eq!(filter.get_url(), expected.to_string());
        }
    }
}

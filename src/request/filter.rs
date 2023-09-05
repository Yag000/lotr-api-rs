use crate::{attribute::Attribute, ItemType};

use super::GetUrl;

/// A filter that can be used to filter the results of a request.
///
/// # Examples
/// ```
/// use lotr_api_wrapper::{Filter,  Operator, GetUrl,
///     attribute::{Attribute, BookAttribute}};
///
///
/// let filter = Filter::Match(
///     Attribute::Book(BookAttribute::Name),
///     Operator::Eq,
///     vec!["The Fellowship of the Ring".to_string()]);
///
/// assert_eq!(filter.get_url(), "name=The Fellowship of the Ring");
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    Match(Attribute, Operator, Vec<String>),
    Exists(Attribute, bool),
}

impl GetUrl for Filter {
    fn get_url(&self) -> String {
        match self {
            Filter::Match(attribute, operation, values) => {
                let mut url = attribute.get_url();
                url.push_str(&operation.get_url());
                url.push_str(&values.join(","));
                url
            }
            Filter::Exists(attribute, exists) => {
                let mut url = String::new();
                if !exists {
                    url.push('!');
                }
                url.push_str(&attribute.get_url());
                url
            }
        }
    }
}

impl Filter {
    pub(crate) fn get_item_type(&self) -> ItemType {
        match self {
            Filter::Match(attribute, _, _) => attribute.get_item_type(),
            Filter::Exists(attribute, _) => attribute.get_item_type(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl GetUrl for Operator {
    fn get_url(&self) -> String {
        match self {
            Operator::Eq => "=",
            Operator::Ne => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::attribute::{Attribute, BookAttribute, CharacterAttribute, MovieAttribute};

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
        let filter_include = Filter::Exists(Attribute::Character(CharacterAttribute::Name), true);
        assert_eq!(filter_include.get_url(), "name".to_string());
    }

    #[test]
    fn test_dont_exist() {
        let filter_exclude = Filter::Exists(Attribute::Character(CharacterAttribute::Name), false);
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

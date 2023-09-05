use crate::ItemType;

/// The different attributes that can be used to sort the different items that can be retrieved
/// from the API. The contain all the attributes that are available for the different items.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attribute {
    Book(BookAttribute),
    Movie(MovieAttribute),
    Quote(QuoteAttribute),
    Character(CharacterAttribute),
    Chapter(ChapterAttribute),
}

impl From<Attribute> for ItemType {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::Book(_) => ItemType::Book,
            Attribute::Movie(_) => ItemType::Movie,
            Attribute::Quote(_) => ItemType::Quote,
            Attribute::Character(_) => ItemType::Character,
            Attribute::Chapter(_) => ItemType::Chapter,
        }
    }
}

impl Attribute {
    pub(crate) fn get_item_type(&self) -> ItemType {
        match self {
            Attribute::Book(_) => ItemType::Book,
            Attribute::Movie(_) => ItemType::Movie,
            Attribute::Quote(_) => ItemType::Quote,
            Attribute::Character(_) => ItemType::Character,
            Attribute::Chapter(_) => ItemType::Chapter,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BookAttribute {
    Id,
    Name,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MovieAttribute {
    Id,
    Name,
    RuntimeInMinutes,
    BudgetInMillions,
    BoxOfficeRevenueInMillions,
    AcademyAwardNominations,
    AcademyAwardWins,
    RottenTomatoesScore,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum QuoteAttribute {
    Id,
    Dialog,
    Movie,
    Character,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CharacterAttribute {
    Id,
    Height,
    Gender,
    Birth,
    Spouse,
    Death,
    Realm,
    Hair,
    Name,
    WikiUrl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChapterAttribute {
    Id,
    ChapterName,
    Book,
}

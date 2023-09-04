use crate::ItemType;

/// The different attributes that can be used to sort the different items that can be retrieved
/// from the API. The contain all the attributes that are available for the different items.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Returns the url (in this case it corresponds to the name of the attribute) for the attribute.
    pub fn get_url(&self) -> String {
        match self {
            Self::Book(attribute) => attribute.get_url(),
            Self::Movie(attribute) => attribute.get_url(),
            Self::Quote(attribute) => attribute.get_url(),
            Self::Character(attribute) => attribute.get_url(),
            Self::Chapter(attribute) => attribute.get_url(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookAttribute {
    Id,
    Name,
}

impl BookAttribute {
    pub fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Name => "name",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl MovieAttribute {
    pub fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Name => "name",
            Self::RuntimeInMinutes => "runtimeInMinutes",
            Self::BudgetInMillions => "budgetInMillions",
            Self::BoxOfficeRevenueInMillions => "boxOfficeRevenueInMillions",
            Self::AcademyAwardNominations => "academyAwardNominations",
            Self::AcademyAwardWins => "academyAwardWins",
            Self::RottenTomatoesScore => "rottenTomatoesScore",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuoteAttribute {
    Id,
    Dialog,
    Movie,
    Character,
}

impl QuoteAttribute {
    pub fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Dialog => "dialog",
            Self::Movie => "movie",
            Self::Character => "character",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl CharacterAttribute {
    pub fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Height => "height",
            Self::Gender => "gender",
            Self::Birth => "birth",
            Self::Spouse => "spouse",
            Self::Death => "death",
            Self::Realm => "realm",
            Self::Hair => "hair",
            Self::Name => "name",
            Self::WikiUrl => "wikiUrl",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChapterAttribute {
    Id,
    ChapterName,
    Book,
}

impl ChapterAttribute {
    pub fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::ChapterName => "chapterName",
            Self::Book => "book",
        }
        .to_string()
    }
}

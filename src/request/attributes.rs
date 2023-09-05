use crate::attribute::{
    Attribute, BookAttribute, ChapterAttribute, CharacterAttribute, MovieAttribute, QuoteAttribute,
};

use super::GetUrl;

impl GetUrl for Attribute {
    /// Returns the url (in this case it corresponds to the name of the attribute) for the attribute.
    fn get_url(&self) -> String {
        match self {
            Self::Book(attribute) => attribute.get_url(),
            Self::Movie(attribute) => attribute.get_url(),
            Self::Quote(attribute) => attribute.get_url(),
            Self::Character(attribute) => attribute.get_url(),
            Self::Chapter(attribute) => attribute.get_url(),
        }
    }
}

impl GetUrl for BookAttribute {
    fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Name => "name",
        }
        .to_string()
    }
}

impl GetUrl for MovieAttribute {
    fn get_url(&self) -> String {
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

impl GetUrl for QuoteAttribute {
    fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::Dialog => "dialog",
            Self::Movie => "movie",
            Self::Character => "character",
        }
        .to_string()
    }
}

impl GetUrl for CharacterAttribute {
    fn get_url(&self) -> String {
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

impl GetUrl for ChapterAttribute {
    fn get_url(&self) -> String {
        match self {
            Self::Id => "_id",
            Self::ChapterName => "chapterName",
            Self::Book => "book",
        }
        .to_string()
    }
}

//! This module contains the data structures for the items that are returned by the API.
//! It also holds the [`attribute::Attribute`] enum and its derivatives, that contain the attributes
//! that represent the fields of the items ( they are used for filtering and sorting ).

use self::object::{Book, Chapter, Character, Movie, Quote};

pub mod attribute;
pub mod object;

/// The different types of items that can be retrieved from the API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Book,
    Movie,
    Quote,
    Character,
    Chapter,
}

impl From<&str> for ItemType {
    fn from(value: &str) -> Self {
        match value {
            "book" => ItemType::Book,
            "movie" => ItemType::Movie,
            "quote" => ItemType::Quote,
            "character" => ItemType::Character,
            "chapter" => ItemType::Chapter,
            _ => panic!("Invalid item type"),
        }
    }
}

/// The different items that can be retrieved from the API.
/// They are all wrapped in this enum, so that they can be used in the same vector.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Book(Book),
    Movie(Movie),
    Quote(Quote),
    Character(Character),
    Chapter(Chapter),
}

impl From<Book> for Item {
    fn from(book: Book) -> Self {
        Item::Book(book)
    }
}

impl TryInto<Book> for Item {
    type Error = ();

    fn try_into(self) -> Result<Book, Self::Error> {
        match self {
            Item::Book(book) => Ok(book),
            _ => Err(()),
        }
    }
}

impl From<Movie> for Item {
    fn from(movie: Movie) -> Self {
        Item::Movie(movie)
    }
}

impl TryInto<Movie> for Item {
    type Error = ();

    fn try_into(self) -> Result<Movie, Self::Error> {
        match self {
            Item::Movie(movie) => Ok(movie),
            _ => Err(()),
        }
    }
}

impl From<Quote> for Item {
    fn from(quote: Quote) -> Self {
        Item::Quote(quote)
    }
}

impl TryInto<Quote> for Item {
    type Error = ();

    fn try_into(self) -> Result<Quote, Self::Error> {
        match self {
            Item::Quote(quote) => Ok(quote),
            _ => Err(()),
        }
    }
}

impl From<Character> for Item {
    fn from(character: Character) -> Self {
        Item::Character(character)
    }
}

impl TryInto<Character> for Item {
    type Error = ();

    fn try_into(self) -> Result<Character, Self::Error> {
        match self {
            Item::Character(character) => Ok(character),
            _ => Err(()),
        }
    }
}

impl From<Chapter> for Item {
    fn from(chapter: Chapter) -> Self {
        Item::Chapter(chapter)
    }
}

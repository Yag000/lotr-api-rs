use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Book,
    Movie,
    Quote,
    Character,
    Chapter,
}

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

impl From<Movie> for Item {
    fn from(movie: Movie) -> Self {
        Item::Movie(movie)
    }
}

impl From<Quote> for Item {
    fn from(quote: Quote) -> Self {
        Item::Quote(quote)
    }
}

impl From<Character> for Item {
    fn from(character: Character) -> Self {
        Item::Character(character)
    }
}

impl From<Chapter> for Item {
    fn from(chapter: Chapter) -> Self {
        Item::Chapter(chapter)
    }
}

impl ItemType {
    pub fn get_url(&self) -> &str {
        match self {
            ItemType::Book => "book",
            ItemType::Movie => "movie",
            ItemType::Quote => "quote",
            ItemType::Character => "character",
            ItemType::Chapter => "chapter",
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(dead_code)]
pub(crate) struct Response<T> {
    docs: Vec<T>,
    total: u32,
    limit: u32,
    offset: u32,
    page: u32,
    pages: u32,
}

impl From<Response<Book>> for Vec<Item> {
    fn from(response: Response<Book>) -> Self {
        response.docs.into_iter().map(Item::from).collect()
    }
}

impl From<Response<Movie>> for Vec<Item> {
    fn from(response: Response<Movie>) -> Self {
        response.docs.into_iter().map(Item::from).collect()
    }
}

impl From<Response<Quote>> for Vec<Item> {
    fn from(response: Response<Quote>) -> Self {
        response.docs.into_iter().map(Item::from).collect()
    }
}

impl From<Response<Character>> for Vec<Item> {
    fn from(response: Response<Character>) -> Self {
        response.docs.into_iter().map(Item::from).collect()
    }
}

impl From<Response<Chapter>> for Vec<Item> {
    fn from(response: Response<Chapter>) -> Self {
        response.docs.into_iter().map(Item::from).collect()
    }
}

impl<T> Response<T> {
    pub fn get_contents(self) -> Vec<T> {
        self.docs
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub _id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub _id: String,
    pub name: String,

    #[serde(rename = "runtimeInMinutes")]
    pub runtime_in_minutes: f32,

    #[serde(rename = "budgetInMillions")]
    pub budget_in_millions: f32,

    #[serde(rename = "boxOfficeRevenueInMillions")]
    pub box_office_revenue_in_millions: f32,

    #[serde(rename = "academyAwardNominations")]
    pub academy_award_nominations: u32,

    #[serde(rename = "academyAwardWins")]
    pub academy_award_wins: u32,

    #[serde(rename = "rottenTomatoesScore")]
    pub rotten_tomates_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub _id: String,
    pub dialog: String,
    pub movie: String,
    pub character: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub _id: String,
    pub height: Option<String>,
    pub gender: Option<String>,
    pub birth: Option<String>,
    pub spouse: Option<String>,
    pub death: Option<String>,
    pub realm: Option<String>,
    pub hair: Option<String>,
    pub name: String,

    #[serde(rename = "wikiUrl")]
    pub wiki_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub _id: String,

    #[serde(rename = "chapterName")]
    pub chapter_name: String,

    pub book: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_movie_deserialize() {
        let tests = vec![
            r#"
    {
      "_id": "5cd95395de30eff6ebccde57",
      "name": "The Hobbit Series",
      "runtimeInMinutes": 462,
      "budgetInMillions": 675,
      "boxOfficeRevenueInMillions": 2932,
      "academyAwardNominations": 7,
      "academyAwardWins": 1,
      "rottenTomatoesScore": 66
    }"#,
            r#"
    {
      "_id": "5cd95395de30eff6ebccde57",
      "name": "The Hobbit Series",
      "runtimeInMinutes": 462.1,
      "budgetInMillions": 675.23,
      "boxOfficeRevenueInMillions": 2932.31,
      "academyAwardNominations": 7,
      "academyAwardWins": 1,
      "rottenTomatoesScore": 66.33333333
    }"#,
        ];
        for json in tests {
            println!("{}", json);
            serde_json::from_str::<Movie>(json).unwrap();
        }
    }
}

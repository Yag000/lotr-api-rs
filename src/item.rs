use serde::Deserialize;

pub enum Item {
    Book,
    Movie,
    Quote,
    Character,
    Chapter,
}

impl Item {
    pub fn get_url(&self) -> &str {
        match self {
            Item::Book => "book",
            Item::Movie => "movie",
            Item::Quote => "quote",
            Item::Character => "character",
            Item::Chapter => "chapter",
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Response<T> {
    docs: Vec<T>,
    total: u32,
    limit: u32,
    offset: u32,
    page: u32,
    pages: u32,
}

impl<T> Response<T> {
    pub fn get_contents(self) -> Vec<T> {
        self.docs
    }
}

#[derive(Deserialize, Debug)]
pub struct Book {
    pub _id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub _id: String,
    pub dialog: String,
    pub movie: String,
    pub character: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

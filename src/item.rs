use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub docs: Vec<T>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub page: u32,
    pub pages: u32,
}

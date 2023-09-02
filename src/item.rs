use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Book {
    pub _id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub _id: String,
    pub name: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub budget_in_millions: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub box_office_revenue_in_millions: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub academy_award_nominations: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub academy_award_wins: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub rotten_tomates_score: u32,
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
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub _id: String,
    pub height: String,
    pub gender: String,
    pub birth: String,
    pub spouse: String,
    pub death: String,
    pub realm: String,
    pub hair: String,
    pub name: String,
    pub wiki_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub _id: String,
    pub chapter_name: String,
    pub book: String,
}

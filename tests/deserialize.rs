use lotr_api_wrapper::{
    item::{Book, Chapter, Character, Movie, Quote, Response},
    requests::Requester,
};

async fn get_json(item: &str) -> String {
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");

    Requester::new(token).get(item).await.unwrap()
}

#[test]
fn test_movie_unit() {
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

#[tokio::test]
async fn test_movie() {
    let json = get_json("movie").await;
    serde_json::from_str::<Response<Movie>>(&json).unwrap();
}

#[tokio::test]
async fn test_book() {
    let json = get_json("book").await;
    serde_json::from_str::<Response<Book>>(&json).unwrap();
}

#[tokio::test]
async fn test_quote() {
    let json = get_json("quote").await;
    serde_json::from_str::<Response<Quote>>(&json).unwrap();
}

#[tokio::test]
async fn test_character() {
    let json = get_json("character").await;
    serde_json::from_str::<Response<Character>>(&json).unwrap();
}

#[tokio::test]
async fn test_chapter() {
    let json = get_json("chapter").await;
    serde_json::from_str::<Response<Chapter>>(&json).unwrap();
}

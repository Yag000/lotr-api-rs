use lotr_api_wrapper::client::Client;

fn get_client() -> Client {
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");
    Client::new(token)
}

#[tokio::test]
async fn test_movie() {
    let client = get_client();
    client.get_movies().await.unwrap();
}

#[tokio::test]
async fn test_book() {
    let client = get_client();
    client.get_books().await.unwrap();
}

#[tokio::test]
async fn test_quote() {
    let client = get_client();
    client.get_quotes().await.unwrap();
}

#[tokio::test]
async fn test_character() {
    let client = get_client();
    client.get_characters().await.unwrap();
}

#[tokio::test]
async fn test_chapter() {
    let client = get_client();
    client.get_chapters().await.unwrap();
}

use lotr_api_wrapper::{
    client::Client,
    item::{Book, ItemType},
    requests::RequestBuilder,
};

pub fn get_client() -> Client {
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

#[tokio::test]
async fn test_get_books_request_builder() {
    let client = get_client();
    let request = RequestBuilder::default().item_type(ItemType::Book).build();
    client.get(request).await.unwrap();
}

#[tokio::test]
async fn tets_get_aragorn_ii_quote() {
    let client = get_client();
    let characters = client.get_characters().await.unwrap();
    let id = &characters
        .iter()
        .find(|c| c.name == "Aragorn II Elessar")
        .unwrap()
        ._id;

    let request = RequestBuilder::default()
        .item_type(ItemType::Character)
        .id(id.clone())
        .second_item(ItemType::Quote)
        .build();

    let quotes = client.get(request).await.unwrap();
    assert!(quotes.len() > 0);
}

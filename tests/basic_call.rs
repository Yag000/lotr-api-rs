use lotr_api::{
    attribute::{Attribute, BookAttribute, CharacterAttribute},
    filter::{Filter, Operator},
    request::{
        sort::{Sort, SortOrder},
        RequestBuilder,
    },
    Client, Item, ItemType, Pagination,
};

pub fn get_client() -> Client {
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");
    Client::new(token)
}

#[tokio::test]
async fn test_movie() {
    let client = get_client();
    let movies = client.get_movies().await.unwrap();

    assert!(movies.len() > 0);
}

#[tokio::test]
async fn test_book() {
    let client = get_client();
    let books = client.get_books().await.unwrap();

    assert!(books.len() > 0);
}

#[tokio::test]
async fn test_quote() {
    let client = get_client();
    let quotes = client.get_quotes().await.unwrap();

    assert!(quotes.len() > 0);
}

// Reminder to update the quote limit if it ever changes
#[tokio::test]
async fn test_quote_number() {
    let client = get_client();
    let quotes = client.get_quotes().await.unwrap();

    assert_eq!(quotes.len(), 2384);
}

#[tokio::test]
async fn test_character() {
    let client = get_client();
    let characters = client.get_characters().await.unwrap();

    assert!(characters.len() > 0);
}

#[tokio::test]
async fn test_chapter() {
    let client = get_client();
    let chapters = client.get_chapters().await.unwrap();

    assert!(chapters.len() > 0);
}

#[tokio::test]
async fn test_get_books_request_builder() {
    let client = get_client();
    let request = RequestBuilder::new(ItemType::Book).build().unwrap();
    let books = client.get(request).await.unwrap();

    assert!(books.len() > 0);
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

    let request = RequestBuilder::new(ItemType::Character)
        .id(id.into())
        .secondary_item_type(ItemType::Quote)
        .build()
        .expect("Failed to build request");

    let quotes = client.get(request).await.unwrap();
    assert!(quotes.len() > 0);
}

#[tokio::test]
async fn test_limit_offset_page() {
    let client = get_client();
    let pagination = Pagination::new(10, 10, 2);
    let request = RequestBuilder::new(ItemType::Character)
        .pagination(pagination)
        .build()
        .expect("Failed to build request");
    let characters = client.get(request).await.unwrap();
    assert_eq!(characters.len(), 10);
}

#[tokio::test]
async fn test_sort() {
    let client = get_client();
    let request = RequestBuilder::new(ItemType::Book)
        .sort(Sort::new(
            SortOrder::Ascending,
            Attribute::Book(BookAttribute::Name),
        ))
        .build()
        .expect("Failed to build request");
    let books = client.get(request).await.unwrap();
    assert!(books.len() > 0);
    match books.first() {
        Some(Item::Book(book)) => assert_eq!(book.name, "The Fellowship Of The Ring"),
        _ => panic!("No books found"),
    }
}

#[tokio::test]
async fn test_filter() {
    let client = get_client();
    let request = RequestBuilder::new(ItemType::Book)
        .filter(Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Eq,
            vec!["The Fellowship Of The Ring".to_string()],
        ))
        .build()
        .expect("Failed to build request");

    let books = client.get(request).await.unwrap();
    assert!(books.len() > 0);
    match books.first() {
        Some(Item::Book(book)) => assert_eq!(book.name, "The Fellowship Of The Ring"),
        _ => panic!("No books found"),
    }
}

#[tokio::test]
async fn test_filter_include() {
    let client = get_client();

    let request = RequestBuilder::new(ItemType::Character)
        .filter(Filter::Match(
            Attribute::Character(CharacterAttribute::Realm),
            Operator::Eq,
            vec!["Gondor".to_string(), "Rohan".to_string()],
        ))
        .build()
        .expect("Failed to build request");

    let characters = client.get(request).await.unwrap();

    assert!(characters.len() > 0);

    for character in characters {
        match character {
            Item::Character(character) => {
                assert!(
                    character.realm == Some("Gondor".to_string())
                        || character.realm == Some("Rohan".to_string()),
                    "Realm is not Gondor or Rohan, it is {:?}",
                    character.realm
                );
            }
            _ => panic!("No characters found"),
        }
    }
}

use lotr_api_wrapper::{
    attribute::{Attribute, BookAttribute},
    client::Client,
    item::ItemType,
    request::{
        pagination::{AddPagination, Pagination},
        FilterReq, GetReq, SortReq, SpecificReq,
    },
    Filter, Item, Operator, Request, Sort, SortOrder,
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
    let request = Request::Get(GetReq::new(ItemType::Book));
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

    let get_req = GetReq::new(ItemType::Character).id(id.into());
    let request = Request::Specific(SpecificReq::new(get_req, ItemType::Quote));

    let quotes = client.get(request).await.unwrap();
    assert!(quotes.len() > 0);
}

#[tokio::test]
async fn test_limit_offset_page() {
    let client = get_client();
    let pagination = Pagination::new(10, 10, 2);
    let request = Request::Get(GetReq::new(ItemType::Quote).add_pagination(pagination));
    let books = client.get(request).await.unwrap();
    assert_eq!(books.len(), 10);
}

#[tokio::test]
async fn test_sort() {
    let client = get_client();
    let request = Request::Sort(SortReq::new(Sort::new(
        SortOrder::Ascending,
        Attribute::Book(BookAttribute::Name),
    )));
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
    let get_req = GetReq::new(ItemType::Book);
    let request = Request::Filter(FilterReq::new(
        get_req,
        Filter::Match(
            Attribute::Book(BookAttribute::Name),
            Operator::Eq,
            vec!["The Fellowship Of The Ring".to_string()],
        ),
    ));
    let books = client.get(request).await.unwrap();
    assert!(books.len() > 0);
    match books.first() {
        Some(Item::Book(book)) => assert_eq!(book.name, "The Fellowship Of The Ring"),
        _ => panic!("No books found"),
    }
}

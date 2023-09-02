use lotr_api_wrapper::{
    item::{Movie, Response},
    requests::Requester,
};

#[tokio::main]
async fn main() {
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");

    let response = Requester::new(token).get("character").await.unwrap();

    println!("{}", response);
}

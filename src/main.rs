use lotr_api_wrapper::requests::Requester;

#[tokio::main]
async fn main() {
    let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");

    let response = Requester::new(token).get("chapter").await.unwrap();
    println!("{:#?}", response);
}

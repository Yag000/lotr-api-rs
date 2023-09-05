# lotr-api-rs

This project is a Rust wrapper around the [LOTR API](https://the-one-api.dev/).

## Usage

There is more documentation available on the [docs.rs](https://docs.rs/lotr-api) page.

```rust
use lotr_api::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("your-api-key");
    let book = client.get_book("5cf5805fb53e011a64671582").await.unwrap();
    println!("{:?}", book);
}
```

This minimal example will print the information about the LOTR books available on the API.

## License

This project is licensed under the [MIT license](LICENSE-MIT) and [Apache License 2.0](LICENSE-APACHE).

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in `lotr-api-rs` by you, shall be licensed as MIT and Apache 2.0, 
without any additional terms or conditions.

Having said that, every contribution is welcome and I look forward to your PRs and issues.

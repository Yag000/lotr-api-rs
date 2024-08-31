//! # `lotr-api`
//!
//! This crate is a wrapper for the [lotr-api](https://the-one-api.dev/).
//! It provides a simple interface to make requests to the API.
//!
//! # Examples
//!
//! ```rust, no_run
//! use lotr_api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!    let client = Client::new("your_token".to_string());
//!    let books = client.get_books().await.unwrap();
//!    let characters = client.get_characters().await.unwrap();
//! }
//! ```
//!
//! ```rust, no_run
//! use lotr_api::{Client, ItemType, RequestBuilder};
//! use lotr_api::filter::{Filter, Operator};
//! use lotr_api::sort::{Sort, SortOrder};
//! use lotr_api::attribute::{Attribute, BookAttribute};
//!
//! #[tokio::main]
//! async fn main() {
//!   let client = Client::new("your_token".to_string());
//!   let request = RequestBuilder::new(ItemType::Book)
//!     .filter(Filter::Match(
//!         Attribute::Book(BookAttribute::Name),
//!         Operator::Eq,
//!         vec!["The Fellowship of the Ring".to_string()])
//!     )
//!     .sort(Sort::new(SortOrder::Ascending, Attribute::Book(BookAttribute::Name)))
//!     .build()
//!     .expect("Failed to build request");
//!     let books = client.get(request).await.unwrap();
//!     // ...
//! }
//! ```
//!
//! # Features
//!
//! - [`Client`] functions to get all items of a type .
//! - [`RequestBuilder`] to build a request with filters, pagination and sorting, which allows the user full control over the request without having to deal with the url.
//!
//!

pub mod client;
pub mod error;
pub mod item;
pub mod request;

pub use client::Client;
pub use error::Error;
pub use item::attribute;
pub use item::object::*;
pub use item::Item;
pub use item::ItemType;
pub use request::filter;
pub use request::pagination::Pagination;
pub use request::sort;
pub use request::Request;
pub use request::RequestBuilder;

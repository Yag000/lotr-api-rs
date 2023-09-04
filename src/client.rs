use crate::{
    item::{Book, Chapter, Character, Item, ItemType, Movie, Quote, Response},
    request::{Request, Requester},
};

/// The error type for this crate.
/// It is used to harmonize the error types of the dependencies.
#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self {
            message: format!("reqwest error: {}", error),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self {
            message: format!("serde_json error: {}", error),
        }
    }
}

/// The client for the one api to rule them all.
/// It is used to make requests to the API.
///
/// # Examples
/// ```rust, no_run
/// use lotr_api_wrapper::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("your_token".to_string());
///     let books = client.get_books().await.unwrap();
///     // ...
/// }
/// ```
pub struct Client {
    requester: Requester,
}

impl Client {
    /// Creates a new client with the given token.
    /// The token is used to authenticate the requests.
    /// You can get a token from <https://the-one-api.dev/>.
    pub fn new(token: String) -> Self {
        Self {
            requester: Requester::new(token),
        }
    }

    async fn request_with_url<T>(&self, url: &str) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.requester.get(url).await?;
        let response: Response<T> = serde_json::from_str(&response)?;
        Ok(response)
    }

    async fn request<T>(&self, request: Request) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.requester.get_from_request(request).await?;
        let response: Response<T> = serde_json::from_str(&response)?;
        Ok(response)
    }

    /// Returns all books.
    pub async fn get_books(&self) -> Result<Vec<Book>, Error> {
        Ok(self.request_with_url::<Book>("book").await?.get_contents())
    }

    /// Returns all movies.
    pub async fn get_movies(&self) -> Result<Vec<Movie>, Error> {
        Ok(self
            .request_with_url::<Movie>("movie")
            .await?
            .get_contents())
    }

    /// Returns the first 1000 quotes, due to the default limit of 1000.
    pub async fn get_quotes(&self) -> Result<Vec<Quote>, Error> {
        Ok(self
            .request_with_url::<Quote>("quote")
            .await?
            .get_contents())
    }

    /// Returns the first 1000 characters, due to the default limit of 1000.
    pub async fn get_characters(&self) -> Result<Vec<Character>, Error> {
        Ok(self
            .request_with_url::<Character>("character")
            .await?
            .get_contents())
    }

    /// Returns all chapters.
    pub async fn get_chapters(&self) -> Result<Vec<Chapter>, Error> {
        Ok(self
            .request_with_url::<Chapter>("chapter")
            .await?
            .get_contents())
    }

    /// Returns the items of the given custom request.
    pub async fn get(&self, request: Request) -> Result<Vec<Item>, Error> {
        let item_type = if let Some(ref item_type) = request.second_item {
            item_type
        } else {
            &request.item_type
        };
        match item_type {
            ItemType::Book => {
                let response = self.request::<Book>(request).await?;
                Ok(response.into())
            }

            ItemType::Movie => {
                let response = self.request::<Movie>(request).await?;
                Ok(response.into())
            }

            ItemType::Quote => {
                let response = self.request::<Quote>(request).await?;
                Ok(response.into())
            }

            ItemType::Character => {
                let response = self.request::<Character>(request).await?;
                Ok(response.into())
            }

            ItemType::Chapter => {
                let response = self.request::<Chapter>(request).await?;
                Ok(response.into())
            }
        }
    }
}

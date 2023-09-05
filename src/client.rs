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
        let response: Response<T> = serde_json::from_str(&response).or(Err(Error {
            message: format!("Failed to deserialize response: {}", response),
        }))?;
        Ok(response)
    }

    async fn request<T>(&self, request: Request) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.requester.get_from_request(request).await?;
        let response: Response<T> = serde_json::from_str(&response).or(Err(Error {
            message: format!("Failed to deserialize response: {}", response),
        }))?;
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

    /// Returns all the quotes. Due to the API default limit of 1000,
    /// this function has to set a hardcoded limit.
    /// Currently there are  2384 quotes, so the limit is set to 2400,
    /// a little bit more than the current amount.
    pub async fn get_quotes(&self) -> Result<Vec<Quote>, Error> {
        Ok(self
            .request_with_url::<Quote>("quote?limit=2400")
            .await?
            .get_contents())
    }

    // Returns all the characters.
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

    /// Returns the book with the given id.
    ///
    /// # Errors
    /// If there is no book with the given id, an error is returned.
    pub async fn get_book_by_id(&self, id: &str) -> Result<Book, Error> {
        let url = format!("book/{}", id);
        let mut books = self.request_with_url::<Book>(&url).await?.get_contents();
        books.pop().ok_or(Error {
            message: format!("No book with id {} found", id),
        })
    }

    /// Returns the movie with the given id.
    ///
    /// # Errors
    /// If there is no movie with the given id, an error is returned.
    pub async fn get_movie_by_id(&self, id: &str) -> Result<Movie, Error> {
        let url = format!("movie/{}", id);
        let mut movies = self.request_with_url::<Movie>(&url).await?.get_contents();
        movies.pop().ok_or(Error {
            message: format!("No movie with id {} found", id),
        })
    }

    /// Returns the quote with the given id.
    ///
    /// # Errors
    /// If there is no quote with the given id, an error is returned.
    pub async fn get_quote_by_id(&self, id: &str) -> Result<Quote, Error> {
        let url = format!("quote/{}", id);
        let mut quotes = self.request_with_url::<Quote>(&url).await?.get_contents();
        quotes.pop().ok_or(Error {
            message: format!("No quote with id {} found", id),
        })
    }

    /// Returns the character with the given id.
    ///
    /// # Errors
    /// If there is no character with the given id, an error is returned.
    pub async fn get_character_by_id(&self, id: &str) -> Result<Character, Error> {
        let url = format!("character/{}", id);
        let mut characters = self
            .request_with_url::<Character>(&url)
            .await?
            .get_contents();
        characters.pop().ok_or(Error {
            message: format!("No character with id {} found", id),
        })
    }

    /// Returns the chapter with the given id.
    ///
    /// # Errors
    /// If there is no chapter with the given id, an error is returned.
    pub async fn get_chapter_by_id(&self, id: &str) -> Result<Chapter, Error> {
        let url = format!("chapter/{}", id);
        let mut chapters = self.request_with_url::<Chapter>(&url).await?.get_contents();
        chapters.pop().ok_or(Error {
            message: format!("No chapter with id {} found", id),
        })
    }

    /// Returns the chapters of the given book.
    pub async fn get_chapters_from_book(&self, book_id: &str) -> Result<Vec<Chapter>, Error> {
        let url = format!("book/{}/chapter", book_id);
        Ok(self.request_with_url::<Chapter>(&url).await?.get_contents())
    }

    /// Returns the quotes of the given book.
    pub async fn get_quotes_from_movie(&self, movie_id: &str) -> Result<Vec<Quote>, Error> {
        let url = format!("movie/{}/quote", movie_id);
        Ok(self.request_with_url::<Quote>(&url).await?.get_contents())
    }

    /// Returns the quotes of the given book.
    pub async fn get_quotes_from_character(&self, character_id: &str) -> Result<Vec<Quote>, Error> {
        let url = format!("character/{}/quote", character_id);
        Ok(self.request_with_url::<Quote>(&url).await?.get_contents())
    }

    /// returns the result of the given request.
    /// You must specify the type of the result, if not
    /// there is no way of deserialize the result.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use lotr_api_wrapper::{Client, ItemType};
    /// use lotr_api_wrapper::item::Book;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("your_token".to_string());
    ///    let url= "book?page=2&limit=2";
    ///    let books = client.get_from_url::<Book>(url).await.unwrap();
    ///    // ...
    /// }
    /// ```
    ///
    pub async fn get_from_url<T>(&self, url: &str) -> Result<Vec<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self.request_with_url::<T>(url).await?.get_contents())
    }

    /// Returns the items of the given custom request.
    pub async fn get(&self, request: Request) -> Result<Vec<Item>, Error> {
        match request.get_item_type() {
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

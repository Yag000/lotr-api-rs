use crate::{
    item::{Book, Chapter, Character, Movie, Quote, Response},
    requests::Requester,
};

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
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

pub struct Client {
    requester: Requester,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            requester: Requester::new(token),
        }
    }

    async fn request<T>(&self, url: &str) -> Result<Response<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.requester.get(url).await?;
        let response: Response<T> = serde_json::from_str(&response)?;
        Ok(response)
    }

    pub async fn get_books(&self) -> Result<Vec<Book>, Error> {
        Ok(self.request::<Book>("book").await?.get_contents())
    }

    pub async fn get_movies(&self) -> Result<Vec<Movie>, Error> {
        Ok(self.request::<Movie>("movie").await?.get_contents())
    }

    /// Returns the first 1000 quotes, due to the default limit of 1000.
    pub async fn get_quotes(&self) -> Result<Vec<Quote>, Error> {
        Ok(self.request::<Quote>("quote").await?.get_contents())
    }

    /// Returns the first 1000 characters, due to the default limit of 1000.
    pub async fn get_characters(&self) -> Result<Vec<Character>, Error> {
        Ok(self.request::<Character>("character").await?.get_contents())
    }

    pub async fn get_chapters(&self) -> Result<Vec<Chapter>, Error> {
        Ok(self.request::<Chapter>("chapter").await?.get_contents())
    }
}

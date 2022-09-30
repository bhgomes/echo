//! Reqwest HTTP Client Utilities

use serde::{de::DeserializeOwned, Serialize};

#[doc(inline)]
pub use reqwest::*;

/// Asynchronous HTTP Client
///
/// This client is a wrapper around [`reqwest::Client`] with a known server URL.
pub struct KnownUrlClient {
    /// Server URL
    pub server_url: Url,

    /// Base HTTP Client
    pub client: Client,
}

impl KnownUrlClient {
    /// Builds a new HTTP [`KnownUrlClient`] that connects to `server_url`.
    #[inline]
    pub fn new<U>(server_url: U) -> Result<Self>
    where
        U: IntoUrl,
    {
        Ok(Self {
            client: Client::builder().build()?,
            server_url: server_url.into_url()?,
        })
    }

    /// Sends a new request asynchronously of type `command` with query string `request`.
    #[inline]
    pub async fn request<T, R>(&self, method: Method, command: &str, request: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.client
            .request(
                method,
                self.server_url
                    .join(command)
                    .expect("Building the URL is not allowed to fail."),
            )
            .json(request)
            .send()
            .await?
            .json()
            .await
    }

    /// Sends a POST request of type `command` with query string `request`.
    #[inline]
    pub async fn post<T, R>(&self, command: &str, request: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.request(Method::POST, command, request).await
    }
}

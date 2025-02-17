//! Send data off to a remote API

pub mod provider;

mod error;
pub use error::*;

use crate::sender::provider::{TokenInjector, TokenProvider};
use reqwest::{header, IntoUrl, Method, RequestBuilder};
use std::sync::Arc;
use std::time::Duration;

pub struct HttpSender {
    client: reqwest::Client,
    provider: Arc<dyn TokenProvider>,
}

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub connect_timeout: Option<Duration>,
    pub timeout: Option<Duration>,
}

const USER_AGENT: &str = concat!("CSAF-Walker/", env!("CARGO_PKG_VERSION"));

impl HttpSender {
    pub async fn new<P>(provider: P, options: Options) -> Result<Self, anyhow::Error>
    where
        P: TokenProvider + 'static,
    {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));

        let mut client = reqwest::ClientBuilder::new().default_headers(headers);

        if let Some(connect_timeout) = options.connect_timeout {
            client = client.connect_timeout(connect_timeout);
        }

        if let Some(timeout) = options.timeout {
            client = client.timeout(timeout);
        }

        Ok(Self {
            client: client.build()?,
            provider: Arc::new(provider),
        })
    }

    /// build a new request, injecting the token
    pub async fn request<U: IntoUrl>(
        &self,
        method: Method,
        url: U,
    ) -> Result<RequestBuilder, Error> {
        self.client
            .request(method, url)
            .inject_token(&self.provider)
            .await
    }
}

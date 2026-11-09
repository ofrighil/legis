use std::fmt;

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::url::canonical_url;

const BASE_URL: &str = "https://api.congress.gov/v3";

#[derive(Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Result<Self> {
        let key: String = key.into();
        let key = key.trim();
        if key.is_empty() {
            Err(Error::MissingApiKey)
        } else {
            Ok(Self(key.to_owned()))
        }
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ApiKey([REDACTED])")
    }
}

#[derive(Clone, Debug)]
pub struct HttpRequest {
    pub url: String,
    pub api_key: ApiKey,
}

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

pub trait Endpoint {
    type Response: DeserializeOwned;

    fn path(&self) -> String;

    fn query(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

pub fn request<E: Endpoint>(api_key: &ApiKey, endpoint: &E) -> HttpRequest {
    let mut query = endpoint.query();
    query.push(("format", "json".to_owned()));
    let url = canonical_url(BASE_URL, &endpoint.path(), query);
    HttpRequest {
        url,
        api_key: api_key.clone(),
    }
}

pub fn decode<E: Endpoint>(url: &str, response: HttpResponse) -> Result<E::Response> {
    if !(200..300).contains(&response.status) {
        Err(Error::Http {
            status: response.status,
            url: url.to_owned(),
        })
    } else {
        serde_json::from_slice(&response.body).map_err(|source| Error::Decode {
            url: url.to_owned(),
            source,
        })
    }
}

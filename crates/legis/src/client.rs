use std::time::Duration;

use legis_core::http::{ApiKey, Endpoint, HttpRequest, HttpResponse, decode, request};

use crate::error::{Error, Result};

pub struct Client {
    agent: ureq::Agent,
}

#[derive(Debug)]
pub struct Fetched<T> {
    pub url: String,
    pub body: Vec<u8>,
    pub value: T,
}

impl Client {
    pub fn new() -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_connect(Some(Duration::from_secs(10)))
            .timeout_global(Some(Duration::from_secs(30)))
            .http_status_as_error(false)
            .user_agent(concat!("legis/", env!("CARGO_PKG_VERSION")))
            .build()
            .into();
        Self { agent }
    }

    pub fn send(&self, req: &HttpRequest) -> Result<HttpResponse> {
        let transport = |e: ureq::Error| Error::Transport {
            url: req.url.clone(),
            message: e.to_string(),
        };

        let mut resp = self
            .agent
            .get(&req.url)
            .header("X-Api-Key", req.api_key.expose())
            .call()
            .map_err(transport)?;
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_vec().map_err(transport)?;
        Ok(HttpResponse { status, body })
    }

    pub fn fetch<E: Endpoint>(
        &self,
        api_key: &ApiKey,
        endpoint: &E,
    ) -> Result<Fetched<E::Response>> {
        let req = request(api_key, endpoint);
        let resp = self.send(&req)?;
        let value = decode::<E>(&req.url, &resp)?;
        Ok(Fetched {
            url: req.url,
            body: resp.body,
            value,
        })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

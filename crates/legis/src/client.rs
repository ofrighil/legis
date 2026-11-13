use std::thread;
use std::time::{Duration, Instant};

use legis_core::http::{ApiKey, Endpoint, HttpRequest, HttpResponse, decode, request};
use legis_core::page::Paged;
use legis_core::policy::{Decision, Pacer, RetryPolicy};

use crate::error::{Error, Result};

pub struct Client {
    agent: ureq::Agent,
    pacer: Pacer,
    retry: RetryPolicy,
}

#[derive(Debug)]
pub struct Fetched<T> {
    pub url: String,
    pub body: Vec<u8>,
    pub value: T,
}

impl Client {
    pub fn new(pacer: Pacer, retry: RetryPolicy) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_connect(Some(Duration::from_secs(10)))
            .timeout_global(Some(Duration::from_secs(30)))
            .http_status_as_error(false)
            .user_agent(concat!("legis/", env!("CARGO_PKG_VERSION")))
            .build()
            .into();
        Self {
            agent,
            pacer,
            retry,
        }
    }

    pub fn fetch<E: Endpoint>(
        &mut self,
        api_key: &ApiKey,
        endpoint: &E,
    ) -> Result<Fetched<E::Response>> {
        let req = request(api_key, endpoint);
        let resp = self.execute(&req)?;
        let value = decode::<E>(&req.url, &resp)?;
        Ok(Fetched {
            url: req.url,
            body: resp.body,
            value,
        })
    }

    pub fn fetch_all<E: Paged>(
        &mut self,
        api_key: &ApiKey,
        endpoint: &E,
    ) -> Result<Vec<Fetched<E::Response>>> {
        let mut pages = Vec::new();
        let mut next = Some(endpoint.page());
        while let Some(page) = next {
            let fetched = self.fetch(api_key, &endpoint.with_page(page))?;
            next = page.next(E::pagination(&fetched.value));
            pages.push(fetched);
        }
        Ok(pages)
    }

    fn execute(&mut self, req: &HttpRequest) -> Result<HttpResponse> {
        let mut retries = 0;
        loop {
            thread::sleep(self.pacer.delay(Instant::now()));
            let resp = self.send(req)?;
            match self.retry.decide(resp.status, retries) {
                Decision::RetryAfter(delay) => {
                    thread::sleep(delay);
                    retries += 1;
                }
                Decision::Accept | Decision::Fail => return Ok(resp),
            }
        }
    }

    fn send(&self, req: &HttpRequest) -> Result<HttpResponse> {
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
}

impl Default for Client {
    fn default() -> Self {
        Self::new(Pacer::default(), RetryPolicy::default())
    }
}

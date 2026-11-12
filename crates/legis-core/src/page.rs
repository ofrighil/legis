use serde::Deserialize;

use crate::http::Endpoint;

const MAX_LIMIT: u32 = 250;

pub trait Paged: Endpoint {
    fn page(&self) -> Page;

    fn with_page(&self, page: Page) -> Self
    where
        Self: Sized;

    fn pagination(response: &Self::Response) -> &Pagination;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Page {
    pub offset: u32,
    pub limit: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: MAX_LIMIT,
        }
    }
}

impl Page {
    pub fn next(self, pagination: &Pagination) -> Option<Self> {
        pagination.next.as_ref()?;
        if self.limit == 0 {
            return None;
        }
        let offset = self.offset.checked_add(self.limit)?;
        (offset < pagination.count).then_some(Self { offset, ..self })
    }

    pub fn query(self) -> [(&'static str, String); 2] {
        [
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
        ]
    }
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub count: u32,
    pub next: Option<String>,
}

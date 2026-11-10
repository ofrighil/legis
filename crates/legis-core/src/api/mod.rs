use serde::Deserialize;

pub mod bill;
pub mod congress;
pub mod member;

#[derive(Debug, Deserialize)]
pub struct CountedLink {
    pub count: u32,
    pub url: String,
}

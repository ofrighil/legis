use jiff::civil::{Date, Time};
use serde::Deserialize;

pub mod bill;
pub mod congress;
pub mod member;

#[derive(Debug, Deserialize)]
pub struct CountedLink {
    pub count: u32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Items<T> {
    pub item: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: Date,
    pub action_time: Option<Time>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct PolicyArea {
    pub name: String,
}

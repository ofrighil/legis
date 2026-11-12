use jiff::civil::Date;
use serde::Deserialize;

use crate::http::Endpoint;
use crate::ids::Congress;

pub struct Current;

impl Endpoint for Current {
    type Response = CongressResponse;

    fn path(&self) -> String {
        "congress/current".to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct CongressResponse {
    pub congress: CongressDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongressDetail {
    pub number: Congress,
    pub end_year: String,
    pub name: String,
    pub sessions: Vec<CongressSession>,
    pub start_year: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongressSession {
    pub chamber: String,
    pub number: u16,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

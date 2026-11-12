use jiff::Timestamp;
use jiff::civil::{Date, Time};
use serde::Deserialize;

use super::CountedLink;
use crate::http::Endpoint;
use crate::ids::{BillId, BillType, Congress};

pub struct Get(pub BillId);

impl Endpoint for Get {
    type Response = BillResponse;

    fn path(&self) -> String {
        let id = &self.0;
        format!("bill/{}/{}/{}", id.congress, id.kind.as_path(), id.number)
    }
}

#[derive(Debug, Deserialize)]
pub struct BillResponse {
    pub bill: BillDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillDetail {
    pub congress: Congress,
    #[serde(rename = "type")]
    pub kind: BillType,
    pub number: String,
    pub title: String,
    pub introduced_date: Date,
    pub origin_chamber: String,
    pub origin_chamber_code: String,
    pub policy_area: Option<PolicyArea>,
    pub latest_action: LatestAction,
    pub sponsors: Vec<Sponsor>,
    pub cosponsors: Option<Cosponsors>,
    pub laws: Option<Vec<Law>>,
    pub legislation_url: String,
    pub update_date: Timestamp,
    pub actions: Option<CountedLink>,
    pub amendments: Option<CountedLink>,
    pub committees: Option<CountedLink>,
    pub related_bills: Option<CountedLink>,
    pub subjects: Option<CountedLink>,
    pub summaries: Option<CountedLink>,
    pub text_versions: Option<CountedLink>,
    pub titles: Option<CountedLink>,
}

#[derive(Debug, Deserialize)]
pub struct PolicyArea {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestAction {
    pub action_date: Date,
    pub action_time: Option<Time>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub bioguide_id: String,
    pub full_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub party: String,
    pub state: String,
    pub district: Option<u8>,
    pub is_by_request: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosponsors {
    pub count: u32,
    pub count_including_withdrawn_cosponsors: u32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Law {
    pub number: String,
    #[serde(rename = "type")]
    pub kind: String,
}

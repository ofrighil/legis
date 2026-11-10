use serde::Deserialize;

use super::CountedLink;
use crate::http::Endpoint;
use crate::ids::BioguideId;

pub struct Get(pub BioguideId);

impl Endpoint for Get {
    type Response = MemberResponse;

    fn path(&self) -> String {
        format!("member/{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct MemberResponse {
    pub member: MemberDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberDetail {
    pub bioguide_id: String,
    pub direct_order_name: String,
    pub inverted_order_name: String,
    pub honorific_name: Option<String>,
    pub birth_year: Option<String>,
    pub current_member: bool,
    pub state: String,
    pub district: Option<u8>,
    pub official_website_url: Option<String>,
    pub party_history: Vec<PartyHistory>,
    pub terms: Vec<Term>,
    pub address_information: Option<AddressInformation>,
    pub depiction: Option<Depiction>,
    pub sponsored_legislation: Option<CountedLink>,
    pub cosponsored_legislation: Option<CountedLink>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyHistory {
    pub party_abbreviation: String,
    pub party_name: String,
    pub start_year: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub chamber: String,
    pub congress: u16,
    pub member_type: String,
    pub start_year: u16,
    pub end_year: Option<u16>,
    pub state_code: String,
    pub state_name: String,
    pub district: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressInformation {
    pub office_address: String,
    pub city: Option<String>,
    pub district: Option<String>,
    pub phone_number: Option<String>,
    pub zip_code: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depiction {
    pub image_url: String,
    pub attribution: Option<String>,
}

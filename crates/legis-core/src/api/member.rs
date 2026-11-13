use jiff::Timestamp;
use jiff::civil::Date;
use serde::Deserialize;

use super::{CountedLink, Items, LatestAction, PolicyArea};
use crate::http::Endpoint;
use crate::ids::{BillType, BioguideId, Congress};
use crate::page::{Page, Paged, Pagination};

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
    pub congress: Congress,
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
    pub image_url: Option<String>,
    pub attribution: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct List {
    pub current_member: bool,
    pub page: Page,
}

impl Endpoint for List {
    type Response = MemberListResponse;

    fn path(&self) -> String {
        "member".to_string()
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::from(self.page.query());
        if self.current_member {
            query.push(("currentMember", "true".to_owned()));
        }
        query
    }
}

impl Paged for List {
    fn page(&self) -> Page {
        self.page
    }

    fn with_page(&self, page: Page) -> Self {
        Self { page, ..*self }
    }

    fn pagination(response: &Self::Response) -> &Pagination {
        &response.pagination
    }
}

#[derive(Debug, Deserialize)]
pub struct MemberListResponse {
    pub members: Vec<MemberItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberItem {
    pub bioguide_id: String,
    pub name: String,
    pub party_name: String,
    pub state: String,
    pub district: Option<u8>,
    pub depiction: Option<Depiction>,
    pub terms: Items<MemberItemTerm>,
    pub update_date: Timestamp,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberItemTerm {
    pub chamber: String,
    pub start_year: u16,
    pub end_year: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct SponsoredLegislation {
    pub id: BioguideId,
    pub page: Page,
}

impl Endpoint for SponsoredLegislation {
    type Response = SponsoredLegislationResponse;

    fn path(&self) -> String {
        format!("member/{}/sponsored-legislation", self.id)
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        Vec::from(self.page.query())
    }
}

impl Paged for SponsoredLegislation {
    fn page(&self) -> Page {
        self.page
    }

    fn with_page(&self, page: Page) -> Self {
        Self {
            id: self.id.clone(),
            page,
        }
    }

    fn pagination(response: &Self::Response) -> &Pagination {
        &response.pagination
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredLegislationResponse {
    pub sponsored_legislation: Vec<SponsoredItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredItem {
    pub congress: Congress,
    pub introduced_date: Date,
    #[serde(rename = "type")]
    pub kind: Option<BillType>,
    pub number: Option<String>,
    pub amendment_number: Option<String>,
    pub title: Option<String>,
    pub policy_area: Option<PolicyArea>,
    pub latest_action: Option<LatestAction>,
    pub url: String,
}

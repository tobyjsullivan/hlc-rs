use url::Url;
use std::borrow::Cow;

use crate::error::RequestError;
use crate::data::{
    AccountID,
    Timestamp,
};

#[derive(Debug)]
pub struct FilterRequest<'a> {
    criteria: Box<Vec<FilterCriteria<'a>>>,
}

#[derive(Debug)]
pub enum FilterCriteria<'a> {
    SexEq { sex: Sex },
    EmailDomain { domain: &'a str },
    EmailGt { gt: String },
    EmailLt { lt: String },
    StatusEq { status: Status },
    StatusNeq { status: Status },
    FnameEq { fname: String },
    FnameAny { names: Vec<String> },
    FnameNull,
    FnameNotNull,
    SnameEq { sname: String },
    SnameStarts { prefix: String },
    SnameNull,
    SnameNotNull,
    PhoneCode { code: u16 },
    PhoneNull,
    PhoneNotNull,
    CountryEq { country: String },
    CountryNull,
    CountryNotNull,
    CityEq { city: String },
    CityAny { cities: Vec<String> },
    CityNull,
    CityNotNull,
    BirthLt { date: Timestamp },
    BirthGt { date: Timestamp },
    BirthYear { year: u16 },
    InterestsContains { interests: Vec<String> },
    InterestsAny { interests: Vec<String> },
    LikesContains { likees: Vec<AccountID> },
    PremiumNow,
    PremiumNull,
    PremiumNotNull,
}

impl <'a> FilterRequest<'a> {
    fn from(url: &str) -> Result<Self, RequestError> {
        let url = Url::parse(url).map_err(|_| RequestError::BadRequest)?;

        let mut criteria = Box::new(Vec::new());
        let pairs = url.query_pairs();
    
        for pair in pairs {
            match pair {
                (Cow::Borrowed(k), Cow::Borrowed(v)) => criteria.push(map_criteria(k, v)?),
                _ => break,
            };
        }

        Ok(Self {
            criteria: criteria,
        })
    }

    pub fn resolve(url: &str) -> Result<FilterResponse, RequestError> {
        let req = Self::from(url)?;
        // TODO
        Ok(FilterResponse {
            accounts: Vec::new(),
        })
    }
}

fn map_criteria<'a>(k: &'a str, v: &'a str) -> Result<FilterCriteria<'a>, RequestError> {
    match k {
        "sex_eq" => Ok(FilterCriteria::SexEq{sex: parse_sex(v)?}),
        "email_domain" => Ok(FilterCriteria::EmailDomain { domain: v }),
        "email_lt" => Ok(FilterCriteria::EmailLt { lt: String::from(v) }),
        "email_gt" => Ok(FilterCriteria::EmailGt { gt: String::from(v) }),
        "status_eq" => Ok(FilterCriteria::StatusEq { status: parse_status(v)? }),
        "status_neq" => Ok(FilterCriteria::StatusNeq { status: parse_status(v)? }),
        "fname_eq" => Ok(FilterCriteria::FnameEq { fname: String::from(v) }),
        _ => Err(RequestError::BadRequest),
    }
}

fn split_on_commas(s: &str) -> Vec<&str> {
    if s == "" {
        return Vec::new();
    }

    s.split(',').collect()
}

fn parse_sex(s: &str) -> Result<Sex, RequestError> {
    match s {
        "f" => Ok(Sex::Female),
        "m" => Ok(Sex::Male),
        _ => Err(RequestError::BadRequest),
    }
}

fn parse_status(s: &str) -> Result<Status, RequestError> {
    match s {
        "свободны" => Ok(Status::Looking),
        "заняты" => Ok(Status::InRelationship),
        "всё сложно" => Ok(Status::ItsComplicated),
        _ => Err(RequestError::BadRequest),
    }
}

#[derive(Debug)]
pub struct FilterResponse {
    accounts: Vec<FilterResponseAccount>,
}

#[derive(Debug)]
struct FilterResponseAccount {
}


#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
pub enum Status {
    Looking,
    ItsComplicated,
    InRelationship,
}

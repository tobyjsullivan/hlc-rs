use crate::data::{
    AccountID,
    Timestamp,
};

#[derive(Debug)]
pub struct CreateRequest {
    id: AccountID,
    email: String,
    fname: Option<String>,
    sname: Option<String>,
    phone: Option<PhoneNumber>,
    sex: Sex,
    birth: Timestamp,
    country: Option<String>,
    city: Option<String>,
    joined: Timestamp,
    status: Status,
    interests: Vec<String>,
    premium: Option<Premium>,
    likes: Vec<Like>,
}

impl CreateRequest {
    pub fn from(data: &[u8]) -> Result<CreateRequest, ParseErr> {
        let payload: CreateRequestPayload = serde_json::from_slice(data)?;
        payload.into()
    }
}

impl From<CreateRequestPayload> for Result<CreateRequest, ParseErr> {
    fn from(payload: CreateRequestPayload) -> Result<CreateRequest, ParseErr> {
        Ok(CreateRequest {
            id: payload.id,
            email: payload.email,
            fname: payload.fname,
            sname: payload.sname,
            phone: payload.phone.and_then(|ph| parse_phone(ph)),
            sex: parse_sex(payload.sex)?,
            birth: payload.birth,
            country: payload.country,
            city: payload.city,
            joined: payload.joined,
            status: parse_status(payload.status)?,
            interests: payload.interests,
            premium: payload.premium.map(|p| p.into()),
            likes: payload.likes.into_iter().map(|l| l.into()).collect(),
        })
    }
}

#[derive(Debug)]
pub enum ParseErr {
    InvalidSex,
    InvalidStatus,
    JsonError { cause: serde_json::Error },
}

impl From<serde_json::Error> for ParseErr {
    fn from(err: serde_json::Error) -> ParseErr {
        ParseErr::JsonError { cause: err }
    }
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

#[derive(Debug)]
pub struct PhoneNumber {
    prefix: u8,
    code: u16,
    suffix: u32,
}

#[derive(Debug)]
pub struct Premium {
    start: Timestamp,
    finish: Timestamp,
}

impl From<CreateRequestPayloadPremium> for Premium {
    fn from(p: CreateRequestPayloadPremium) -> Premium {
        Premium {
            start: p.start,
            finish: p.finish,
        }
    }
}

#[derive(Debug)]
pub struct Like {
    id: AccountID,
    ts: Timestamp,
}

impl From<CreateRequestPayloadLike> for Like {
    fn from(p: CreateRequestPayloadLike) -> Like {
        Like { id: p.id, ts: p.ts }
    }
}

fn parse_phone(ph: String) -> Option<PhoneNumber> {
    // TODO
    None
}

fn parse_sex(s: String) -> Result<Sex, ParseErr> {
    match s.as_ref() {
        "f" => Ok(Sex::Female),
        "m" => Ok(Sex::Male),
        _ => Err(ParseErr::InvalidSex),
    }
}

fn parse_status(s: String) -> Result<Status, ParseErr> {
    match s.as_ref() {
        "свободны" => Ok(Status::Looking),
        "заняты" => Ok(Status::InRelationship),
        "всё сложно" => Ok(Status::ItsComplicated),
        _ => Err(ParseErr::InvalidStatus),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRequestPayload {
    id: AccountID,
    email: String,
    fname: Option<String>,
    sname: Option<String>,
    phone: Option<String>,
    sex: String,
    birth: Timestamp,
    country: Option<String>,
    city: Option<String>,
    joined: Timestamp,
    status: String,
    interests: Vec<String>,
    premium: Option<CreateRequestPayloadPremium>,
    likes: Vec<CreateRequestPayloadLike>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRequestPayloadPremium {
    start: Timestamp,
    finish: Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRequestPayloadLike {
    id: AccountID,
    ts: Timestamp,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct IPInfoErrorRespDetail {
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct IPInfoErrorResp {
    pub status: u16,
    pub error: IPInfoErrorRespDetail,
}

#[derive(Debug)]
pub enum IPInfoError {
    Request(reqwest::Error),
    Api(IPInfoErrorResp),
}

pub struct IPInfoClient {
    pub client: reqwest::Client,
    pub base_url: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct IPInfo {
    pub ip: String,
    pub hostname: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub loc: Option<String>,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub timezone: Option<String>,
    pub readme: Option<String>,
    pub anycast: Option<bool>,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
pub struct IPInfoMini {
    pub ip: String,
}

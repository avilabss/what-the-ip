use serde::Deserialize;

const IP_INFO_PROVIDER: &str = "https://ipinfo.io";

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct IPInfo {
    ip: String,
    hostname: Option<String>,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>,
    org: Option<String>,
    postal: Option<String>,
    timezone: Option<String>,
    readme: Option<String>,
}

impl IPInfo {
    pub async fn fetch(ip: Option<&str>) -> Result<Self, reqwest::Error> {
        let mut url = String::from(IP_INFO_PROVIDER);

        match ip {
            Some(ip) => {
                url.push('/');
                url.push_str(ip);
            }
            None => {}
        }

        url.push_str("/json");

        reqwest::get(&url).await?.json::<IPInfo>().await
    }

    pub fn print(&self) {
        println!("ip: {}", self.ip);
        if let Some(hostname) = &self.hostname {
            println!("hostname: {}", hostname);
        }
        if let Some(city) = &self.city {
            println!("city: {}", city);
        }
        if let Some(region) = &self.region {
            println!("region: {}", region);
        }
        if let Some(country) = &self.country {
            println!("country: {}", country);
        }
        if let Some(loc) = &self.loc {
            println!("loc: {}", loc);
        }
        if let Some(org) = &self.org {
            println!("org: {}", org);
        }
        if let Some(postal) = &self.postal {
            println!("postal: {}", postal);
        }
        if let Some(timezone) = &self.timezone {
            println!("timezone: {}", timezone);
        }
        if let Some(readme) = &self.readme {
            println!("readme: {}", readme);
        }
    }
}

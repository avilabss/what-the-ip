use serde::{Deserialize, Serialize};
use std::time::Duration;

const IP_INFO_PROVIDER: &str = "https://ipinfo.io";

pub struct IPInfoClient {
    client: reqwest::Client,
    base_url: String,
}

impl IPInfoClient {
    pub fn new(proxy_url: Option<&str>, timeout: u64) -> Result<Self, reqwest::Error> {
        let mut client = reqwest::Client::builder().timeout(Duration::from_secs(timeout));

        if let Some(proxy_url) = proxy_url {
            client = client.proxy(reqwest::Proxy::all(proxy_url)?);
        }

        Ok(IPInfoClient {
            client: client.build()?,
            base_url: IP_INFO_PROVIDER.to_string(),
        })
    }

    #[cfg(test)]
    fn with_base_url(base_url: &str) -> Self {
        IPInfoClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn fetch(&self, ip: Option<&str>) -> Result<IPInfo, reqwest::Error> {
        let mut url = self.base_url.clone();

        if let Some(ip) = ip {
            url.push('/');
            url.push_str(ip);
        }

        url.push_str("/json");

        self.client.get(&url).send().await?.json::<IPInfo>().await
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
    anycast: Option<bool>,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct IPInfoMini {
    ip: String,
}

impl IPInfo {
    fn get_mini(&self) -> IPInfoMini {
        IPInfoMini {
            ip: self.ip.clone(),
        }
    }

    fn print_human_mini(&self) {
        let mini = self.get_mini();
        println!("ip: {}", mini.ip);
    }

    fn print_human_full(&self) {
        const W: usize = 10;

        println!("{:W$} {}", "ip:", self.ip);
        if let Some(hostname) = &self.hostname {
            println!("{:W$} {}", "hostname:", hostname);
        }
        if let Some(city) = &self.city {
            println!("{:W$} {}", "city:", city);
        }
        if let Some(region) = &self.region {
            println!("{:W$} {}", "region:", region);
        }
        if let Some(country) = &self.country {
            println!("{:W$} {}", "country:", country);
        }
        if let Some(loc) = &self.loc {
            println!("{:W$} {}", "loc:", loc);
        }
        if let Some(org) = &self.org {
            println!("{:W$} {}", "org:", org);
        }
        if let Some(postal) = &self.postal {
            println!("{:W$} {}", "postal:", postal);
        }
        if let Some(timezone) = &self.timezone {
            println!("{:W$} {}", "timezone:", timezone);
        }
        if let Some(readme) = &self.readme {
            println!("{:W$} {}", "readme:", readme);
        }
        if let Some(anycast) = &self.anycast {
            println!("{:W$} {}", "anycast:", anycast);
        }
    }

    fn print_human(&self, extra_metadata: bool) {
        if extra_metadata {
            self.print_human_full();
        } else {
            self.print_human_mini();
        }
    }

    fn print_json_mini(&self) {
        let mini = self.get_mini();
        println!("{}", serde_json::to_string_pretty(&mini).unwrap());
    }

    fn print_json_full(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }

    fn print_json(&self, extra_metadata: bool) {
        if extra_metadata {
            self.print_json_full();
        } else {
            self.print_json_mini();
        }
    }

    pub fn print(&self, extra_metadata: bool, json: bool) {
        if json {
            self.print_json(extra_metadata);
        } else {
            self.print_human(extra_metadata);
        }
    }
}

#[tokio::test]
async fn test_fetch_ip_info() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ip": "1.1.1.1",
            "hostname": "one.one.one.one",
            "city": "Brisbane",
            "region": "Queensland",
            "country": "AU",
            "loc": "-27.4679,153.0281",
            "org": "AS13335 Cloudflare, Inc.",
            "postal": "9010",
            "timezone": "Australia/Brisbane",
            "readme": "https://ipinfo.io/missingauth",
            "anycast": true
        })))
        .mount(&mock_server)
        .await;

    let client = IPInfoClient::with_base_url(&mock_server.uri());
    let ip_info = client.fetch(None).await.unwrap();

    assert_eq!(ip_info.ip, "1.1.1.1");
    assert_eq!(ip_info.hostname.unwrap(), "one.one.one.one");
    assert_eq!(ip_info.city.unwrap(), "Brisbane");
    assert_eq!(ip_info.region.unwrap(), "Queensland");
    assert_eq!(ip_info.country.unwrap(), "AU");
    assert_eq!(ip_info.loc.unwrap(), "-27.4679,153.0281");
    assert_eq!(ip_info.org.unwrap(), "AS13335 Cloudflare, Inc.");
    assert_eq!(ip_info.postal.unwrap(), "9010");
    assert_eq!(ip_info.timezone.unwrap(), "Australia/Brisbane");
    assert_eq!(ip_info.readme.unwrap(), "https://ipinfo.io/missingauth");
    assert_eq!(ip_info.anycast.unwrap(), true);
}

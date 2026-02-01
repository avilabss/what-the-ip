use super::error::{IPInfoError, IPInfoErrorResp};
use super::models::IPInfo;
use std::time::Duration;

const IP_INFO_PROVIDER: &str = "https://ipinfo.io";

pub struct IPInfoClient {
    client: reqwest::Client,
    base_url: String,
}

impl IPInfoClient {
    pub fn new(proxy_url: Option<&str>, timeout: u64) -> Result<Self, IPInfoError> {
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

    pub async fn fetch(&self, ip: Option<&str>) -> Result<IPInfo, IPInfoError> {
        let mut url = self.base_url.clone();

        if let Some(ip) = ip {
            url.push('/');
            url.push_str(ip);
        }

        url.push_str("/json");

        let resp = self.client.get(&url).send().await?;
        if resp.status().is_success() {
            let ip_info = resp.json::<IPInfo>().await?;
            Ok(ip_info)
        } else {
            let e = resp.json::<IPInfoErrorResp>().await?;
            Err(IPInfoError::Api(e))
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

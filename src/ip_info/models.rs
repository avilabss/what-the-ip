use super::utils::print_opt;
use serde::{Deserialize, Serialize};

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

impl IPInfo {
    fn get_mini(&self) -> IPInfoMini {
        IPInfoMini {
            ip: self.ip.clone(),
        }
    }

    fn print_human_mini(&self) {
        let mini = self.get_mini();
        print_opt("ip:", &Some(mini.ip));
    }

    fn print_human_full(&self) {
        print_opt("ip:", &Some(self.ip.clone()));
        print_opt("hostname:", &self.hostname);
        print_opt("city:", &self.city);
        print_opt("region:", &self.region);
        print_opt("country:", &self.country);
        print_opt("loc:", &self.loc);
        print_opt("org:", &self.org);
        print_opt("postal:", &self.postal);
        print_opt("timezone:", &self.timezone);
        print_opt("readme:", &self.readme);
        print_opt("anycast:", &self.anycast);
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

use crate::model::Zone;
use crate::{YamahaErrorCode, parse_response_code};
use reqwest::blocking::Client;
use serde_json::Value;
use std::net::Ipv4Addr;

pub struct YamahaAmpBlocking {
    pub ip: Ipv4Addr,
    client: Client,
    pub model: String,
    pub device_id: String,
    pub api_version: String,
}

impl YamahaAmpBlocking {
    pub(crate) fn from_discovery(ip: Ipv4Addr, json: Value) -> Self {
        Self {
            ip,
            client: Client::new(),
            model: json
                .get("model_name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            device_id: json
                .get("device_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            api_version: json
                .get("api_version")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("http://{}/YamahaExtendedControl/v1/{}", self.ip, path)
    }
    pub fn get_zone_status(&self, zone: Zone) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("{zone}/getStatus"));

        let resp = self
            .client
            .get(url)
            .send()
            .map_err(|_| YamahaErrorCode::Timeout)?;
        let json: serde_json::Value = resp.json().map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&json.to_string());
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(json)
    }
    pub fn get_main_status(&self) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint("main/getStatus");

        let resp = self
            .client
            .get(url)
            .send()
            .map_err(|_| YamahaErrorCode::Timeout)?;
        let json: serde_json::Value = resp.json().map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&json.to_string());
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(json)
    }
    pub fn set_volume(&self, volume: i32) -> Result<(), YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setVolume?volume={}", volume));

        let resp = self
            .client
            .get(url)
            .send()
            .map_err(|_| YamahaErrorCode::Timeout)?;
        let text = resp.text().map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&text);
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(())
    }
}

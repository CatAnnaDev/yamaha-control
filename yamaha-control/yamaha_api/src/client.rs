use reqwest::Client;
pub(crate) use crate::model::Zone;
use crate::{parse_response_code, YamahaAmp, YamahaErrorCode};

impl YamahaAmp {

    pub async fn get_zone_status(&self, zone: Zone) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("{zone}/getStatus"));
        let client = Client::new();

        let resp = client.get(url).send().await.map_err(|_| YamahaErrorCode::Timeout)?;
        let json: serde_json::Value = resp.json().await.map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&json.to_string());
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(json)
    }
    pub async fn get_main_status(&self) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint("main/getStatus");
        let client = Client::new();

        let resp = client.get(url).send().await.map_err(|_| YamahaErrorCode::Timeout)?;
        let json: serde_json::Value = resp.json().await.map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&json.to_string());
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(json)
    }

    pub async fn set_volume(&self, volume: i32) -> Result<(), YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setVolume?volume={}", volume));
        let client = Client::new();

        let resp = client.get(url).send().await.map_err(|_| YamahaErrorCode::Timeout)?;
        let text = resp.text().await.map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&text);
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(())
    }
}

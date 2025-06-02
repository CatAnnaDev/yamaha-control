use crate::model::Zone;
use crate::{YamahaErrorCode, connect_direct, discover_amplifiers, parse_response_code, PowerState, Input, SoundProgram};
use reqwest::Client;
use serde_json::Value;
use std::net::Ipv4Addr;

pub struct YamahaAmpAsync {
    pub ip: Ipv4Addr,
    client: Client,
    pub model: String,
    pub device_id: String,
    pub api_version: String,
}

impl YamahaAmpAsync {
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

    pub async fn discover() -> Vec<YamahaAmpAsync> {
        discover_amplifiers().await
    }

    pub async fn connect(ip: Ipv4Addr) -> Option<YamahaAmpAsync> {
        connect_direct(ip).await
    }

    fn endpoint(&self, path: &str) -> String {
        format!("http://{}/YamahaExtendedControl/v1/{}", self.ip, path)
    }
    
    async fn request(&self, url: String) -> Result<serde_json::Value, YamahaErrorCode> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| YamahaErrorCode::Timeout)?;
        
        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| YamahaErrorCode::InvalidResponse)?;

        let code = parse_response_code(&json.to_string());
        if code != YamahaErrorCode::Ok {
            return Err(code);
        }

        Ok(json)
    }

    pub async fn get_zone_status(&self, zone: Zone) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("{zone}/getStatus"));
        Self::request(self, url).await
    }
    
    pub async fn get_sound_program_list(&self, zone: Zone) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("{zone}/getSoundProgramList"));
        Self::request(self, url).await
    }
    
    pub async fn get_main_status(&self) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint("main/getStatus");
        Self::request(self, url).await
    }

    pub async fn get_features(&self) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint("system/getFeatures");
        Self::request(self, url).await
    }

    pub async fn get_signal_info(&self) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint("main/getSignalInfo");
        Self::request(self, url).await
    }

    pub async fn set_volume(&self, volume: i32) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setVolume?volume={}", volume));
        Self::request(self, url).await
    }

    pub async fn set_sound_program(&self, program: SoundProgram) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setSoundProgram?program={}", program));
        Self::request(self, url).await
    }
    
    pub async fn set_power(&self, power_state: PowerState) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setPower?power={}", power_state.as_str()));
        Self::request(self, url).await
    }

    pub async fn set_mute(&self, mute: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setMute?enable={}", mute.to_string().to_lowercase()));
        Self::request(self, url).await
    }

    pub async fn set_input(&self, input: Input) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setInput?input={}", input));
        Self::request(self, url).await
    }

    pub async fn set_direct(&self, direct: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setDirect?enable={}", direct.to_string().to_lowercase()));
        Self::request(self, url).await
    }

    pub async fn set_pure_direct(&self, direct: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setPureDirect?enable={}", direct.to_string().to_lowercase()));
        Self::request(self, url).await
    }

    pub async fn set_enhancer(&self, enhance: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setEnhancer?enable={}", enhance.to_string().to_lowercase()));
        Self::request(self, url).await
    }

    pub async fn set_dialogue_level(&self, level: i32) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setDialogueLevel?value={}", level));
        Self::request(self, url).await
    }

    pub async fn set_subwoofer_volume(&self, volume: i32) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setSubwooferVolume?volume={}", volume));
        Self::request(self, url).await
    }

    pub async fn set_bass_extension(&self, extension: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setBassExtension?enable={}", extension.to_string().to_lowercase()));
        Self::request(self, url).await
    }

    pub async fn set_extra_bass(&self, extra_bass: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setExtraBass?enable={}", extra_bass.to_string().to_lowercase()));
        Self::request(self, url).await
    }
    
    pub async fn set_adaptative_drc(&self, drc: bool) -> Result<serde_json::Value, YamahaErrorCode> {
        let url = self.endpoint(&format!("main/setAdaptativeDrc?enable={}", drc.to_string().to_lowercase()));
        Self::request(self, url).await   
    }
}

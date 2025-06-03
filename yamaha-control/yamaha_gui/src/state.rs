use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::time::Duration;
use yamaha_api::*;

#[derive(Clone)]
pub struct AppState {
    pub amplifiers: Vec<AmpInfo>,
    pub selected_amp: Option<usize>,
    pub is_discovering: bool,
    pub discovery_config: DiscoveryConfig,
    pub current_status: Option<GetStatus>,
    pub volume: i32,
    pub is_muted: bool,
    pub current_input: Option<Input>,
    pub current_program: Option<SoundProgram>,
    pub power_state: Option<PowerState>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AmpInfo {
    pub ip: Ipv4Addr,
    pub model: String,
    pub device_id: String,
    pub api_version: f32,
    pub is_connected: bool,
    pub last_seen: Option<std::time::SystemTime>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            amplifiers: Vec::new(),
            selected_amp: None,
            is_discovering: false,
            discovery_config: DiscoveryConfig {
                subnet: Ipv4Addr::new(192, 168, 1, 0),
                mask: 24,
                timeout: Duration::from_millis(500),
                max_concurrent: 50,
            },
            current_status: None,
            volume: 0,
            is_muted: false,
            current_input: None,
            current_program: None,
            power_state: None,
        }
    }

    pub fn add_amplifier(&mut self, amp_info: AmpInfo) {
        if let Some(existing) = self.amplifiers.iter_mut().find(|a| a.ip == amp_info.ip) {
            *existing = amp_info;
        } else {
            self.amplifiers.push(amp_info);
        }
    }

    pub fn get_selected_amp(&self) -> Option<&AmpInfo> {
        self.selected_amp.and_then(|idx| self.amplifiers.get(idx))
    }
}

use crate::state::{AmpInfo, AppState};
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use yamaha_api::*;

pub struct DiscoveryPanel {
    state: Arc<Mutex<AppState>>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl DiscoveryPanel {
    pub fn new(state: Arc<Mutex<AppState>>, rt: &Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            state,
            rt: rt.clone(),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Discovery");

            let rt = self.rt.clone();
            let state = self.state.clone();

            //ui.horizontal(|ui| {
            //    let mut subnet_str = {
            //        let state_guard = rt.block_on(async { state.lock().await });
            //        state_guard.discovery_config.subnet.to_string()
            //    };
            //
            //    ui.label("Subnet:");
            //    if ui.text_edit_singleline(&mut subnet_str).changed() {
            //        if let Ok(_ip) = subnet_str.parse::<Ipv4Addr>() {
            //            let _state_guard = rt.block_on(async { state.lock().await });
            //        }
            //    }
            //});

            //ui.horizontal(|ui| {
            //    let mut mask = {
            //        let state_guard = rt.block_on(async { state.lock().await });
            //        state_guard.discovery_config.mask as i32
            //    };
            //    ui.label("Mask:");
            //    ui.add(egui::Slider::new(&mut mask, 8..=30));
            //    let state_guard = rt.block_on(async { state.lock().await });
            //    if state_guard.discovery_config.mask != mask as u8 {
            //        drop(state_guard);
            //        let mut state_guard = rt.block_on(async { state.lock().await });
            //        state_guard.discovery_config.mask = mask as u8;
            //    }
            //});

            let is_discovering = {
                let state_guard = rt.block_on(async { state.lock().await });
                state_guard.is_discovering
            };

            if is_discovering {
                ui.add_enabled(false, egui::Button::new("🔍 Discovering..."));
                ui.spinner();
            } else {
                if ui.button("🔍 Discover Amplifiers").clicked() {
                    self.start_discovery();
                }
            }
        });
    }

    fn start_discovery(&self) {
        let rt = self.rt.clone();
        let state = self.state.clone();

        rt.spawn(async move {
            {
                let mut state_guard = state.lock().await;
                state_guard.is_discovering = true;
                state_guard.amplifiers.clear();
            }

            let config = {
                let state_guard = state.lock().await;
                state_guard.discovery_config
            };

            match YamahaAmpAsync::discover(Some(config)).await {
                Ok(amps) => {
                    let mut state_guard = state.lock().await;
                    for amp in amps {
                        let amp_info = AmpInfo {
                            ip: amp.ip,
                            model: amp.info.model.clone(),
                            device_id: amp.info.device_id.clone(),
                            api_version: amp.info.api_version,
                            is_connected: true,
                            last_seen: Some(std::time::SystemTime::now()),
                        };
                        state_guard.add_amplifier(amp_info);
                    }
                }
                Err(e) => {
                    eprintln!("Discovery failed: {:?}", e);
                }
            }

            {
                let mut state_guard = state.lock().await;
                state_guard.is_discovering = false;
            }
        });
    }
}

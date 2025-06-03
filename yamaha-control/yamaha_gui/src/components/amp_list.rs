use crate::state::AppState;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AmpList {
    state: Arc<Mutex<AppState>>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl AmpList {
    pub fn new(state: Arc<Mutex<AppState>>, rt: &Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            state,
            rt: rt.clone(),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let rt = self.rt.clone();
        let state = self.state.clone();

        egui::ScrollArea::vertical().show(ui, |ui| {
            let (amplifiers, selected_amp) = {
                let state_guard = rt.block_on(async { state.lock().await });
                (state_guard.amplifiers.clone(), state_guard.selected_amp)
            };

            if amplifiers.is_empty() {
                ui.label("No amplifiers found");
                ui.label("Click 'Discover' to search for devices");
                return;
            }

            for (idx, amp) in amplifiers.iter().enumerate() {
                let is_selected = selected_amp == Some(idx);

                let response =
                    ui.selectable_label(is_selected, format!("🎵 {} ({})", amp.model, amp.ip));

                if response.clicked() {
                    let mut state_guard = rt.block_on(async { state.lock().await });
                    state_guard.selected_amp = Some(idx);
                    drop(state_guard);
                    self.load_amp_status(idx);
                }

                response.on_hover_text(format!(
                    "Device ID: {}\nAPI Version: {}\nLast seen: {:?}",
                    amp.device_id, amp.api_version, amp.last_seen
                ));
            }
        });
    }

    fn load_amp_status(&self, amp_idx: usize) {
        let rt = self.rt.clone();
        let state = self.state.clone();

        rt.spawn(async move {
            let amp_ip = {
                let state_guard = state.lock().await;
                if let Some(amp) = state_guard.amplifiers.get(amp_idx) {
                    amp.ip
                } else {
                    return;
                }
            };

            if let Ok(Some(amp)) = yamaha_api::YamahaAmpAsync::connect(amp_ip).await {
                if let Ok(status_json) = amp.get_main_status().await {
                    if let Ok(status) = serde_json::from_value::<yamaha_api::GetStatus>(status_json)
                    {
                        let mut state_guard = state.lock().await;
                        state_guard.current_status = Some(status.clone());
                        state_guard.volume = status.volume as i32;
                        state_guard.is_muted = status.mute;
                        state_guard.power_state = Some(if status.power == "on" {
                            yamaha_api::PowerState::On
                        } else {
                            yamaha_api::PowerState::Standby
                        });
                    }
                }
            }
        });
    }
}

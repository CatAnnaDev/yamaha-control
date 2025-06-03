use crate::state::AppState;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use yamaha_api::*;

pub struct AmpControlPanel {
    state: Arc<Mutex<AppState>>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl AmpControlPanel {
    pub fn new(state: Arc<Mutex<AppState>>, rt: &Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            state,
            rt: rt.clone(),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let rt = self.rt.clone();
        let state = self.state.clone();

        let state_guard = rt.block_on(async { state.lock().await });

        if state_guard.selected_amp.is_none() {
            ui.centered_and_justified(|ui| {
                ui.label("Select an amplifier to control");
            });
            return;
        }

        let selected_amp = state_guard.get_selected_amp().unwrap().clone();
        drop(state_guard);

        ui.heading(&format!("Control: {}", selected_amp.model));
        ui.separator();

        ui.group(|ui| {
            ui.heading("Power");
            ui.horizontal(|ui| {
                if ui.button("🔴 Power On").clicked() {
                    self.send_power_command(selected_amp.ip, PowerState::On);
                }
                if ui.button("⏸ Standby").clicked() {
                    self.send_power_command(selected_amp.ip, PowerState::Standby);
                }
            });
        });

        ui.separator();

        ui.group(|ui| {
            ui.heading("Volume");

            let mut volume = {
                let state_guard = rt.block_on(async { state.lock().await });
                state_guard.volume
            };

            ui.horizontal(|ui| {
                if ui.button("🔇").clicked() {
                    self.send_mute_command(selected_amp.ip, true);
                }

                let slider_response = ui.add(
                    egui::Slider::new(&mut volume, 0..=161)
                        .text("dB")
                        .step_by(1.0),
                );

                if slider_response.changed() {
                    self.send_volume_command(selected_amp.ip, volume);
                    let state_guard = rt.block_on(async { state.lock().await });
                    drop(state_guard);
                    let mut state_guard = rt.block_on(async { state.lock().await });
                    state_guard.volume = volume;
                }

                if ui.button("🔊").clicked() {
                    self.send_mute_command(selected_amp.ip, false);
                }
            });

            ui.label(format!("Current: {} dB", volume));
        });

        ui.separator();

        ui.group(|ui| {
            ui.heading("Input Source");

            egui::Grid::new("input_grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    let inputs = [
                        ("HDMI 1", Input::Hdmi1),
                        ("HDMI 2", Input::Hdmi2),
                        ("HDMI 3", Input::Hdmi3),
                        ("Optical", Input::Optical),
                        ("Bluetooth", Input::Bluetooth),
                        ("USB", Input::Usb),
                        ("CD", Input::Cd),
                        ("Phono", Input::Phono),
                        ("Aux", Input::Aux),
                    ];

                    for (i, (name, input)) in inputs.iter().enumerate() {
                        if ui.button(*name).clicked() {
                            self.send_input_command(selected_amp.ip, *input);
                        }

                        if (i + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });

        ui.separator();

        ui.group(|ui| {
            ui.heading("Sound Programs");

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let programs = [
                        ("Straight", SoundProgram::Straight),
                        ("Stereo", SoundProgram::Stereo),
                        ("Movie", SoundProgram::Movie),
                        ("Music", SoundProgram::Music),
                        ("Concert", SoundProgram::Concert),
                        ("Jazz Club", SoundProgram::JazzClub),
                        ("Arena", SoundProgram::Arena),
                    ];

                    for (name, program) in &programs {
                        if ui.button(*name).clicked() {
                            self.send_sound_program_command(selected_amp.ip, *program);
                        }
                    }
                });
            });
        });

        ui.separator();

        ui.group(|ui| {
            ui.heading("Status");

            let state_guard = rt.block_on(async { state.lock().await });

            if let Some(status) = &state_guard.current_status {
                egui::Grid::new("status_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Power:");
                        ui.label(&status.power);
                        ui.end_row();

                        ui.label("Volume:");
                        ui.label(format!("{} / {}", status.volume, status.max_volume));
                        ui.end_row();

                        ui.label("Mute:");
                        ui.label(if status.mute { "Yes" } else { "No" });
                        ui.end_row();

                        ui.label("Input:");
                        ui.label(&status.input);
                        ui.end_row();

                        ui.label("Sound Program:");
                        ui.label(&status.sound_program);
                        ui.end_row();
                    });
            } else {
                ui.label("Loading status...");
            }
        });
    }

    fn send_power_command(&self, ip: std::net::Ipv4Addr, power_state: PowerState) {
        let rt = self.rt.clone();
        rt.spawn(async move {
            if let Ok(Some(amp)) = YamahaAmpAsync::connect(ip).await {
                let _ = amp.set_power(power_state).await;
            }
        });
    }

    fn send_volume_command(&self, ip: std::net::Ipv4Addr, volume: i32) {
        let rt = self.rt.clone();
        rt.spawn(async move {
            if let Ok(Some(amp)) = YamahaAmpAsync::connect(ip).await {
                let _ = amp.set_volume(volume).await;
            }
        });
    }

    fn send_mute_command(&self, ip: std::net::Ipv4Addr, mute: bool) {
        let rt = self.rt.clone();
        rt.spawn(async move {
            if let Ok(Some(amp)) = YamahaAmpAsync::connect(ip).await {
                let _ = amp.set_mute(mute).await;
            }
        });
    }

    fn send_input_command(&self, ip: std::net::Ipv4Addr, input: Input) {
        let rt = self.rt.clone();
        rt.spawn(async move {
            if let Ok(Some(amp)) = YamahaAmpAsync::connect(ip).await {
                let _ = amp.set_input(input).await;
            }
        });
    }

    fn send_sound_program_command(&self, ip: std::net::Ipv4Addr, program: SoundProgram) {
        let rt = self.rt.clone();
        rt.spawn(async move {
            if let Ok(Some(amp)) = YamahaAmpAsync::connect(ip).await {
                let _ = amp.set_sound_program(program).await;
            }
        });
    }
}

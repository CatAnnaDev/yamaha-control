use crate::components::{AmpControlPanel, AmpList, DiscoveryPanel};
use crate::state::AppState;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct YamahaApp {
    state: Arc<Mutex<AppState>>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl YamahaApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();

        Self {
            state: Arc::new(Mutex::new(AppState::new())),
            rt: Arc::new(rt),
        }
    }
}

impl eframe::App for YamahaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = self.state.clone();

        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let state_guard = self.rt.block_on(async { state.lock().await });
                let status = if state_guard.is_discovering {
                    "🔍 Discovering amplifiers..."
                } else {
                    &format!("Found {} amplifier(s)", state_guard.amplifiers.len())
                };
                ui.label(status);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state_guard.selected_amp.is_some() {
                        ui.colored_label(egui::Color32::GREEN, "● Connected");
                    } else {
                        ui.colored_label(egui::Color32::RED, "● Disconnected");
                    }
                });
            });
        });

        egui::SidePanel::left("amp_list_panel")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Amplifiers");

                let discovery_panel = DiscoveryPanel::new(state.clone(), &self.rt);
                discovery_panel.show(ui);

                ui.separator();

                let amp_list = AmpList::new(state.clone(), &self.rt);
                amp_list.show(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let control_panel = AmpControlPanel::new(state.clone(), &self.rt);
            control_panel.show(ui);
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

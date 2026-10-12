#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod outlook;

use chrono::Local;
use eframe::egui;

use crate::config::ConfigData;

/// Builds the receipient email body — identical to the Python `entry()` function.
fn build_entry(
    date: &str,
    hours: &str,
    phase: &str,
    tk_num: &str,
    rate: &str,
    description: &str,
) -> String {
    format!(
        "INCUR DATE={}\nHOURS={}\nRate={}\nTimekeeper={}\nPhase={}\nDESCRIPTION={}\n",
        date, hours, rate, tk_num, phase, description
    )
}

struct App {
    cfg: Option<ConfigData>,
    date: String,
    matter: String,
    hours: String,
    phase: String,
    rate: String,
    description: String,
    error: Option<String>,
}

impl App {
    fn new() -> Self {
        match config::load_config() {
            Ok(cfg) => App {
                cfg: Some(cfg),
                date: Local::now().format("%m/%d/%Y").to_string(),
                matter: String::new(),
                hours: String::new(),
                phase: String::new(),
                rate: String::new(),
                description: String::new(),
                error: None,
            },
            Err(e) => App {
                cfg: None,
                date: String::new(),
                matter: String::new(),
                hours: String::new(),
                phase: String::new(),
                rate: String::new(),
                description: String::new(),
                error: Some(e),
            },
        }
    }

    /// Clears the data-entry fields — mirrors the Python `clear()` function.
    fn clear_fields(&mut self) {
        self.matter.clear();
        self.hours.clear();
        self.phase.clear();
        self.description.clear();
        if let Some(cfg) = &self.cfg {
            if cfg.multi_bill_rate {
                // Reset billing rate to default rate.
                self.rate = cfg.billing_rate.clone();
            }
        }
    }

    fn submit(&mut self) {
        let cfg = match &self.cfg {
            Some(c) => c,
            None => return,
        };
        let effective_rate = if cfg.multi_bill_rate {
            self.rate.clone()
        } else {
            cfg.billing_rate.clone()
        };

        let msg = build_entry(
            &self.date,
            &self.hours,
            &self.phase,
            &cfg.tk_number,
            &effective_rate,
            &self.description,
        );

        match outlook::send_email(&cfg.email, &self.matter, &msg) {
            Ok(()) => self.clear_fields(),
            Err(e) => self.error = Some(e),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // If the config file could not be loaded, behave like the Python
        // popup: show the problem and stop. Nothing can be done until it's fixed.
        let cfg = match self.cfg.clone() {
            Some(c) => c,
            None => {
                egui::Window::new("Config File Error")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        if let Some(err) = &self.error {
                            ui.label(err);
                        }
                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                return;
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FIPBilling Time Entry System");
            ui.add_space(6.0);

            egui::Grid::new("entry_form")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Date:");
                    ui.add(egui::TextEdit::singleline(&mut self.date).desired_width(220.0));
                    ui.end_row();

                    ui.label("Matter No.:");
                    ui.add(egui::TextEdit::singleline(&mut self.matter).desired_width(220.0));
                    ui.end_row();

                    ui.label("Hours:");
                    ui.add(egui::TextEdit::singleline(&mut self.hours).desired_width(220.0));
                    ui.end_row();

                    ui.label("Phase:");
                    ui.add(egui::TextEdit::singleline(&mut self.phase).desired_width(220.0));
                    ui.end_row();

                    // Rate row only exists for users with multiple billing rates.
                    if cfg.multi_bill_rate {
                        ui.label("Rate:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.rate).desired_width(220.0),
                        );
                        ui.end_row();
                    }

                    ui.label("Description:");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.description)
                            .desired_rows(5)
                            .desired_width(320.0),
                    );
                    ui.end_row();
                });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Submit").clicked() {
                    self.submit();
                }
                if ui.button("Clear").clicked() {
                    self.clear_fields();
                }
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });

        // Error popup for failed submissions.
        if let Some(err) = self.error.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(&err);
                    ui.add_space(6.0);
                    if ui.button("OK").clicked() {
                        self.error = None;
                    }
                });
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 430.0])
            .with_title("FIPBilling Time Entry System"),
        ..Default::default()
    };
    eframe::run_native(
        "FIPBilling Time Entry System",
        options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
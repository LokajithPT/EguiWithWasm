use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Stroke, Vec2};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    app: AppInfo,
    description: String,
    author: String,
    settings: Settings,
    features: Features,
    users: Users,
}

#[derive(Debug, Deserialize)]
struct AppInfo {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    theme: String,
    language: String,
    font_size: u32,
}

#[derive(Debug, Deserialize)]
struct Features {
    feature: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
struct Feature {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@active")]
    active: bool,
}

#[derive(Debug, Deserialize)]
struct Users {
    user: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@role")]
    role: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@email")]
    email: String,
}

pub struct WasmandeguiApp {
    config: Config,
    side_panel_open: bool,
}

const ACCENT: Color32 = Color32::from_rgb(0x5C, 0x9C, 0xE6);
const SURFACE: Color32 = Color32::from_rgb(0x1E, 0x1E, 0x2E);
const CARD: Color32 = Color32::from_rgb(0x25, 0x25, 0x3A);
const TEXT: Color32 = Color32::from_rgb(0xE0, 0xE0, 0xE0);
const TEXT_MUTED: Color32 = Color32::from_rgb(0x88, 0x88, 0x99);
const GREEN: Color32 = Color32::from_rgb(0x4C, 0xAF, 0x50);
const RED: Color32 = Color32::from_rgb(0xF4, 0x44, 0x44);
const GOLD: Color32 = Color32::from_rgb(0xFF, 0xD7, 0x00);
const BLUE: Color32 = Color32::from_rgb(0x4A, 0x9E, 0xFF);
const GRAY: Color32 = Color32::from_rgb(0x99, 0x99, 0x99);
const BORDER: Color32 = Color32::from_rgb(0x3A, 0x3A, 0x50);

impl WasmandeguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.global_style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.widgets.noninteractive.bg_fill = SURFACE;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_MUTED);
        style.visuals.widgets.inactive.weak_bg_fill = CARD;
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
        style.visuals.widgets.active.weak_bg_fill = ACCENT;
        style.visuals.selection.bg_fill = ACCENT.gamma_multiply(0.3f32);
        style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        cc.egui_ctx.set_global_style(style);

        let xml = include_str!("../config.xml");
        let config: Config = quick_xml::de::from_str(xml).expect("failed to parse config.xml");
        Self {
            config,
            side_panel_open: true,
        }
    }
}

impl eframe::App for WasmandeguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.side_panel_open {
            egui::Panel::left("side_panel")
                .resizable(true)
                .default_size(220.0)
                .min_size(160.0)
                .frame(Frame {
                    fill: SURFACE,
                    inner_margin: Margin::symmetric(8, 8),
                    ..Default::default()
                })
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("☰ NAVIGATION")
                                .size(12.0)
                                .color(ACCENT)
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("✕").size(14.0).color(TEXT_MUTED),
                                    )
                                    .fill(Color32::TRANSPARENT)
                                    .min_size(Vec2::new(24.0, 24.0)),
                                )
                                .clicked()
                            {
                                self.side_panel_open = false;
                            }
                        });
                    });
                    ui.add_space(4.0);

                    Frame {
                        fill: BORDER,
                        inner_margin: Margin::symmetric(0, 1),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(ui.available_width(), 1.0));
                    });
                    ui.add_space(4.0);

                    egui::ScrollArea::vertical()
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            side_section(ui, "📦 APP INFO", ACCENT, |ui| {
                                row(ui, "Name", &self.config.app.name);
                                row(ui, "Version", &self.config.app.version);
                            });
                            ui.add_space(2.0);
                            side_section(ui, "⚙ SETTINGS", ACCENT, |ui| {
                                row(ui, "Theme", &self.config.settings.theme);
                                row(ui, "Language", &self.config.settings.language);
                                row(ui, "Font", &self.config.settings.font_size.to_string());
                            });
                            ui.add_space(2.0);
                            side_section(ui, "🚀 FEATURES", ACCENT, |ui| {
                                for f in &self.config.features.feature {
                                    ui.horizontal(|ui| {
                                        let (icon, color) = if f.active { ("●", GREEN) } else { ("○", RED) };
                                        ui.label(egui::RichText::new(icon).size(10.0).color(color));
                                        ui.label(
                                            egui::RichText::new(&f.name)
                                                .size(13.0)
                                                .color(if f.active { TEXT } else { TEXT_MUTED }),
                                        );
                                    });
                                }
                            });
                            ui.add_space(2.0);
                            side_section(ui, "👥 USERS", ACCENT, |ui| {
                                for u in &self.config.users.user {
                                    ui.horizontal(|ui| {
                                        let role_color = role_color(&u.role);
                                        ui.label(
                                            egui::RichText::new("▸")
                                                .size(12.0)
                                                .color(role_color),
                                        );
                                        ui.label(egui::RichText::new(&u.name).size(13.0).color(TEXT));
                                        ui.label(
                                            egui::RichText::new(&u.role)
                                                .size(11.0)
                                                .color(role_color),
                                        );
                                    });
                                }
                            });
                        });
                });
        }

        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(0x16, 0x16, 0x24),
                inner_margin: Margin::symmetric(16, 16),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                if !self.side_panel_open {
                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("☰ Show sidebar")
                                        .size(13.0)
                                        .color(ACCENT),
                                )
                                .fill(SURFACE)
                                .stroke(Stroke::new(1.0, BORDER)),
                            )
                            .clicked()
                        {
                            self.side_panel_open = true;
                        }
                    });
                    ui.add_space(8.0);
                }

                card_frame(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("📋 Config Dashboard")
                                .size(20.0)
                                .color(TEXT)
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(&format!("v{}", self.config.app.version))
                                    .size(12.0)
                                    .color(ACCENT),
                            );
                        });
                    });
                });

                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        card(ui, "📦 Application", ACCENT, |ui| {
                            egui::Grid::new("app_grid")
                                .striped(true)
                                .min_col_width(80.0)
                                .show(ui, |ui| {
                                    row_strip(ui, "Name", &self.config.app.name);
                                    ui.end_row();
                                    row_strip(ui, "Version", &self.config.app.version);
                                    ui.end_row();
                                    row_strip(ui, "Author", &self.config.author);
                                    ui.end_row();
                                });
                        });

                        ui.add_space(8.0);

                        card(ui, "📝 Description", ACCENT, |ui| {
                            ui.label(egui::RichText::new(&self.config.description).color(TEXT_MUTED));
                        });
                    });

                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        card(ui, "⚙ Settings", ACCENT, |ui| {
                            egui::Grid::new("settings_grid")
                                .striped(true)
                                .min_col_width(80.0)
                                .show(ui, |ui| {
                                    row_strip(ui, "Theme", &self.config.settings.theme);
                                    ui.end_row();
                                    row_strip(ui, "Language", &self.config.settings.language);
                                    ui.end_row();
                                    row_strip(ui, "Font Size", &self.config.settings.font_size.to_string());
                                    ui.end_row();
                                });
                        });

                        ui.add_space(8.0);

                        card(ui, "🌐 Environment", ACCENT, |ui| {
                            ui.label(
                                egui::RichText::new("Cross-platform Rust application")
                                    .color(TEXT_MUTED),
                            );
                        });
                    });
                });

                ui.add_space(12.0);

                card(ui, "🚀 Features", ACCENT, |ui| {
                    egui::Grid::new("features_grid")
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            for f in &self.config.features.feature {
                                let (icon, color) = if f.active { ("✓", GREEN) } else { ("✗", RED) };
                                ui.label(egui::RichText::new(icon).size(14.0).color(color).strong());
                                ui.label(egui::RichText::new(&f.name).size(14.0).color(TEXT).strong());
                                ui.label(
                                    egui::RichText::new(if f.active { "enabled" } else { "disabled" })
                                        .size(12.0)
                                        .color(if f.active { GREEN } else { TEXT_MUTED }),
                                );
                                ui.end_row();
                            }
                        });
                });

                ui.add_space(12.0);

                card(ui, "👥 Users", ACCENT, |ui| {
                    for u in &self.config.users.user {
                        let role_color = role_color(&u.role);
                        let header_bg = role_color.gamma_multiply(0.15f32);
                        Frame {
                            fill: header_bg,
                            corner_radius: CornerRadius::same(6),
                            inner_margin: Margin::symmetric(8, 4),
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            egui::CollapsingHeader::new(
                                egui::RichText::new(format!("{} — {}", u.name, u.role))
                                    .size(14.0)
                                    .color(TEXT)
                                    .strong(),
                            )
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.add_space(4.0);
                                egui::Grid::new(format!("user_{}", u.id))
                                    .striped(true)
                                    .min_col_width(60.0)
                                    .show(ui, |ui| {
                                        row_strip(ui, "ID", &u.id.to_string());
                                        ui.end_row();
                                        row_strip(ui, "Role", &u.role);
                                        ui.end_row();
                                        row_strip(ui, "Email", &u.email);
                                        ui.end_row();
                                    });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
            });
    }
}

fn role_color(role: &str) -> Color32 {
    match role {
        "admin" => GOLD,
        "editor" => BLUE,
        _ => GRAY,
    }
}

fn card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    Frame {
        fill: SURFACE,
        corner_radius: CornerRadius::same(8),
        stroke: Stroke::new(1.0, BORDER),
        inner_margin: Margin::symmetric(14, 10),
        ..Default::default()
    }
    .show(ui, add_contents);
}

fn card(ui: &mut egui::Ui, title: &str, color: Color32, add_contents: impl FnOnce(&mut egui::Ui)) {
    Frame {
        fill: SURFACE,
        corner_radius: CornerRadius::same(8),
        stroke: Stroke::new(1.0, BORDER),
        inner_margin: Margin::symmetric(0, 0),
        ..Default::default()
    }
    .show(ui, |ui| {
        Frame {
            fill: color.gamma_multiply(0.12f32),
            corner_radius: CornerRadius {
                nw: 8,
                ne: 8,
                sw: 0,
                se: 0,
            },
            inner_margin: Margin::symmetric(12, 6),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(13.0)
                    .color(color)
                    .strong(),
            );
        });

        Frame {
            inner_margin: Margin::symmetric(12, 10),
            ..Default::default()
        }
        .show(ui, add_contents);
    });
}

fn side_section(ui: &mut egui::Ui, title: &str, color: Color32, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(
        egui::RichText::new(title).size(11.0).color(color).strong(),
    )
    .default_open(true)
    .show(ui, |ui| {
        ui.add_space(2.0);
        Frame {
            inner_margin: Margin::symmetric(4, 2),
            ..Default::default()
        }
        .show(ui, add_contents);
    });
}

fn row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).size(12.0).color(TEXT_MUTED));
        ui.label(egui::RichText::new(format!("  {}", value)).size(12.0).color(TEXT));
    });
}

fn row_strip(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(egui::RichText::new(label).size(13.0).color(TEXT_MUTED).strong());
    let bg = if ui.style().visuals.widgets.noninteractive.bg_fill == SURFACE {
        CARD
    } else {
        SURFACE
    };
    Frame {
        fill: bg,
        corner_radius: CornerRadius::same(4),
        inner_margin: Margin::symmetric(6, 2),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.label(egui::RichText::new(value).size(13.0).color(TEXT));
    });
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("the_canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let runner = eframe::WebRunner::new();
    runner
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(WasmandeguiApp::new(cc)))),
        )
        .await
        .ok();
}

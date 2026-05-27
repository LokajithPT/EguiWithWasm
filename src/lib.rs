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

#[derive(Clone)]
struct Theme {
    bg: Color32,
    surface: Color32,
    card: Color32,
    card_alt: Color32,
    border: Color32,
    text: Color32,
    text_muted: Color32,
    accent: Color32,
    green: Color32,
    red: Color32,
    gold: Color32,
    blue: Color32,
    gray: Color32,
    is_dark: bool,
}

impl Theme {
    fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(0x0F, 0x0F, 0x1A),
            surface: Color32::from_rgb(0x1A, 0x1A, 0x2E),
            card: Color32::from_rgb(0x23, 0x23, 0x40),
            card_alt: Color32::from_rgb(0x2A, 0x2A, 0x48),
            border: Color32::from_rgb(0x33, 0x33, 0x50),
            text: Color32::from_rgb(0xE8, 0xE8, 0xF0),
            text_muted: Color32::from_rgb(0x88, 0x88, 0xA8),
            accent: Color32::from_rgb(0x6C, 0xAC, 0xF0),
            green: Color32::from_rgb(0x4C, 0xAF, 0x50),
            red: Color32::from_rgb(0xF4, 0x44, 0x44),
            gold: Color32::from_rgb(0xFF, 0xD7, 0x00),
            blue: Color32::from_rgb(0x4A, 0x9E, 0xFF),
            gray: Color32::from_rgb(0x99, 0x99, 0xB0),
            is_dark: true,
        }
    }

    fn light() -> Self {
        Self {
            bg: Color32::from_rgb(0xF0, 0xF2, 0xF5),
            surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            card: Color32::from_rgb(0xF8, 0xF9, 0xFA),
            card_alt: Color32::from_rgb(0xF0, 0xF1, 0xF3),
            border: Color32::from_rgb(0xDA, 0xDE, 0xE4),
            text: Color32::from_rgb(0x1A, 0x1A, 0x2E),
            text_muted: Color32::from_rgb(0x6B, 0x72, 0x80),
            accent: Color32::from_rgb(0x4A, 0x8C, 0xE0),
            green: Color32::from_rgb(0x2E, 0x7D, 0x32),
            red: Color32::from_rgb(0xD3, 0x2F, 0x2F),
            gold: Color32::from_rgb(0xB8, 0x86, 0x0B),
            blue: Color32::from_rgb(0x3B, 0x82, 0xF6),
            gray: Color32::from_rgb(0x6B, 0x72, 0x80),
            is_dark: false,
        }
    }

    fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();
        style.visuals.dark_mode = self.is_dark;
        style.visuals.widgets.noninteractive.bg_fill = self.surface;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_muted);
        style.visuals.widgets.inactive.weak_bg_fill = self.card;
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text);
        style.visuals.widgets.active.weak_bg_fill = self.accent;
        style.visuals.selection.bg_fill = self.accent.gamma_multiply(0.3f32);
        style.visuals.selection.stroke = Stroke::new(1.0, self.accent);
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        ctx.set_global_style(style);
    }
}

pub struct WasmandeguiApp {
    config: Config,
    side_panel_open: bool,
    theme: Theme,
}

impl WasmandeguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::dark();
        theme.apply(&cc.egui_ctx);

        let xml = include_str!("../config.xml");
        let config: Config = quick_xml::de::from_str(xml).expect("failed to parse config.xml");
        Self {
            config,
            side_panel_open: true,
            theme,
        }
    }
}

impl eframe::App for WasmandeguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = self.theme.clone();

        if self.side_panel_open {
            egui::Panel::left("side_panel")
                .resizable(true)
                .default_size(220.0)
                .min_size(160.0)
                .frame(Frame {
                    fill: theme.surface,
                    inner_margin: Margin::symmetric(8, 8),
                    ..Default::default()
                })
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("☰ NAVIGATION")
                                .size(12.0)
                                .color(theme.accent)
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("✕").size(14.0).color(theme.text_muted),
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
                        fill: theme.border,
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
                            side_section(ui, "📦 APP INFO", &theme, |ui| {
                                row(ui, "Name", &self.config.app.name, &theme);
                                row(ui, "Version", &self.config.app.version, &theme);
                            });
                            ui.add_space(2.0);
                            side_section(ui, "⚙ SETTINGS", &theme, |ui| {
                                row(ui, "Theme", &self.config.settings.theme, &theme);
                                row(ui, "Language", &self.config.settings.language, &theme);
                                row(ui, "Font", &self.config.settings.font_size.to_string(), &theme);
                            });
                            ui.add_space(2.0);
                            side_section(ui, "🚀 FEATURES", &theme, |ui| {
                                for f in &self.config.features.feature {
                                    ui.horizontal(|ui| {
                                        let (icon, color) = if f.active { ("●", theme.green) } else { ("○", theme.red) };
                                        ui.label(egui::RichText::new(icon).size(10.0).color(color));
                                        ui.label(
                                            egui::RichText::new(&f.name)
                                                .size(13.0)
                                                .color(if f.active { theme.text } else { theme.text_muted }),
                                        );
                                    });
                                }
                            });
                            ui.add_space(2.0);
                            side_section(ui, "👥 USERS", &theme, |ui| {
                                for u in &self.config.users.user {
                                    ui.horizontal(|ui| {
                                        let role_color = role_color(&u.role, &theme);
                                        ui.label(
                                            egui::RichText::new("▸")
                                                .size(12.0)
                                                .color(role_color),
                                        );
                                        ui.label(egui::RichText::new(&u.name).size(13.0).color(theme.text));
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
                fill: theme.bg,
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
                                        .color(theme.accent),
                                )
                                .fill(theme.surface)
                                .stroke(Stroke::new(1.0, theme.border)),
                            )
                            .clicked()
                        {
                            self.side_panel_open = true;
                        }
                    });
                    ui.add_space(8.0);
                }

                card_frame(ui, &theme, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("📋 Config Dashboard")
                                .size(20.0)
                                .color(theme.text)
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let btn = egui::Button::new(
                                egui::RichText::new(if theme.is_dark { "☀️" } else { "🌙" })
                                    .size(16.0),
                            )
                            .fill(Color32::TRANSPARENT)
                            .min_size(Vec2::new(32.0, 28.0));
                            if ui.add(btn).clicked() {
                                self.toggle_theme(ui.ctx());
                            }
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(&format!("v{}", self.config.app.version))
                                    .size(12.0)
                                    .color(theme.accent),
                            );
                        });
                    });
                });

                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        card(ui, "📦 Application", &theme, |ui| {
                            egui::Grid::new("app_grid")
                                .striped(true)
                                .min_col_width(80.0)
                                .show(ui, |ui| {
                                    row_strip(ui, "Name", &self.config.app.name, &theme);
                                    ui.end_row();
                                    row_strip(ui, "Version", &self.config.app.version, &theme);
                                    ui.end_row();
                                    row_strip(ui, "Author", &self.config.author, &theme);
                                    ui.end_row();
                                });
                        });

                        ui.add_space(8.0);

                        card(ui, "📝 Description", &theme, |ui| {
                            ui.label(egui::RichText::new(&self.config.description).color(theme.text_muted));
                        });
                    });

                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        card(ui, "⚙ Settings", &theme, |ui| {
                            egui::Grid::new("settings_grid")
                                .striped(true)
                                .min_col_width(80.0)
                                .show(ui, |ui| {
                                    row_strip(ui, "Theme", &self.config.settings.theme, &theme);
                                    ui.end_row();
                                    row_strip(ui, "Language", &self.config.settings.language, &theme);
                                    ui.end_row();
                                    row_strip(ui, "Font Size", &self.config.settings.font_size.to_string(), &theme);
                                    ui.end_row();
                                });
                        });

                        ui.add_space(8.0);

                        card(ui, "🌐 Environment", &theme, |ui| {
                            ui.label(
                                egui::RichText::new("Cross-platform Rust application")
                                    .color(theme.text_muted),
                            );
                        });
                    });
                });

                ui.add_space(12.0);

                card(ui, "🚀 Features", &theme, |ui| {
                    egui::Grid::new("features_grid")
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            for f in &self.config.features.feature {
                                let (icon, color) = if f.active { ("✓", theme.green) } else { ("✗", theme.red) };
                                ui.label(egui::RichText::new(icon).size(14.0).color(color).strong());
                                ui.label(egui::RichText::new(&f.name).size(14.0).color(theme.text).strong());
                                ui.label(
                                    egui::RichText::new(if f.active { "enabled" } else { "disabled" })
                                        .size(12.0)
                                        .color(if f.active { theme.green } else { theme.text_muted }),
                                );
                                ui.end_row();
                            }
                        });
                });

                ui.add_space(12.0);

                card(ui, "👥 Users", &theme, |ui| {
                    for u in &self.config.users.user {
                        let role_color = role_color(&u.role, &theme);
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
                                    .color(theme.text)
                                    .strong(),
                            )
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.add_space(4.0);
                                egui::Grid::new(format!("user_{}", u.id))
                                    .striped(true)
                                    .min_col_width(60.0)
                                    .show(ui, |ui| {
                                        row_strip(ui, "ID", &u.id.to_string(), &theme);
                                        ui.end_row();
                                        row_strip(ui, "Role", &u.role, &theme);
                                        ui.end_row();
                                        row_strip(ui, "Email", &u.email, &theme);
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

impl WasmandeguiApp {
    fn toggle_theme(&mut self, ctx: &egui::Context) {
        self.theme = if self.theme.is_dark { Theme::light() } else { Theme::dark() };
        self.theme.apply(ctx);
    }
}

fn role_color(role: &str, theme: &Theme) -> Color32 {
    match role {
        "admin" => theme.gold,
        "editor" => theme.blue,
        _ => theme.gray,
    }
}

fn card_frame(ui: &mut egui::Ui, theme: &Theme, add_contents: impl FnOnce(&mut egui::Ui)) {
    Frame {
        fill: theme.surface,
        corner_radius: CornerRadius::same(8),
        stroke: Stroke::new(1.0, theme.border),
        inner_margin: Margin::symmetric(14, 10),
        ..Default::default()
    }
    .show(ui, add_contents);
}

fn card(ui: &mut egui::Ui, title: &str, theme: &Theme, add_contents: impl FnOnce(&mut egui::Ui)) {
    Frame {
        fill: theme.surface,
        corner_radius: CornerRadius::same(8),
        stroke: Stroke::new(1.0, theme.border),
        inner_margin: Margin::symmetric(0, 0),
        ..Default::default()
    }
    .show(ui, |ui| {
        Frame {
            fill: theme.accent.gamma_multiply(0.12f32),
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
                    .color(theme.accent)
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

fn side_section(ui: &mut egui::Ui, title: &str, theme: &Theme, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(
        egui::RichText::new(title).size(11.0).color(theme.accent).strong(),
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

fn row(ui: &mut egui::Ui, label: &str, value: &str, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).size(12.0).color(theme.text_muted));
        ui.label(egui::RichText::new(format!("  {}", value)).size(12.0).color(theme.text));
    });
}

fn row_strip(ui: &mut egui::Ui, label: &str, value: &str, theme: &Theme) {
    ui.label(egui::RichText::new(label).size(13.0).color(theme.text_muted).strong());
    let is_alt = ui.style().visuals.widgets.noninteractive.bg_fill == theme.surface;
    let bg = if is_alt { theme.card } else { theme.card_alt };
    Frame {
        fill: bg,
        corner_radius: CornerRadius::same(4),
        inner_margin: Margin::symmetric(6, 2),
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.label(egui::RichText::new(value).size(13.0).color(theme.text));
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

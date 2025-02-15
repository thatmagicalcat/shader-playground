use std::sync::Arc;
use std::sync::Mutex;

use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::Layout;
use eframe::egui::RichText;
use eframe::glow;

use crate::opengl_renderer::OpenGLRenderer;

const FRAGMENT_BOILERPLATE: &str = r#"#version 300 es
precision mediump float;
uniform vec2 u_resolution;
"#;

pub struct App {
    opengl_renderer: Arc<Mutex<OpenGLRenderer>>,

    shader_code: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let shader = include_str!("shader.fs");
        Self {
            shader_code: shader.to_string(),
            opengl_renderer: Arc::new(Mutex::new(OpenGLRenderer::new(
                Arc::clone(
                    cc.gl
                        .as_ref()
                        .expect("You need to run eframe with glow backend"),
                ),
                include_str!("shader.vs"),
                &format!("{FRAGMENT_BOILERPLATE}{shader}"),
            ))),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let height = ui.available_height();
            let width = ui.available_width();

            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(width * 0.6, height),
                    Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.set_min_width(width * 0.6);
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                                ui.heading("Shader Editor")
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("Compile").clicked() {
                                    self.opengl_renderer
                                        .lock()
                                        .unwrap()
                                        .recompile_fragment_shader(&format!(
                                            "{FRAGMENT_BOILERPLATE}{}",
                                            self.shader_code
                                        ));
                                }
                            });
                        });

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.shader_code)
                                    .code_editor()
                                    .min_size(egui::vec2(
                                        ui.available_width(),
                                        ui.available_height() - 90.0,
                                    )),
                            );
                        });

                        ui.push_id(10, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.set_min_height(ui.available_height());
                                if let Some(error) =
                                    self.opengl_renderer.lock().unwrap().get_error()
                                {
                                    ui.label(
                                        RichText::new(format!(
                                            "Failed to compile shader\nerror: {error}"
                                        ))
                                        .color(Color32::RED),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new("Shader compilation was successful")
                                            .color(Color32::GREEN),
                                    );
                                }
                            });
                        });
                    },
                );

                ui.separator();

                let width = ui.available_width();
                ui.allocate_ui_with_layout(
                    egui::vec2(width, height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.heading("Shader Output");
                        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                            let (rect, _response) =
                                ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

                            let renderer = self.opengl_renderer.clone();

                            let callback = egui::PaintCallback {
                                rect,
                                callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                                    move |_, _| {
                                        renderer.lock().unwrap().paint(
                                            egui::Rect::from_x_y_ranges(
                                                rect.min.x..=rect.min.x + width,
                                                rect.y_range(),
                                            ),
                                        );
                                    },
                                )),
                            };

                            ui.painter().add(callback);
                        });
                    },
                );
            });
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.opengl_renderer.lock().unwrap().destroy(gl);
        }
    }
}

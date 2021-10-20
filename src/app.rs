use eframe::{egui, epi};
use super::compendium::Character;
use super::compendium::srd;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    character: Character,
    #[serde(skip)]
    mode: Mode,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum Mode { New, Display, Edit, SaveAs }

impl Default for App {
    fn default() -> Self {
        Self {
            character: Character::default(),
            mode: Mode::Display,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "D&D 5e Interactive Character Sheet"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        self.mode = Mode::Display;
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { character, mode: _ } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("New").clicked() {
                        self.mode = Mode::New;
                    }
                    if ui.button("Level Up").clicked() {
                        egui::Window::new("Window").resizable(false).title_bar(false).show(ctx, |ui| {
                            ui.label("Windows can be moved by dragging them.");

                            if ui.button("OK").clicked() {
                                self.mode = Mode::Display;
                            };
                        });
                    }
                    if ui.button("Save As").clicked() {
                        self.mode = Mode::SaveAs;
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                match self.mode {
                    Mode::Display => if ui.button("Edit").clicked() {
                        self.mode = Mode::Edit;
                    }
                    Mode::Edit => if ui.button("Save").clicked() {
                        self.mode = Mode::Display;
                    }
                    _ => {}
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                Mode::Display | Mode::SaveAs => {
                    ui.heading(&character.name);
                    ui.horizontal(|ui| {
                        ui.label(&character.gender);
                        ui.label(&character.race.name);
                        ui.label(format!("{} {}", character.class.name, character.level));
                    });
                    ui.horizontal(|ui| {
                        let rect = ui.allocate_at_least(egui::Vec2 {x:64.0, y:86.0}, egui::Sense::hover()).0;
                        ui.painter().rect(
                            rect,
                            13.0,
                            egui::Color32::from_gray(64),
                            egui::Stroke::new(1.0, egui::Color32::WHITE)
                        );
                        ui.painter().rect(
                            rect,
                            13.0,
                            egui::Color32::from_gray(64),
                            egui::Stroke::new(1.0, egui::Color32::WHITE)
                        );
                        ui.label("STRENGTH");
                        
                        egui::Area::new("str").show(ctx, |ui| {
                            ui.label(character.abilities.strength.to_string());
                        });
                    });
                }
                Mode::Edit => {
                    ui.vertical(|ui| {
                        ui.text_edit_singleline(&mut character.name);
                        ui.text_edit_singleline(&mut character.gender);

                        egui::ComboBox::from_id_source("race")
                            .selected_text(&character.race.name)
                            .show_ui(ui, |ui| {
                                for i in srd::get_races().iter().map(|e| e.name.clone()) {
                                    ui.selectable_value(&mut character.race.name, i.clone(), i);
                                }
                            }
                        );
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_source("class")
                                .selected_text(&character.class.name)
                                .show_ui(ui, |ui| {
                                    for i in srd::get_classes().iter().map(|e| e.name.clone()) {
                                        ui.selectable_value(&mut character.class.name, i.clone(), i);
                                    }
                                }
                            );
                            ui.add(egui::DragValue::new(&mut character.level).clamp_range(0..=20).speed(0.1));
                        });
                    });
                }
                _ => (),
            };
        });

        match self.mode {
            Mode::New => {
                egui::Window::new("Window").resizable(false).title_bar(false).show(ctx, |ui| {
                    egui::ComboBox::from_id_source("race")
                        .selected_text(&character.race.name)
                        .show_ui(ui, |ui| {
                            for i in srd::get_races().iter().map(|e| e.name.clone()) {
                                ui.selectable_value(&mut character.race.name, i.clone(), i);
                            }
                        }
                    );
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("class")
                            .selected_text(&character.class.name)
                            .show_ui(ui, |ui| {
                                for i in srd::get_classes().iter().map(|e| e.name.clone()) {
                                    ui.selectable_value(&mut character.class.name, i.clone(), i);
                                }
                            }
                        );
                    });

                    if ui.button("OK").clicked() {
                        self.mode = Mode::Display;
                    };
                });
            }
            Mode::SaveAs => {
                egui::Window::new("Window").resizable(false).title_bar(false).show(ctx, |ui| {
                    ui.label("Windows can be moved by dragging them.");
                    ui.label("They are automatically sized based on contents.");
                    ui.label("You can turn on resizing and scrolling if you like.");
                    ui.label("You would normally chose either panels OR windows.");
                    if ui.button("OK").clicked() {
                        self.mode = Mode::Display;
                    };
                });
            }
            _ => {}
        }
    }
}

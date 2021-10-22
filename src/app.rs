use eframe::{egui, epi};
use crate::compendium::srd;
// use crate::widgets;
use crate::{character::Proficiency, Character};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub character: Character,
    #[serde(skip)]
    pub mode: Mode,
    #[serde(skip)]
    pub dirty: bool,
    #[serde(skip)]
    pub tmp: Character,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Mode { Display, New, Edit, Save }

impl Default for App {
    fn default() -> Self {
        Self {
            character: Character::default(),
            mode: Mode::Display,
            dirty: false,
            tmp: Character::default(),
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
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { character, mode, dirty: _, tmp } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                match *mode {
                    Mode::Display => {
                        if ui.button("New").clicked() {
                            *tmp = character.clone();
                            *character = Character::default();
                            *mode = Mode::New;
                        }
                        if ui.button("Edit").clicked() {
                            *tmp = character.clone();
                            *mode = Mode::Edit;
                        }
                        if ui.button("Save").clicked() {
                            std::fs::write(
                                format!("{}_the_{}_{}.charsheet",
                                    character.name,
                                    character.race.name,
                                    character.class.name
                                ),
                                ron::to_string(&character).unwrap()
                            ).expect("failed to write to file");
                        }
                        if ui.button("Load").clicked() {
                            //*mode = Mode::New;
                        }
                        if character.level < 20 {
                            if ui.button("Level Up").clicked() {
                                character.level += 1;
                            }
                        } else {
                            ui.add(egui::Button::new("Level Up").enabled(false));
                        }
                    }
                    Mode::New | Mode::Edit => {
                        ui.add(egui::Button::new("New").enabled(false));
                        ui.add(egui::Button::new("Edit").enabled(false));
                        ui.add(egui::Button::new("Save").enabled(false));
                        ui.add(egui::Button::new("Load").enabled(false));
                        ui.add(egui::Button::new("Level Up").enabled(false));
                    }
                    _ => {}
                }
                ui.separator();
                match *mode {
                    Mode::Display => {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    }
                    Mode::New | Mode::Edit => {
                        if ui.button("Done").clicked() {
                            character.race = srd::get_race(&character.race.name).unwrap();
                            character.class = srd::get_class(&character.class.name).unwrap();
                            character.background = srd::get_background(&character.background.name).unwrap();
                            *mode = Mode::Display;
                        }
                        if ui.button("Cancel").clicked() {
                            *character = tmp.clone();
                            *mode = Mode::Display;
                        }
                    }
                    _ => {}
                }
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        // main UI
        egui::CentralPanel::default().show(ctx, |ui| {
            match *mode {
                Mode::Display => {
                    ui.heading(&character.name);
                    ui.horizontal(|ui| {
                        ui.label(&character.gender);
                        ui.label(&character.race.name);
                        ui.label(format!("{} {}", character.class.name, character.level));
                    });
                    ui.label(&character.background.name);
                    ui.heading(format!("+{}", &character.proficiency_bonus()));

                    ui.horizontal(|ui| {
                        fn ab(title: &str, value: u8, ui: &mut egui::Ui) {
                            ui.vertical(|ui| {
                                ui.set_width(64.0);
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(title);
                                    ui.label(format!("{}{}",
                                        if value > 9 { "+" } else { "" },
                                        Character::calc_mod(value))
                                    );
                                    ui.label(value.to_string());
                                });
                            });
                        }

                        ab("STR", character.abilities().strength, ui);
                        ab("DEX", character.abilities().dexterity, ui);
                        ab("CON", character.abilities().constitution, ui);
                        ab("INT", character.abilities().intelligence, ui);
                        ab("WIS", character.abilities().wisdom, ui);
                        ab("CHA", character.abilities().charisma, ui);
                    });

                    ui.vertical(|ui| {
                        let sk = |title: &str, ability: u8, value: Proficiency, ui: &mut egui::Ui| {
                            let pb = character.proficiency_bonus() as i8;

                            ui.horizontal(|ui| {
                                let mut bonus = Character::calc_mod(ability);
                                let icon;

                                match value {
                                    Proficiency::None => {
                                        icon = "○";
                                    }
                                    Proficiency::Proficient => {
                                        icon = "⏺";
                                        bonus += pb;
                                    }
                                    Proficiency::Expert => {
                                        icon = "★";
                                        bonus += pb * 2;
                                    }
                                }
                                ui.label(icon);
                                ui.label(title);
                                ui.label(format!("{}{}",
                                    if ability > 9 { "+" } else { "" },
                                    bonus
                                ));
                            });
                        };

                        sk("Acrobatics", character.abilities().dexterity,
                            character.skills().acrobatics, ui);
                        sk("Animal Handling", character.abilities().wisdom,
                            character.skills().animal_handling, ui);
                        sk("Arcana", character.abilities().intelligence,
                            character.skills().arcana, ui);
                        sk("Athletics", character.abilities().strength,
                            character.skills().athletics, ui);
                        sk("Deception", character.abilities().charisma,
                            character.skills().deception, ui);
                        sk("History", character.abilities().intelligence,
                            character.skills().history, ui);
                        sk("Insight", character.abilities().wisdom,
                            character.skills().insight, ui);
                        sk("Intimidation", character.abilities().charisma,
                            character.skills().intimidation, ui);
                        sk("Investigation", character.abilities().intelligence,
                            character.skills().investigation, ui);
                        sk("Medicine", character.abilities().wisdom,
                            character.skills().medicine, ui);
                        sk("Nature", character.abilities().intelligence,
                            character.skills().nature, ui);
                        sk("Perception", character.abilities().wisdom,
                            character.skills().perception, ui);
                        sk("Performance", character.abilities().charisma,
                            character.skills().performance, ui);
                        sk("Persuision", character.abilities().charisma,
                            character.skills().persuasion, ui);
                        sk("Religion", character.abilities().intelligence,
                            character.skills().religion, ui);
                        sk("Sleight of Hand", character.abilities().dexterity,
                            character.skills().sleight_of_hand, ui);
                        sk("Stealth", character.abilities().dexterity,
                            character.skills().stealth, ui);
                        sk("Survival", character.abilities().wisdom,
                            character.skills().survival, ui);
                    });
                }
                Mode::New => {
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
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("background")
                            .selected_text(&character.background.name)
                            .show_ui(ui, |ui| {
                                for i in srd::get_backgrounds().iter().map(|e| e.name.clone()) {
                                    ui.selectable_value(&mut character.background.name, i.clone(), i);
                                }
                            }
                        );
                    });
                    ui.horizontal(|ui| {
                        fn ab(title: &str, value: &mut u8, ui: &mut egui::Ui) {
                            ui.vertical(|ui| {
                                ui.set_width(64.0);
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(title);
                                    ui.add(egui::DragValue::new(value)
                                        .clamp_range(3..=18).speed(0.1));
                                });
                            });
                        }

                        ab("STR", &mut character.abilities.strength, ui);
                        ab("DEX", &mut character.abilities.dexterity, ui);
                        ab("CON", &mut character.abilities.constitution, ui);
                        ab("INT", &mut character.abilities.intelligence, ui);
                        ab("WIS", &mut character.abilities.wisdom, ui);
                        ab("CHA", &mut character.abilities.charisma, ui);
                    });
                }
                Mode::Edit => {
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
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("background")
                            .selected_text(&character.background.name)
                            .show_ui(ui, |ui| {
                                for i in srd::get_backgrounds().iter().map(|e| e.name.clone()) {
                                    ui.selectable_value(&mut character.background.name, i.clone(), i);
                                }
                            }
                        );
                    });
                    ui.horizontal(|ui| {
                        fn ab(title: &str, value: &mut u8, ui: &mut egui::Ui) {
                            ui.vertical(|ui| {
                                ui.set_width(64.0);
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(title);
                                    if *value < 20 {
                                        if ui.button("⏶").clicked() {
                                            *value += 1;
                                        }
                                    } else {
                                        ui.add(egui::Button::new("⏶").enabled(false));
                                    }
                                    ui.label(value.to_string());
                                    if *value > 3 {
                                        if ui.button("⏷").clicked() {
                                            *value -= 1;
                                        }
                                    } else {
                                        ui.add(egui::Button::new("⏷").enabled(false));
                                    }
                                });
                            });
                        }

                        ab("STR", &mut character.abilities.strength, ui);
                        ab("DEX", &mut character.abilities.dexterity, ui);
                        ab("CON", &mut character.abilities.constitution, ui);
                        ab("INT", &mut character.abilities.intelligence, ui);
                        ab("WIS", &mut character.abilities.wisdom, ui);
                        ab("CHA", &mut character.abilities.charisma, ui);
                    });
                }
                _ => (),
            };
        });
    }
}

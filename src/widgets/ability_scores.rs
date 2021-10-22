use std::cell::RefCell;
use eframe::egui;
use crate::compendium::{Character};
use crate::{App, Mode};

pub struct AbilityScores {}

impl AbilityScores {
    pub fn show(app: &mut App, ui: &mut egui::Ui) {
        match app.mode {
            Mode::Display => AbilityScores::show_display(app, ui),
            Mode::New => AbilityScores::show_new(app, ui),
            Mode::Edit => AbilityScores::show_edit(app, ui),
            _ => (),
        }
    }

    fn show_display(app: &mut App, ui: &mut egui::Ui) {
        let App { character, mode, dirty: _, tmp } = app;

        fn s(title: &str, value: u8) {
            |mut ui: egui::Ui| {
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
            };
        }
        ui.horizontal(|ui| {
            s("NOT", 8);
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("STR");
                    let v = character.abilities().strength;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("DEX");
                    let v = character.abilities().dexterity;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CON");
                    let v = character.abilities().constitution;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("INT");
                    let v = character.abilities().intelligence;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("WIS");
                    let v = character.abilities().wisdom;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CHA");
                    let v = character.abilities().charisma;
                    ui.label(format!("{}{}",
                        if v > 9 { "+" } else { "" },
                        Character::calc_mod(v))
                    );
                    ui.label(v.to_string());
                });
            });
        });
    }

    fn show_new(app: &mut App, ui: &mut egui::Ui) {
        let App { character, mode, dirty: _, tmp } = app;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("STR");
                    ui.add(egui::DragValue::new(&mut character.abilities.strength)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("DEX");
                    ui.add(egui::DragValue::new(&mut character.abilities.dexterity)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CON");
                    ui.add(egui::DragValue::new(&mut character.abilities.constitution)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("INT");
                    ui.add(egui::DragValue::new(&mut character.abilities.intelligence)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("WIS");
                    ui.add(egui::DragValue::new(&mut character.abilities.wisdom)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CHA");
                    ui.add(egui::DragValue::new(&mut character.abilities.charisma)
                        .clamp_range(3..=18).speed(0.1));
                });
            });
        });
    }

    fn show_edit(app: &mut App, ui: &mut egui::Ui) {
        let App { character, mode, dirty: _, tmp } = app;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("STR");
                    let v = &mut character.abilities.strength;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("DEX");
                    let v = &mut character.abilities.dexterity;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CON");
                    let v = &mut character.abilities.constitution;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("INT");
                    let v = &mut character.abilities.intelligence;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("WIS");
                    let v = &mut character.abilities.wisdom;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
            ui.vertical(|ui| {
                ui.set_width(64.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("CHA");
                    let v = &mut character.abilities.charisma;
                    if *v < 20 {
                        if ui.button("⏶").clicked() {
                            *v += 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏶").enabled(false));
                    }
                    ui.label(v.to_string());
                    if *v > 3 {
                        if ui.button("⏷").clicked() {
                            *v -= 1;
                        }
                    } else {
                        ui.add(egui::Button::new("⏷").enabled(false));
                    }
                });
            });
        });
    }
}
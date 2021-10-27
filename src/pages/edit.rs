use crate::character::{self, Ability, Skill};
use crate::compendium::{backgrounds, classes, races};
use crate::{App, Mode};
use eframe::egui;

pub fn show(app: &mut App, ctx: &egui::CtxRef) {
    let App {
        character,
        mode,
        dirty: _,
        tmp,
    } = app;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.text_edit_singleline(&mut character.name);
        ui.text_edit_singleline(&mut character.alignment);
        ui.text_edit_singleline(&mut character.gender);

        egui::ComboBox::from_id_source("race")
            .width(256.0)
            .selected_text(&character.race.name)
            .show_ui(ui, |ui| {
                for race in races::get_races().iter() {
                    match &race.subraces {
                        Some(subraces) => {
                            ui.label(&race.name);
                            for subrace in subraces {
                                ui.selectable_value(
                                    &mut character.race.name,
                                    subrace.name.clone(),
                                    format!("  {}", subrace.name),
                                );
                            }
                        }
                        None => {
                            ui.selectable_value(
                                &mut character.race.name,
                                race.name.clone(),
                                race.name.clone(),
                            );
                        }
                    }
                }
            });

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("class")
                .width(256.0)
                .selected_text(&character.class.name)
                .show_ui(ui, |ui| {
                    for i in classes::get_classes().iter().map(|e| e.name.clone()) {
                        ui.selectable_value(&mut character.class.name, i.clone(), i);
                    }
                });
            if character.class.name != tmp.class.name {
                character.spec.name = "None".to_string();
            }
            if *mode == Mode::Edit {
                ui.add(
                    egui::DragValue::new(&mut character.level)
                        .clamp_range(0..=20)
                        .speed(0.1),
                );
            }
        });
        ui.horizontal(|ui| {
            if *mode == Mode::Edit {
                ui.label(" ");
                egui::ComboBox::from_id_source("subclass")
                    .width(256.0)
                    .selected_text(&character.spec.name)
                    .show_ui(ui, |ui| {
                        for i in classes::get_class(&character.class.name)
                            .unwrap()
                            .subclasses
                            .iter()
                            .map(|e| e.name.clone())
                        {
                            ui.selectable_value(&mut character.spec.name, i.clone(), i);
                        }
                    });
                ui.horizontal(|ui| {
                    ui.label(" ".to_string());
                });
            }
        });

        ui.label(" ");

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("background")
                .width(256.0)
                .selected_text(&character.background.name)
                .show_ui(ui, |ui| {
                    for i in backgrounds::get_backgrounds()
                        .iter()
                        .map(|e| e.name.clone())
                    {
                        ui.selectable_value(&mut character.background.name, i.clone(), i);
                    }
                });
        });

        ui.horizontal(|ui| {
            let mut ab = |ability: Ability, ui: &mut egui::Ui| match *mode {
                Mode::New => {
                    ui.vertical(|ui| {
                        ui.set_width(64.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(ability.to_string_short());
                            ui.add(
                                egui::DragValue::new(character.ability_score_mut(ability))
                                    .clamp_range(3..=18)
                                    .speed(0.1),
                            );
                        });
                    });
                }
                Mode::Edit => {
                    ui.vertical(|ui| {
                        ui.set_width(64.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            let val = character.ability_score_mut(ability);

                            ui.heading(ability.to_string_short());
                            if *val < 20 {
                                if ui.button("⏶").clicked() {
                                    *val += 1;
                                }
                            } else {
                                ui.add(egui::Button::new("⏶").enabled(false));
                            }
                            ui.label(val.to_string());
                            if *val > 3 {
                                if ui.button("⏷").clicked() {
                                    *val -= 1;
                                }
                            } else {
                                ui.add(egui::Button::new("⏷").enabled(false));
                            }
                        });
                    });
                }
                _ => unreachable!(),
            };

            ab(Ability::Strength, ui);
            ab(Ability::Dexterity, ui);
            ab(Ability::Constitution, ui);
            ab(Ability::Intelligence, ui);
            ab(Ability::Wisdom, ui);
            ab(Ability::Charisma, ui);
        });

        if *mode == Mode::Edit {
            ui.vertical(|ui| {
                let mut sk = |skill: Skill, ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        if ui.button(character.skill_level(skill).icon()).clicked() {
                            character.skill_level_mut(skill).learn();
                        }
                        ui.label(skill.to_string());
                        ui.label(character::mod_str(character.skill_mod(skill)));
                    });
                };

                sk(Skill::Acrobatics, ui);
                sk(Skill::AnimalHandling, ui);
                sk(Skill::Arcana, ui);
                sk(Skill::Athletics, ui);
                sk(Skill::Deception, ui);
                sk(Skill::History, ui);
                sk(Skill::Insight, ui);
                sk(Skill::Intimidation, ui);
                sk(Skill::Investigation, ui);
                sk(Skill::Medicine, ui);
                sk(Skill::Nature, ui);
                sk(Skill::Perception, ui);
                sk(Skill::Performance, ui);
                sk(Skill::Persuasion, ui);
                sk(Skill::Religion, ui);
                sk(Skill::SleightOfHand, ui);
                sk(Skill::Stealth, ui);
                sk(Skill::Survival, ui);
            });
        }
    });
}

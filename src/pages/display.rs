use crate::character::{self, Ability, Skill};
use crate::App;
use eframe::egui;

pub fn show_char_bar(app: &mut App, ctx: &egui::CtxRef) {
    let App {
        character,
        mode: _,
        dirty: _,
        tmp: _,
    } = app;

    egui::TopBottomPanel::top("char").frame(egui::Frame{
        margin: egui::Vec2 {x: 10.0, y: 4.0},
        corner_radius: Default::default(),
        shadow: Default::default(),
        fill: egui::Color32::from_rgb(0x1b, 0x1b, 0x1b),
        stroke: egui::Stroke::none(),
    }).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(260.0);
                ui.heading(&character.name);
                ui.horizontal(|ui| {
                    ui.label(&character.alignment);
                    ui.label(&character.gender);
                    ui.label(&character.race.name);
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{} {}", character.class.name, character.level));
                    match character.spec.name.as_str() {
                        "None" => {}
                        s => {
                            ui.label(format!("({})", s));
                        }
                    }
                });
                ui.label(&character.background.name);
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.set_width(100.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label("Armor Class");
                            ui.heading(character.ac().to_string());
                        });
                    });
                    ui.vertical(|ui| {
                        ui.set_width(100.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label("Initiative".to_string());
                            ui.heading(character::mod_str(
                                character.ability_mod(Ability::Dexterity),
                            ));
                        });
                    });
                    ui.vertical(|ui| {
                        ui.set_width(100.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label("Speed".to_string());
                            ui.heading(character.speed().to_string());
                        });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.set_width(100.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label(format!("Hit Points ({})", character.max_hp));
                            ui.heading(character.hp.to_string());
                        });
                    });
                    ui.vertical(|ui| {
                        ui.set_width(100.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label(format!("Hit Dice (d{})", character.class.hit_die));
                            ui.heading(character.level.to_string());
                        });
                    });
                    if let Some(a) = character.class.spellcaster {
                        ui.vertical(|ui| {
                            ui.set_width(100.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                ui.label(format!("Spell DC ({})", a.to_string_short()));
                                ui.heading(
                                    (8 + character.ability_mod(a)
                                        + character.proficiency_bonus() as i8)
                                        .to_string(),
                                );
                            });
                        });
                    }
                    ui.separator();
                });
                ui.separator();
            });
        });
        ui.separator();
    });
}

pub fn show(app: &mut App, ctx: &egui::CtxRef) {
    show_char_bar(app, ctx);

    let App {
        character,
        mode: _,
        dirty: _,
        tmp: _,
    } = app;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.set_height(ctx.available_rect().bottom() - 130.0);
            ui.vertical(|ui| {
                ui.set_width(80.0);
                let ab = |ability: Ability, ui: &mut egui::Ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(5.0);
                        ui.label(ability.to_string());
                        ui.heading(character::mod_str(character.ability_mod(ability)));
                        ui.label(character.ability_score(ability).to_string());
                        ui.add_space(5.0);
                        ui.separator();
                    });
                };

                ab(Ability::Strength, ui);
                ab(Ability::Dexterity, ui);
                ab(Ability::Constitution, ui);
                ab(Ability::Intelligence, ui);
                ab(Ability::Wisdom, ui);
                ab(Ability::Charisma, ui);
            });
            ui.separator();

            ui.vertical(|ui| {
                ui.set_width(160.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label("Proficiency Bonus".to_string());
                    ui.heading(format!("+{}", &character.proficiency_bonus()));
                });
                let sv = |ability: Ability, ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        ui.label(character.save(ability).icon());
                        ui.label(character::mod_str(character.save_mod(ability)));
                        ui.label(ability.to_string());
                    });
                };
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.separator();
                    ui.heading("Saving Throws");
                    sv(Ability::Strength, ui);
                    sv(Ability::Dexterity, ui);
                    sv(Ability::Constitution, ui);
                    sv(Ability::Intelligence, ui);
                    sv(Ability::Wisdom, ui);
                    sv(Ability::Charisma, ui);
                });

                let sk = |skill: Skill, ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        ui.label(character.skill_level(skill).icon());
                        ui.label(character::mod_str(character.skill_mod(skill)));
                        ui.label(skill.to_string());
                        ui.small(format!("({})", skill.attr().to_string_short()));
                    });
                };
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.separator();
                    ui.heading("Skills");
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
            });

            ui.separator();

            ui.vertical(|ui| {
                let w = (ctx.available_rect().right() - 340.0) * 0.5;
                ui.set_width(w);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Racial Traits");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("racial_traits")
                    .show(ui, |ui| {
                        for i in character.race.get_modifiers(|k, _v| !k.starts_with('.')) {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(100.0);
                                    ui.label(i.0);
                                    ui.label(String::new());
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w - 100.0);
                                    ui.label(i.1);
                                    ui.label(String::new());
                                });
                            });
                        }
                    });
            });

            ui.separator();

            ui.vertical(|ui| {
                let w = (ctx.available_rect().right() - 280.0) * 0.5;
                ui.set_width(w);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Class Features");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("class_features")
                    .show(ui, |ui| {
                        for i in character
                            .class
                            .get_modifiers(character.level, |k, _v| !k.starts_with('.'))
                        {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(100.0);
                                    ui.label(i.0);
                                    ui.label(String::new());
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w - 140.0);
                                    ui.label(i.1);
                                    ui.label(String::new());
                                });
                            });
                        }
                        ui.heading(&character.spec.name);
                        for i in character
                            .class
                            .get_subclass(&character.spec.name)
                            .unwrap_or_default()
                            .get_modifiers(character.level, |k, _v| !k.starts_with('.'))
                        {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(100.0);
                                    ui.label(i.0);
                                    ui.label(String::new());
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w - 140.0);
                                    ui.label(i.1);
                                    ui.label(String::new());
                                });
                            });
                        }
                    });
            });
        });
    });
}

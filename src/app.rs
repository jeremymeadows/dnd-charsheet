use crate::character::{self, Ability, Character, Skill};
use crate::compendium::{backgrounds, classes, races};
use eframe::{egui, epi};
// use crate::widgets;

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

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Mode {
    Display,
    New,
    Edit,
    Save,
}

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
        let Self {
            character,
            mode,
            dirty: _,
            tmp,
        } = self;

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
                                format!(
                                    "{}_the_{}_{}.charsheet",
                                    character.name, character.race.name, character.class.name
                                ),
                                ron::to_string(&character).unwrap(),
                            )
                            .expect("failed to write to file");
                        }
                        if ui.button("Load").clicked() {
                            // *mode = Mode::Load;
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
                            character.race = races::get_race(&character.race.name).unwrap();
                            character.class = classes::get_class(&character.class.name).unwrap();
                            character.background =
                                backgrounds::get_background(&character.background.name).unwrap();

                            if *mode == Mode::New {
                                character.max_hp = (character.class.hit_die as i8
                                    + character.ability_mod(Ability::Constitution))
                                    as u16;
                                character.hp = character.max_hp as i16;
                            }
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

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(80.0);
                            let ab = |ability: Ability, ui: &mut egui::Ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.label(ability.to_string());
                                    ui.heading(character::mod_str(character.ability_mod(ability)));
                                    ui.label(character.ability_score(ability).to_string());
                                });
                            };

                            ab(Ability::Strength, ui);
                            ab(Ability::Dexterity, ui);
                            ab(Ability::Constitution, ui);
                            ab(Ability::Intelligence, ui);
                            ab(Ability::Wisdom, ui);
                            ab(Ability::Charisma, ui);
                        });

                        ui.vertical(|ui| {
                            ui.set_width(176.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                ui.label("Proficiency Bonus".to_string());
                                ui.heading(format!("+{}", &character.proficiency_bonus()));
                                ui.label("".to_string());
                            });
                            let sk = |skill: Skill, ui: &mut egui::Ui| {
                                ui.horizontal(|ui| {
                                    ui.label(character.skill_level(skill).icon());
                                    ui.label(character::mod_str(character.skill_mod(skill)));
                                    ui.label(skill.to_string());
                                    ui.label(format!("({})", skill.attr().to_string_short()));
                                });
                            };

                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
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
                        ui.vertical(|ui| {
                            ui.set_width(240.0);
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(80.0);
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            ui.label("Armor Class".to_string());
                                            ui.heading(character.ac().to_string());
                                        },
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(80.0);
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            ui.label("Initiative".to_string());
                                            ui.heading(character::mod_str(
                                                character.ability_mod(Ability::Dexterity),
                                            ));
                                        },
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(80.0);
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            ui.label("Speed".to_string());
                                            ui.heading(character.speed().to_string());
                                        },
                                    );
                                });
                            });
                        });
                    });
                }
                Mode::New => {
                    ui.text_edit_singleline(&mut character.name);
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
                                                // format!("{}|{}", race.name, subrace.name),
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
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("background")
                            .width(256.0)
                            .selected_text(&character.background.name)
                            .show_ui(ui, |ui| {
                                for i in backgrounds::get_backgrounds()
                                    .iter()
                                    .map(|e| e.name.clone())
                                {
                                    ui.selectable_value(
                                        &mut character.background.name,
                                        i.clone(),
                                        i,
                                    );
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        let mut ab = |ability: Ability, ui: &mut egui::Ui| {
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
                        };

                        ab(Ability::Strength, ui);
                        ab(Ability::Dexterity, ui);
                        ab(Ability::Constitution, ui);
                        ab(Ability::Intelligence, ui);
                        ab(Ability::Wisdom, ui);
                        ab(Ability::Charisma, ui);
                    });
                }
                Mode::Edit => {
                    ui.text_edit_singleline(&mut character.name);
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
                                                // format!("{}|{}", race.name, subrace.name),
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
                        ui.add(
                            egui::DragValue::new(&mut character.level)
                                .clamp_range(0..=20)
                                .speed(0.1),
                        );
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("background")
                            .width(256.0)
                            .selected_text(&character.background.name)
                            .show_ui(ui, |ui| {
                                for i in backgrounds::get_backgrounds()
                                    .iter()
                                    .map(|e| e.name.clone())
                                {
                                    ui.selectable_value(
                                        &mut character.background.name,
                                        i.clone(),
                                        i,
                                    );
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        let mut ab = |ability: Ability, ui: &mut egui::Ui| {
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
                        };

                        ab(Ability::Strength, ui);
                        ab(Ability::Dexterity, ui);
                        ab(Ability::Constitution, ui);
                        ab(Ability::Intelligence, ui);
                        ab(Ability::Wisdom, ui);
                        ab(Ability::Charisma, ui);
                    });
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
                _ => (),
            };
        });

        // window
        // match *mode {
        //     Mode::Save => {
        //         egui::Window::new("Window").resizable(false).title_bar(false).show(ctx, |ui| {
        //             ui.label("Windows can be moved by dragging them.");
        //             ui.label("They are automatically sized based on contents.");
        //             ui.label("You can turn on resizing and scrolling if you like.");
        //             ui.label("You would normally chose either panels OR windows.");
        //             if ui.button("OK").clicked() {
        //                 self.mode = Mode::Display;
        //             };
        //         });
        //     }
        //     _ => {}
        // }
    }
}

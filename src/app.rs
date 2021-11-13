use crate::character::{Ability, Character};
use crate::compendium::{backgrounds, classes, races};
use crate::pages;
use eframe::{egui, epi};
use std::fs;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub character: Character,
    #[serde(skip)]
    pub mode: Mode,
    #[serde(skip)]
    pub page: Page,
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
    Load,
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Page {
    Home,
    Equipment,
    Actions,
    Spells,
    Other,
}

impl Default for App {
    fn default() -> Self {
        Self {
            character: Character::default(),
            mode: Mode::Display,
            page: Page::Home,
            dirty: false,
            tmp: Character::default(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "D&D 5e Interactive Character Sheet"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::new(f32::INFINITY, f32::INFINITY)
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        ctx.set_style(std::sync::Arc::new(
            ron::from_str::<egui::Style>(include_str!("style.ron")).unwrap(),
        ));

        // Load previous app state (if any).
        if let Some(storage) = storage {
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
            page,
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
                            *mode = Mode::Load;
                        }
                        if character.level < 20 {
                            if ui.button("Level Up").clicked() {
                                character.level += 1;
                                character.max_hp += ((character.class.hit_die / 2 + 1) as i8
                                    + i8::max(character.ability_mod(Ability::Constitution), 0))
                                    as u16;
                                character.hp = character.max_hp as i16;
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
                            character.spec = character
                                .class
                                .get_subclass(&character.spec.name)
                                .unwrap_or_default();
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

        character.assign_modifiers();

        match mode {
            Mode::Display => match page {
                Page::Home => pages::home::show(self, ctx),
                Page::Equipment => pages::equipment::show(self, ctx),
                Page::Actions => pages::actions::show(self, ctx),
                Page::Spells => pages::spells::show(self, ctx),
                Page::Other => (),
            },
            Mode::New | Mode::Edit => {
                pages::edit::show(self, ctx);
            }
            Mode::Save => {
                #[cfg(not(target_arch = "wasm32"))]
                fs::write(
                    format!(
                        "{}_the_{}_{}.charsheet",
                        character.name, character.race.name, character.class.name
                    ),
                    ron::to_string(&character).unwrap(),
                )
                .expect("failed to write to disk");
            }
            Mode::Load =>
            {
                #[cfg(not(target_arch = "wasm32"))]
                for d in fs::read_dir("./").unwrap() {
                    let file = d.unwrap();
                    if file.file_type().unwrap().is_file() {
                        let filename = file.file_name().into_string().unwrap();
                        if filename.ends_with(".charsheet") {
                            *character =
                                ron::from_str::<Character>(&fs::read_to_string(filename).unwrap())
                                    .unwrap();
                            *mode = Mode::Display;
                            break;
                        }
                    }
                }
            }
        }

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

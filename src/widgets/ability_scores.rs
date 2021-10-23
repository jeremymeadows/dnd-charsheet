use eframe::egui;
use crate::character::{self, Ability};
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

    fn show_display(app: &App, ui: &mut egui::Ui) {
        let App { character, mode: _, dirty: _, tmp: _ } = app;

        ui.horizontal(|ui| {
            let ab = |ability: Ability, ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    ui.set_width(64.0);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.heading(ability.to_string_short());
                        ui.label(character::mod_str(character.ability_mod(ability)));
                        ui.label(character.ability_score(ability).to_string());
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

    fn show_new(app: &mut App, ui: &mut egui::Ui) {
        let App { character, mode, dirty: _, tmp } = app;

    }

    fn show_edit(app: &mut App, ui: &mut egui::Ui) {
        let App { character, mode, dirty: _, tmp } = app;
    }
}
use crate::character::{Ability, DamageType, ItemType, WeaponProperties, WeaponType};
use crate::App;
use eframe::egui;

pub fn show(app: &mut App, ctx: &egui::CtxRef) {
    super::home::show_char_bar(app, ctx);

    let App { character, .. } = app;

    egui::CentralPanel::default().show(ctx, |ui| {
        let h = ctx.available_rect().bottom() - 140.0;
        let w = ctx.available_rect().right() * 0.33;

        ui.horizontal(|ui| {
            ui.set_height(h);

            ui.vertical(|ui| {
                ui.set_width(w);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Attacks");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("attacks")
                    .show(ui, |ui| {
                        let w = w * 0.33;
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width(w);
                                ui.label("Unarmed Strike");
                            });
                            ui.vertical(|ui| {
                                ui.set_width(w * 0.2);
                                ui.label(format!("+{}", character.ability_mod(Ability::Strength) + character.proficiency_bonus() as i8));
                            });
                            ui.vertical(|ui| {
                                ui.set_width(w * 0.3);
                                ui.label("5 ft.");
                            });
                            ui.vertical(|ui| {
                                ui.set_width(w * 1.5);
                                ui.label(
                                    match character
                                        .get_modifiers(".unarmed_strike")
                                        .iter()
                                        .map(|(_k, v)| str::parse::<u8>(v).unwrap())
                                        .max()
                                    {
                                        Some(v) => v,
                                        _ => {
                                            1 + std::cmp::max(
                                                character.ability_mod(Ability::Strength),
                                                0,
                                            ) as u8
                                        }
                                    }
                                    .to_string()
                                        + " "
                                        + &DamageType::Bludgeoning.to_string(),
                                );
                            });
                        });

                        for i in character
                            .items
                            .iter()
                            .filter(|e| matches!(e.item_type, ItemType::Weapon(_)))
                        {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_width(w);
                                    ui.label(i.name.clone());
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w * 0.2);
                                    ui.label(format!("+{}", {
                                        let mut atk = character.ability_mod(Ability::Strength);

                                        if i.properties.contains(&WeaponProperties::Finesse) {
                                            atk = std::cmp::max(
                                                character.ability_mod(Ability::Strength),
                                                character.ability_mod(Ability::Dexterity),
                                            );
                                        }
                                        
                                        atk + character.proficiency_bonus() as i8
                                    }));
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w * 0.3);
                                    ui.label(match i.range.as_ref() {
                                        "" if i.properties.contains(&WeaponProperties::Reach) => {
                                            "10 ft"
                                        }
                                        "" => "5 ft",
                                        _ => &i.range,
                                    });
                                });
                                ui.vertical(|ui| {
                                    ui.set_width(w * 1.5);
                                    ui.label(format!(
                                        "d{}{} {}",
                                        i.damage_die,
                                        if i.properties.contains(&WeaponProperties::Versatile) {
                                            format!(" (d{})", i.damage_die + 2)
                                        } else {
                                            String::new()
                                        },
                                        match i.item_type {
                                            ItemType::Weapon(WeaponType::SimpleMelee(x)) =>
                                                x.to_string(),
                                            ItemType::Weapon(WeaponType::MartialMelee(x)) =>
                                                x.to_string(),
                                            ItemType::Weapon(WeaponType::SimpleRanged(x)) =>
                                                x.to_string(),
                                            ItemType::Weapon(WeaponType::MartialRanged(x)) =>
                                                x.to_string(),
                                            _ => String::new(),
                                        }
                                    ));
                                });
                            });
                        }
                    });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.set_width(w);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Actions");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("actions")
                    .show(ui, |ui| {
                        ui.label("todo");
                    });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.set_width(w);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Bonus Actions");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("bonus_actions")
                    .show(ui, |ui| {
                        ui.label("todo");
                    });
            });
        });
    });
}

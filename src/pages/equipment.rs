use crate::character::ItemType;
use crate::compendium::items;
use crate::App;
use eframe::egui;

pub fn show(app: &mut App, ctx: &egui::CtxRef) {
    let App {
        character,
        mode: _,
        dirty: _,
        tmp: _,
    } = app;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.set_height(ctx.available_rect().bottom() - 40.0);

            ui.vertical(|ui| {
                ui.set_width(ctx.available_rect().right() * 0.5);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("My Equipment");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("equipment")
                    .show(ui, |ui| {
                        let mut i = 0;
                        character.items.sort_by(|a, b| a.name.cmp(&b.name));
                        while i < character.items.len() {
                            let item = character.items[i].clone();
                            ui.horizontal(|ui| {
                                if ui.button(item.equipped.icon()).clicked() {
                                    character.items[i].equipped = !character.items[i].equipped;
                                }
                                if ui.button(item.name.clone()).clicked() {
                                    character.items.remove(
                                        character
                                            .items
                                            .iter()
                                            .position(|e| e.name == item.name)
                                            .unwrap(),
                                    );
                                    i -= 1;
                                }
                            });
                            ui.small(String::new());
                            i += 1;
                        }
                    });
            });

            ui.separator();

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.set_width(ctx.available_rect().right() * 0.5);
                ui.heading("Items");
                egui::containers::ScrollArea::auto_sized()
                    .id_source("items")
                    .show(ui, |ui| {
                        for i in items::get_items()
                            .iter()
                            .filter(|e| e.item_type != ItemType::Currency)
                        {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    // ui.set_width(window.2 / 2.0 * 0.75);
                                    if ui.button(&i.name).clicked() {
                                        character.items.push(i.clone());
                                    }
                                    ui.small(String::new());
                                });
                                ui.vertical(|ui| {
                                    // ui.set_width(window.2 / 2.0 * 0.25);
                                    ui.label(i.value.to_string());
                                    ui.small(String::new());
                                });
                            });
                        }
                    });
            });
        });
    });
}

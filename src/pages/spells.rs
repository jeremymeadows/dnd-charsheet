use crate::compendium::spells;
use crate::App;
use eframe::egui;

pub fn show(app: &mut App, ctx: &egui::CtxRef) {
    super::home::show_char_bar(app, ctx);

    let App { character, .. } = app;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.set_height(ctx.available_rect().bottom() - 140.0);

            ui.vertical(|ui| {
                ui.set_width((ctx.available_rect().right()) * 0.33);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Prepared Spells");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("prep_spells")
                    .show(ui, |ui| {
                        ui.label("todo");
                    });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.set_width((ctx.available_rect().right() - 20.0) * 0.33);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Known Spells");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("spells")
                    .show(ui, |ui| {
                        ui.label("todo");
                    });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.set_width((ctx.available_rect().right() - 40.0) * 0.34);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Spell Details");
                });
                egui::containers::ScrollArea::auto_sized()
                    .id_source("spells_det")
                    .show(ui, |ui| {
                        ui.label("todo");
                    });
            });
        });
    });
}

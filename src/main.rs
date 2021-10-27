#![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all)]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = dnd_charsheet::App::default();
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::epi::IconData {
            rgba: include_bytes!("../icon.bmp").to_vec(),
            height: 334,
            width: 334,
        }),
        initial_window_size: Some(eframe::egui::Vec2::new(970.0, 740.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}

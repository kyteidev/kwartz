use egui::{self, Ui};
use rfd::FileDialog;

use crate::modules::textarea::TextArea;

use super::menu_actions::open_file;

pub fn create_menu(ui: &mut Ui, textarea: &mut TextArea) {
    ui.menu_button("Menu", |ui| {
        if ui.button("New File").clicked() {}
        if ui.button("New Window").clicked() {}
        if ui.button("Open...").clicked() {
            if let Some(file_path) = FileDialog::new().pick_file() {
                open_file(textarea, &file_path.display().to_string());
            }
        }

        ui.separator();

        if ui.button("Save").clicked() {}
        if ui.button("Save As...").clicked() {}

        ui.separator();

        if ui.button("Undo").clicked() {}
        if ui.button("Redo").clicked() {}

        ui.separator();

        if ui.button("Find").clicked() {}
        if ui.button("Replace").clicked() {}
        if ui.button("Select All").clicked() {}

        ui.separator();

        if ui.button("Settings").clicked() {}
        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

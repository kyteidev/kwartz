use egui::{self, Ui};

pub fn create_menu(ui: &mut Ui) {
    ui.menu_button("Menu", |ui| {
        if ui.button("New File").clicked() {}
        if ui.button("New Window").clicked() {}
        if ui.button("Open...").clicked() {}

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

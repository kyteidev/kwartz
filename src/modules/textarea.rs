use egui::TextStyle::Monospace;
use egui::{FontId, Id, TextEdit};

pub struct TextArea {
    pub content: String,
    focused: bool,
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            focused: false,
        }
    }

    pub fn replace_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // style textarea
        let mut style = (*ctx.style()).clone();
        style.text_styles = [(Monospace, FontId::new(24.0, egui::FontFamily::Monospace))].into();
        ui.style_mut().text_styles = style.text_styles;

        let textarea_id = Id::new("textarea");

        let mut textarea = TextEdit::multiline(&mut self.content);
        textarea = textarea.frame(false).code_editor();

        // auto focus on textarea, so user knows they can type there
        if !self.focused {
            ui.memory_mut(|mem| mem.request_focus(textarea_id));
            self.focused = true;
        }

        ui.add_sized(ui.available_size(), textarea.id(textarea_id));
    }
}

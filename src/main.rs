mod modules;
mod utils;

use eframe::egui;
use eframe::App;
use egui::TextureHandle;

use modules::textarea::TextArea;
use modules::title_bar::draw_title_bar;
use utils::load_resized_image;

struct Kwartz {
    close_icon: Option<TextureHandle>,
    minimize_icon: Option<TextureHandle>,

    textarea: TextArea,
}

impl Kwartz {
    fn load_icons(&mut self, ctx: &egui::Context) {
        if self.close_icon.is_none() {
            self.close_icon = Some(load_resized_image(
                ctx,
                include_bytes!("../assets/close.png"),
                (24, 24),
            ));
        }

        if self.minimize_icon.is_none() {
            self.minimize_icon = Some(load_resized_image(
                ctx,
                include_bytes!("../assets/minimize.png"),
                (24, 24),
            ));
        }
    }
}

impl App for Kwartz {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_icons(ctx);

        draw_title_bar(ctx, self);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.textarea.show(ui, ctx);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true),

        ..Default::default()
    };

    eframe::run_native(
        "kwartz",
        options,
        Box::new(|_cc| {
            Ok(Box::new(Kwartz {
                close_icon: None,
                minimize_icon: None,

                textarea: TextArea::new(),
            }))
        }),
    )
}

use eframe::egui;
use eframe::App;
use egui::Color32;
use egui::TextureHandle;
use egui::ViewportCommand;
use image::load_from_memory;

struct Kwartz {
    close_icon: Option<TextureHandle>,
    minimize_icon: Option<TextureHandle>,
}

impl Kwartz {
    fn load_resized_image(ctx: &egui::Context, bytes: &[u8], size: (u32, u32)) -> TextureHandle {
        let image = load_from_memory(bytes).expect("Failed to load image");

        let resized = image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);

        let rgba_image = resized.into_rgba8();

        let size = [size.0 as usize, size.1 as usize];
        let pixels = rgba_image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

        ctx.load_texture("icon", color_image, egui::TextureOptions::LINEAR)
    }

    fn load_icons(&mut self, ctx: &egui::Context) {
        if self.close_icon.is_none() {
            self.close_icon = Some(Self::load_resized_image(
                ctx,
                include_bytes!("../assets/close.png"),
                (24, 24),
            ));
        }

        if self.minimize_icon.is_none() {
            self.minimize_icon = Some(Self::load_resized_image(
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

        egui::TopBottomPanel::top("title_bar").show(ctx, |ui| {
            egui::Frame::none()
                .inner_margin(egui::vec2(-5., 1.))
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.scope(|ui| {
                            // remove margin between window control buttons
                            let mut style: egui::Style = (*ctx.style()).clone();
                            style.spacing.item_spacing = egui::vec2(2., 0.);
                            ctx.set_style(style);

                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                                Color32::TRANSPARENT;

                            if let Some(close_icon) = &self.close_icon {
                                ui.scope(|ui| {
                                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                                        Color32::RED;
                                    ui.style_mut().visuals.widgets.active.weak_bg_fill =
                                        Color32::DARK_RED;

                                    let close = ui.add(egui::ImageButton::new(close_icon));
                                    if close.clicked() {
                                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                });
                            }

                            if let Some(minimize_icon) = &self.minimize_icon {
                                let minimize = ui.add(egui::ImageButton::new(minimize_icon));
                                if minimize.clicked() {
                                    ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                                }
                            }
                        });
                    });

                    let response =
                        ui.interact(ui.min_rect(), ui.id(), egui::Sense::click_and_drag());
                    if response.dragged() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
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
            }))
        }),
    )
}

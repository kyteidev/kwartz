use egui::{Color32, Context, ViewportCommand};

use crate::Kwartz;

pub fn draw_title_bar(ctx: &Context, kwartz: &Kwartz) {
    egui::TopBottomPanel::top("title_bar").show(ctx, |ui| {
        egui::Frame::none()
            .inner_margin(egui::vec2(-5., 1.))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.scope(|ui| {
                        // remove margin between window control buttons
                        let mut style: egui::Style = (*ctx.style()).clone();
                        style.spacing.item_spacing = egui::vec2(3., 0.);
                        ctx.set_style(style);

                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

                        if let Some(close_icon) = &kwartz.close_icon {
                            ui.scope(|ui| {
                                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                                ui.style_mut().visuals.widgets.active.weak_bg_fill =
                                    Color32::DARK_RED;

                                let close = ui.add(egui::ImageButton::new(close_icon));
                                if close.clicked() {
                                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            });
                        }

                        if let Some(minimize_icon) = &kwartz.minimize_icon {
                            let minimize = ui.add(egui::ImageButton::new(minimize_icon));
                            if minimize.clicked() {
                                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                            }
                        }
                    });
                });

                let response = ui.interact(ui.min_rect(), ui.id(), egui::Sense::click_and_drag());
                if response.dragged() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                }
            });
    });
}

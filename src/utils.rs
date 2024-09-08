use egui::TextureHandle;
use image::load_from_memory;

pub fn load_resized_image(ctx: &egui::Context, bytes: &[u8], size: (u32, u32)) -> TextureHandle {
    let image = load_from_memory(bytes).expect("Failed to load image");

    let resized = image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);

    let rgba_image = resized.into_rgba8();

    let size = [size.0 as usize, size.1 as usize];
    let pixels = rgba_image.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    ctx.load_texture("icon", color_image, egui::TextureOptions::LINEAR)
}

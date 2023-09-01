use std::{borrow::BorrowMut, path::Path};

use egui::Vec2;
use egui_extras::image::RetainedImage;

use crate::{app::AppWindow, plot_window::InterferencePlot};

pub struct FilePicker {
    picked_path: Option<String>,
    image: Option<RetainedImage>,
    show: bool,
    next_window: bool,
}
impl FilePicker {
    pub fn new() -> FilePicker {
        FilePicker {
            picked_path: None,
            image: None,
            show: true,
            next_window: false,
        }
    }
}
impl AppWindow for FilePicker {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        egui::Window::new("Choose an image")
            .open(self.show.borrow_mut())
            .show(ctx, |ui| {
                ui.label("Open image you would like to analyze!");

                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", &["jpg", "png", "jpeg"])
                        .pick_file()
                    {
                        self.picked_path = Some(path.display().to_string());
                        self.image = Some(RetainedImage::from_color_image(
                            self.picked_path.as_ref().unwrap(),
                            load_image_from_path(Path::new(
                                self.picked_path.as_ref().unwrap().as_str(),
                            ))
                            .unwrap(),
                        ));
                    }
                }

                if let Some(picked_path) = &self.picked_path {
                    ui.vertical(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });

                    if self.image.is_some() {
                        self.image
                            .as_ref()
                            .unwrap()
                            .show_max_size(ui, Vec2 { x: 300.0, y: 300.0 });
                        if ui.button("Analyze...").clicked() {
                            self.next_window = true;
                        }
                    }
                }
            });
        if self.next_window {
            self.next_window = false;
            if let Some(path) = self.picked_path.clone() {
                return Some(Box::new(InterferencePlot::new(path)));
            }
        }
        return None;
    }

    fn get_visibility(&self) -> bool {
        self.show
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

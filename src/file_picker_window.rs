use std::{borrow::BorrowMut, path::Path};

use egui::{TextureHandle, Vec2};
use egui_extras::RetainedImage;
use image::RgbaImage;
use nokhwa::{
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, CameraIndex, CameraInfo, RequestedFormat, RequestedFormatType},
    Camera,
};

use crate::{app::AppWindow, plot_window::InterferencePlot};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Channel {
    Red,
    Green,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum StripeOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PlotInitData {
    pub channel: Channel,
    pub path: String,
    pub stripe_orientation: StripeOrientation,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageSource {
    File,
    Camera,
}

pub struct FilePicker {
    picked_path: Option<String>,
    image: Option<TextureHandle>,
    image_rgba: Option<RgbaImage>,
    retained_image: Option<RetainedImage>,
    show: bool,
    next_window: bool,
    image_source: ImageSource,
    available_cameras: Vec<CameraInfo>,
    selected_camera: usize,
    camera: Option<Camera>,
    camera_channel: Channel,
    camera_stripe_orientation: StripeOrientation,
}
impl FilePicker {
    pub fn new() -> FilePicker {
        let mut available_cameras = get_available_cameras();
        available_cameras.sort_by_key(|camera| camera.index().as_index().unwrap());
        let camera = if let Some(info) = available_cameras.first() {
            let index = CameraIndex::Index(
                info.index()
                    .as_index()
                    .expect("Camera index is not numeric"),
            );

            let requested =
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

            let mut camera = Camera::new(index, requested).ok();

            if let Some(cam) = camera.as_mut() {
                let _ = cam.open_stream();
            }

            camera
        } else {
            None
        };
        FilePicker {
            picked_path: None,
            image: None,
            image_rgba: None,
            retained_image: None,
            show: true,
            next_window: false,
            image_source: ImageSource::File,
            available_cameras,
            selected_camera: 0,
            camera,
            camera_channel: Channel::Red,
            camera_stripe_orientation: StripeOrientation::Horizontal,
        }
    }

    fn open_selected_camera(&mut self) {
        self.camera = None;

        let Some(info) = self.available_cameras.get(self.selected_camera) else {
            return;
        };

        let Ok(index) = info.index().as_index() else {
            return;
        };

        let requested =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

        match Camera::new(CameraIndex::Index(index), requested) {
            Ok(mut camera) => match camera.open_stream() {
                Ok(()) => {
                    self.camera = Some(camera);
                }
                Err(e) => {
                    eprintln!("Failed to open camera: {e}");
                }
            },
            Err(e) => {
                eprintln!("Failed to create camera: {e}");
            }
        }
    }
}
impl AppWindow for FilePicker {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        let mut camera_changed = false;
        egui::Window::new("Choose an image")
            .open(self.show.borrow_mut())
            .show(ctx, |ui| {
                ui.label("Open image you would like to analyze!");

                ui.horizontal(|ui| {
                    ui.label("Image source: ");
                    ui.radio_value(&mut self.image_source, ImageSource::File, "File");
                    ui.radio_value(&mut self.image_source, ImageSource::Camera, "Camera");
                });

                if self.image_source == ImageSource::Camera {
                    let mut selected_camera = self.selected_camera;

                    egui::ComboBox::from_label("Camera")
                        .selected_text(
                            self.available_cameras
                                .get(self.selected_camera)
                                .map(|c| c.human_name())
                                .unwrap_or("No camera".to_string()),
                        )
                        .show_ui(ui, |ui| {
                            for (index, camera) in self.available_cameras.iter().enumerate() {
                                ui.selectable_value(
                                    &mut selected_camera,
                                    index,
                                    camera.human_name(),
                                );
                            }
                        });

                    if selected_camera != self.selected_camera {
                        self.selected_camera = selected_camera;
                        camera_changed = true;
                    }

                    ui.horizontal(|ui| {
                        ui.label("Color channel: ");
                        ui.radio_value(&mut self.camera_channel, Channel::Red, "Red");
                        ui.radio_value(&mut self.camera_channel, Channel::Green, "Green");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Stripe orientation: ");
                        ui.radio_value(
                            &mut self.camera_stripe_orientation,
                            StripeOrientation::Horizontal,
                            "Horizontal",
                        );
                        ui.radio_value(
                            &mut self.camera_stripe_orientation,
                            StripeOrientation::Vertical,
                            "Vertical",
                        );
                    });
                }

                if self.image_source == ImageSource::File && ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", &["jpg", "png", "jpeg"])
                        .pick_file()
                    {
                        self.picked_path = Some(path.display().to_string());
                        self.image = Some(
                            ctx.load_texture(
                                self.picked_path.as_ref().unwrap(),
                                load_image_from_path(Path::new(
                                    self.picked_path.as_ref().unwrap().as_str(),
                                ))
                                .unwrap(),
                                egui::TextureOptions::LINEAR,
                            ),
                        );
                        self.retained_image = Some(RetainedImage::from_color_image(
                            self.picked_path.as_ref().unwrap(),
                            load_image_from_path(Path::new(
                                self.picked_path.as_ref().unwrap().as_str(),
                            ))
                            .unwrap(),
                        ));
                    }
                }

                if self.image_source == ImageSource::File {
                    if let Some(picked_path) = &self.picked_path {
                        ui.vertical(|ui| {
                            ui.label("Picked file:");
                            ui.monospace(picked_path);
                        });

                        if self.retained_image.is_some() {
                            self.retained_image
                                .as_ref()
                                .unwrap()
                                .show_max_size(ui, Vec2 { x: 300.0, y: 300.0 });
                            if ui.button("Analyze...").clicked() {
                                self.next_window = true;
                            }
                        }
                    }
                } else if self.image_source == ImageSource::Camera {
                    if let Some(camera) = self.camera.as_mut() {
                        let cam_image_data = load_webcam_image(camera).unwrap();
                        let texture = ctx.load_texture(
                            "webcam",
                            cam_image_data.0,
                            egui::TextureOptions::LINEAR,
                        );
                        let aspect_ratio = cam_image_data.2 as f32 / cam_image_data.3 as f32;
                        let display_width = if cam_image_data.2 as f32 > 300.0 {
                            300.0
                        } else {
                            cam_image_data.2 as f32
                        };
                        let display_height = display_width / aspect_ratio;
                        let display_height = if display_height > 300.0 {
                            300.0
                        } else {
                            display_height
                        };
                        ui.add(egui::Image::new(
                            &texture,
                            Vec2::new(display_width, display_height),
                        ));

                        self.image = Some(texture);
                        self.image_rgba = Some(cam_image_data.1);

                        ctx.request_repaint_after(std::time::Duration::from_millis(50));
                        self.next_window = true;
                    }
                } else {
                    ui.label("No camera available");
                }
            });
        if camera_changed {
            self.open_selected_camera();
        }
        if self.next_window {
            self.next_window = false;
            if self.image_source == ImageSource::File {
                if let Some(path) = self.picked_path.clone() {
                    return Some(Box::new(InterferencePlot::new(
                        ImageSource::File,
                        None,
                        PlotInitData {
                            channel: Channel::Red,
                            path,
                            stripe_orientation: StripeOrientation::Horizontal,
                        },
                    )));
                }
            } else if self.image_source == ImageSource::Camera {
                return Some(Box::new(InterferencePlot::new(
                    ImageSource::Camera,
                    self.image_rgba.clone(),
                    PlotInitData {
                        channel: self.camera_channel,
                        path: String::from("Camera"),
                        stripe_orientation: self.camera_stripe_orientation,
                    },
                )));
            }
        }
        return None;
    }

    fn get_visibility(&self) -> bool {
        self.show
    }

    fn get_name(&self) -> &str {
        "Choose an image"
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn load_webcam_image(
    camera: &mut Camera,
) -> anyhow::Result<(egui::ColorImage, RgbaImage, usize, usize)> {
    let frame = camera.frame()?;
    let decoded = frame.decode_image::<RgbAFormat>()?;
    let width = decoded.width() as usize;
    let height = decoded.height() as usize;
    Ok((
        egui::ColorImage::from_rgba_unmultiplied(
            [width, height],
            decoded.as_flat_samples().as_slice(),
        ),
        decoded,
        width,
        height,
    ))
}

fn get_available_cameras() -> Vec<CameraInfo> {
    match query(ApiBackend::Auto) {
        Ok(cameras) => cameras,
        Err(_) => vec![],
    }
}

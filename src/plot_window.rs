use egui_notify::Toasts;
use std::borrow::BorrowMut;
use std::time::Duration;

use egui::{Color32, RichText};
use raster::Image;

use crate::app::AppWindow;
use egui::plot::{Line, Plot, PlotPoints};
use image::{Rgb, RgbaImage};

#[derive(PartialEq)]
enum Channel {
    Red,
    Green,
}

#[derive(PartialEq)]
enum Mode {
    Individual,
    Summarized,
}

pub struct InterferencePlot {
    path: String,
    image: RgbaImage,
    show: bool,
    channel: Channel,
    mode: Mode,
    line_index: u32,
    toasts: Toasts,
    to_summarize: Vec<u32>,
}

impl InterferencePlot {

    fn load_image_from_path(path: String) -> Result<RgbaImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        Ok(image.to_rgba8())
    }

    pub fn new(path: String) -> InterferencePlot {
        InterferencePlot {
            path: path.clone(),
            image: Self::load_image_from_path(path.clone()).unwrap(),
            show: true,
            channel: Channel::Red,
            mode: Mode::Individual,
            line_index: 0,
            toasts: Toasts::default(),
            to_summarize: vec![],
        }
    }
}

impl AppWindow for InterferencePlot {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        egui::Window::new(format!("Analyze ({})", self.path))
            .min_width(1000.0)
            .open(self.show.borrow_mut())
            .show(ctx, |ui| {
                self.toasts.show(ctx);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        //-------------------------------Radio buttons---------------------------------------
                        ui.horizontal(|ui| {
                            ui.label("Mode: ");
                            if self.to_summarize.len() > 0 {
                                ui.radio_value(&mut self.mode, Mode::Individual, "Individual");
                                ui.radio_value(&mut self.mode, Mode::Summarized, "Summarized");
                            } else {
                                self.mode = Mode::Individual;
                                ui.radio_value(&mut self.mode, Mode::Individual, "Individual");
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Color channel: ");
                            ui.radio_value(&mut self.channel, Channel::Red, "Red");
                            ui.radio_value(&mut self.channel, Channel::Green, "Green");
                        });
                        //----------------------------------------------------------------------------------

                        //-------------------------------Index slider---------------------------------------
                        if self.mode == Mode::Individual {
                            ui.horizontal(|ui| {
                                ui.label("Line index: ");
                                if ui.button("-").clicked() {
                                    if self.line_index > 0 {
                                        self.line_index -= 1;
                                    }
                                }
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.line_index,
                                        0..=(self.image.width() - 1),
                                    )
                                    .step_by(1.0)
                                    .suffix("px"),
                                );
                                if ui.button("+").clicked() {
                                    if self.line_index < self.image.width() - 1 {
                                        self.line_index += 1;
                                    }
                                }

                                if !self.to_summarize.contains(&self.line_index) {
                                    if ui.button("Add to summarized graph").clicked() {
                                        self.to_summarize.push(self.line_index);
                                    }
                                } else {
                                    if ui.button("Remove from summarized graph").clicked() {
                                        if let Some(pos) = self
                                            .to_summarize
                                            .iter()
                                            .position(|x| *x == self.line_index)
                                        {
                                            self.to_summarize.remove(pos);
                                        }
                                    }
                                }
                                if ui.button("Sum all graphs").clicked() {
                                    self.to_summarize = (0..self.image.width()).collect();
                                }
                            });
                        }
                        //----------------------------------------------------------------------------------

                        //-------------------------------Main plot---------------------------------------
                        let points: Vec<[f64; 2]>;
                        if self.mode == Mode::Individual {
                            points = get_points(
                                &self.image,
                                if self.channel == Channel::Red {
                                    "r"
                                } else {
                                    "g"
                                },
                                self.line_index,
                            );
                        } else {
                            points = get_sum_points(
                                &self.image,
                                if self.channel == Channel::Red {
                                    "r"
                                } else {
                                    "g"
                                },
                                &self.to_summarize,
                            );
                        }
                        let points: PlotPoints = PlotPoints::new(points);
                        let line = Line::new(points).color(if self.channel == Channel::Red {
                            Color32::RED
                        } else {
                            Color32::GREEN
                        });
                        Plot::new(format!("plot{}", self.line_index))
                            .view_aspect(2.0)
                            .height(350.0)
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                    //----------------------------------------------------------------------------------

                    //-------------------------------Clipboard---------------------------------------
                    let points: Vec<[f64; 2]>;
                    if self.mode == Mode::Individual {
                        points = get_points(
                            &self.image,
                            if self.channel == Channel::Red {
                                "r"
                            } else {
                                "g"
                            },
                            self.line_index,
                        );
                    } else {
                        points = get_sum_points(
                            &self.image,
                            if self.channel == Channel::Red {
                                "r"
                            } else {
                                "g"
                            },
                            &self.to_summarize,
                        );
                    }

                    let mut str_points: String = String::new();
                    for point in points {
                        str_points += format!("{} {}", point[0], point[1]).as_str();
                        str_points += "\n";
                    }
                    ui.vertical(|ui| {
                        if ui.button("Copy coordinates to clipboard").clicked() {
                            use clipboard::ClipboardContext;
                            use clipboard::ClipboardProvider;
                            let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                            clip_ctx.set_contents(str_points).unwrap();

                            self.toasts
                                .info("Copied to clipboard!")
                                .set_duration(Some(Duration::from_secs(3)));
                        }
                        //----------------------------------------------------------------------------------

                        //-------------------------------Graphs to summarize---------------------------------------
                        if !self.to_summarize.is_empty() {
                            ui.add_space(30.0);
                            ui.label(RichText::new("Graphs to summarize:").strong());
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut points_to_remove: Vec<u32> = vec![];
                                for pixel in self.to_summarize.iter() {
                                    ui.horizontal(|ui| {
                                        let points = get_points(
                                            &self.image,
                                            if self.channel == Channel::Red {
                                                "r"
                                            } else {
                                                "g"
                                            },
                                            pixel.to_owned(),
                                        );
                                        let points: PlotPoints = PlotPoints::new(points);
                                        let line = Line::new(points).color(
                                            if self.channel == Channel::Red {
                                                Color32::RED
                                            } else {
                                                Color32::GREEN
                                            },
                                        );
                                        Plot::new(format!("plot{}", pixel))
                                            .view_aspect(2.0)
                                            .height(40.0)
                                            .show(ui, |plot_ui| plot_ui.line(line));

                                        if ui.button("Show").clicked() {
                                            self.line_index = pixel.to_owned();
                                        }
                                        if ui.button(format!("Remove {}px", pixel)).clicked() {
                                            points_to_remove.push(pixel.to_owned());
                                        }
                                    });
                                }
                                for pixel in points_to_remove {
                                    if let Some(pos) =
                                        self.to_summarize.iter().position(|x| *x == pixel)
                                    {
                                        self.to_summarize.remove(pos);
                                    }
                                }
                            });
                            //----------------------------------------------------------------------------------
                        }
                    });
                });
            });
        None
    }

    fn get_visibility(&self) -> bool {
        self.show
    }
}

fn get_sum_points(image: &RgbaImage, color_channel: &str, x_coords: &Vec<u32>) -> Vec<[f64; 2]> {
    let mut sum_points: Vec<[f64; 2]> = vec![];
    for x in x_coords {
        let points = get_points(&image, color_channel, x.to_owned());
        if sum_points.len() > 0 {
            for (j, point) in points.iter().enumerate() {
                sum_points[j][0] += point[0];
                sum_points[j][1] += point[1];
            }
        } else {
            for point in points {
                sum_points.push(point);
            }
        }
    }

    let mut max_y = 0.0;
    let mut max_x = 0.0;
    for i in 0..sum_points.len() {
        sum_points[i][0] /= x_coords.len() as f64;
        sum_points[i][1] /= x_coords.len() as f64;

        if sum_points[i][1] > max_y {
            max_y = sum_points[i][1];
            max_x = sum_points[i][0];
        }
    }

    for i in 0..sum_points.iter().len() {
        sum_points[i][0] -= max_x;
    }

    sum_points
}

fn get_points(image: &RgbaImage, color_channel: &str, x_coord: u32) -> Vec<[f64; 2]> {
    let mut points = vec![];
    for i in 0..image.height() {
        let pixel = image.get_pixel(x_coord, i);
        if color_channel.eq("r") {
            points.push([i as f64, pixel[0] as f64]);
        } else if color_channel.eq("g") {
            points.push([i as f64, pixel[1] as f64]);
        }
    }
    let mut max_y = 0.0;
    let mut max_x = 0.0;
    for point in points.iter() {
        if point[1] > max_y {
            max_y = point[1];
            max_x = point[0];
        }
    }

    for i in 0..points.iter().len() {
        points[i][0] -= max_x;
    }

    points
}

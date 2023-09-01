use egui::RichText;

use crate::help_window::Help;

pub trait AppWindow {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>>;

    fn get_visibility(&self) -> bool;
}

#[derive(Default)]
pub struct App {
    pub windows: Vec<Box<dyn AppWindow>>,
}

impl App {
    pub fn new(windows: Vec<Box<dyn AppWindow>>) -> App {
        App { windows: windows }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.button(RichText::new("Help").size(18.0)).clicked() {
                    self.windows.push(Box::new(Help::new()));
                }
            });
        });

        let mut new_windows: Vec<Box<dyn AppWindow>> = vec![];
        for i in 0..self.windows.len() {
            if self.windows[i].get_visibility() {
                let window = self.windows[i].show(ctx, frame);
                if window.is_some() {
                    new_windows.push(window.unwrap());
                }
            }
        }
        self.windows.append(new_windows.as_mut());
    }
}

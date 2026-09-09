#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use eframe::egui;

use crate::plot_window::GraphsForSummary;
pub mod app;
pub mod file_picker_window;
pub mod help_window;
pub mod plot_window;
pub mod summarized_window;
pub mod summary_plot_window;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        maximized: true,
        initial_window_size: Some(egui::vec2(320.0, 350.0)),
        icon_data: Some(load_icon("./res/icon.png")),
        ..Default::default()
    };
    let graphs: GraphsForSummary = Arc::new(Mutex::new(HashMap::new()));
    let graphs_clone = graphs.clone();
    eframe::run_native(
        "Interference graph analyzer",
        options,
        Box::new(|_cc| {
            Box::<app::App>::new(app::App::new(vec![
                Box::new(file_picker_window::FilePicker::new(graphs)),
                Box::new(summarized_window::SummarizedWindow::new(graphs_clone)),
            ]))
        }),
    )
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

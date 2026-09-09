use egui::Color32;
use egui_notify::Toasts;
use std::collections::HashMap;
use std::f64;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::app::AppWindow;
use egui::plot::{Line, Plot, PlotPoints};

pub type GraphsForSummary = Arc<Mutex<HashMap<String, Vec<[f64; 2]>>>>;

pub struct SummaryPlot {
    name: String,
    show: bool,
    toasts: Toasts,
    graphs_for_summary: GraphsForSummary,
}

impl SummaryPlot {
    pub fn new(graphs_for_summary: GraphsForSummary) -> SummaryPlot {
        SummaryPlot {
            name: String::from("Summary plot"),
            show: true,
            toasts: Toasts::default(),
            graphs_for_summary,
        }
    }
}

impl AppWindow for SummaryPlot {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        egui::Window::new(self.name.clone())
            .min_width(1000.0)
            .open(&mut self.show)
            .show(ctx, |ui| {
                self.toasts.show(ctx);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        //-------------------------------Main plot---------------------------------------
                        let graphs = self.graphs_for_summary.clone();
                        let graphs_lock = graphs.lock().unwrap();
                        let points = get_averaged_graph(&graphs_lock);
                        let points: PlotPoints = PlotPoints::new(points);
                        let line = Line::new(points).width(1.0f32).color(Color32::RED);
                        Plot::new("summary_plot")
                            .view_aspect(2.0)
                            .height(350.0)
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                    //----------------------------------------------------------------------------------

                    //-------------------------------Clipboard---------------------------------------
                    let graphs = self.graphs_for_summary.clone();
                    let graphs_lock = graphs.lock().unwrap();
                    let points = get_averaged_graph(&graphs_lock);

                    let mut str_points: String = String::new();
                    for point in points.iter() {
                        str_points += format!("{} {}", point[0], point[1]).as_str();
                        str_points += "\n";
                    }
                    ui.vertical(|ui| {
                        if ui
                            .add_sized(
                                [180.0, 30.0],
                                egui::Button::new("Copy coordinates to clipboard"),
                            )
                            .clicked()
                        {
                            use clipboard::ClipboardContext;
                            use clipboard::ClipboardProvider;
                            let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                            clip_ctx.set_contents(str_points).unwrap();

                            self.toasts
                                .info("Copied to clipboard!")
                                .set_duration(Some(Duration::from_secs(3)));
                        }
                        //----------------------------------------------------------------------------------
                    });
                });
            });
        None
    }

    fn get_visibility(&self) -> bool {
        self.show
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn get_averaged_graph(graphs: &HashMap<String, Vec<[f64; 2]>>) -> Vec<[f64; 2]> {
    let mut filtered_graphs: Vec<Vec<[f64; 2]>> = vec![];
    let mut points: Vec<[f64; 2]> = vec![];
    let mut x_min = f64::MIN;
    let mut x_max = f64::MAX;
    for (_, value) in graphs.iter() {
        if value.first().unwrap()[0] > x_min {
            x_min = value.first().unwrap()[0];
        }

        if value.last().unwrap()[0] < x_max {
            x_max = value.last().unwrap()[0];
        }
    }

    for (_, value) in graphs.iter() {
        filtered_graphs.push(
            value
                .iter()
                .filter(|v| v[0] >= x_min && v[0] <= x_max)
                .copied()
                .collect(),
        );
    }

    for graph in filtered_graphs.iter() {
        if points.is_empty() {
            points.extend(graph);
        } else {
            points = points
                .iter()
                .zip(graph)
                .map(|(a, b)| [a[0], a[1] + b[1]])
                .collect();
        }
    }

    points = points
        .iter()
        .map(|p| [p[0], p[1] / filtered_graphs.len() as f64])
        .collect();
    points
}

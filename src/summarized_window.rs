use crate::{app::AppWindow, plot_window::GraphsForSummary, summary_plot_window::SummaryPlot};

pub struct SummarizedWindow {
    show: bool,
    graphs_for_summary: GraphsForSummary,
    show_summary: bool,
}

impl SummarizedWindow {
    pub fn new(graphs_for_summary: GraphsForSummary) -> SummarizedWindow {
        SummarizedWindow {
            show: true,
            graphs_for_summary,
            show_summary: false,
        }
    }
}
impl AppWindow for SummarizedWindow {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        egui::Window::new("Summarized Overview").show(ctx, |ui| {
            ui.label("Images to summarize:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut graphs_to_remove: Vec<String> = vec![];
                let graphs = self.graphs_for_summary.clone();
                let mut graphs_lock = graphs.lock().unwrap();
                for (name, _) in graphs_lock.iter() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        if ui.button("Remove").clicked() {
                            graphs_to_remove.push(name.clone());
                        }
                    });
                }
                for graph in &graphs_to_remove {
                    graphs_lock.remove(graph);
                }
            });
            if ui.button("Show graph").clicked() {
                self.show_summary = true;
            }
        });

        if self.show_summary {
            self.show_summary = false;
            let graphs = self.graphs_for_summary.clone();
            return Some(Box::new(SummaryPlot::new(graphs)));
        }
        return None;
    }

    fn get_visibility(&self) -> bool {
        self.show
    }

    fn get_name(&self) -> &str {
        "Summarized Overview"
    }
}

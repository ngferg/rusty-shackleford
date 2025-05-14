use eframe::egui;
use std::sync::Arc;
use todo_lib::{Dao, QueryTodo};

struct MyApp {
    dao: Arc<Dao>,
    new_task_description: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        let dao = Arc::new(Dao::new(format!("{home_dir}/.ftodo/").as_str()));
        Self {
            dao,
            new_task_description: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Todo app!");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_max_width(450.0);
                    ui.label("Todo:");
                    ui.separator();

                    self.dao
                        .get_tasks(QueryTodo {
                            incomplete_tasks_only: true,
                        })
                        .iter()
                        .for_each(|task| {
                            ui.horizontal(|ui| {
                                ui.label(task.description.clone());
                                ui.button("🗑").on_hover_text("Delete").clicked().then(|| {
                                    self.dao.delete_task(task.id);
                                });
                                ui.button("✔").on_hover_text("Finish").clicked().then(|| {
                                    self.dao.finish_task(task.id);
                                });
                            });
                        });
                    ui.separator();
                    ui.label("New Task:");
                    let response = ui.text_edit_singleline(&mut self.new_task_description);
                    if response.lost_focus()
                        && response.ctx.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        if !self.new_task_description.is_empty() {
                            self.dao.add_task(&self.new_task_description);
                            self.new_task_description.clear();
                        }
                    }
                });
                ui.vertical(|ui| {
                    ui.set_max_width(450.0);
                    ui.label("Done:");
                    ui.separator();
                    self.dao
                        .get_tasks(QueryTodo {
                            incomplete_tasks_only: false,
                        })
                        .iter()
                        .for_each(|task| {
                            ui.horizontal(|ui| {
                                ui.label(task.description.clone());
                                ui.button("🗑").on_hover_text("Delete").clicked().then(|| {
                                    self.dao.delete_task(task.id);
                                });
                                ui.button("🔙").on_hover_text("Unfinish").clicked().then(|| {
                                    self.dao.unfinish_task(task.id);
                                });
                            });
                        });
                });
            });
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Todo App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

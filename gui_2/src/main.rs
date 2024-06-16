use eframe::{egui, epi};

#[derive(Default)]
struct MyApp {
    name: String,
    email: String,
    age: String,
    submitted: bool,
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Form App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("User Information Form");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("Email:");
                ui.text_edit_singleline(&mut self.email);
            });

            ui.horizontal(|ui| {
                ui.label("Age:");
                ui.text_edit_singleline(&mut self.age);
            });

            if ui.button("Submit").clicked() {
                self.submitted = true;
            }

            if ui.button("Reset").clicked() {
                self.name.clear();
                self.email.clear();
                self.age.clear();
                self.submitted = false;
            }

            if self.submitted {
                ui.separator();
                ui.label(format!("Submitted Name: {}", self.name));
                ui.label(format!("Submitted Email: {}", self.email));
                ui.label(format!("Submitted Age: {}", self.age));
            }
        });

        ctx.request_repaint(); // Ensure the UI keeps updating
    }
}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}


use eframe::{egui, epi};
use reqwest::Error;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Deserialize, Debug, Clone)]
struct MockData {
    id: u32,
    name: String,
    email: String,
}

async fn fetch_mock_data() -> Result<Vec<MockData>, Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/users")
        .await?
        .json::<Vec<MockData>>()
        .await?;
    Ok(response)
}

struct MyApp {
    data: Arc<Mutex<Vec<MockData>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "API Data Fetcher"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("User Data");

            let data = self.data.lock().unwrap();
            if data.is_empty() {
                ui.label("Fetching data...");
            } else {
                for user in data.iter() {
                    ui.group(|ui| {
                        ui.label(format!("ID: {}", user.id));
                        ui.label(format!("Name: {}", user.name));
                        ui.label(format!("Email: {}", user.email));
                    });
                }
            }
        });

        ctx.request_repaint(); // Ensure the UI keeps updating
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let data_clone = Arc::clone(&self.data);
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let fetched_data = rt.block_on(fetch_mock_data()).unwrap();
            let mut data = data_clone.lock().unwrap();
            *data = fetched_data;
        });
    }
}

#[tokio::main]
async fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}


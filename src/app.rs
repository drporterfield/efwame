use std::{
    sync::{Arc, Mutex},
};
#[derive(serde::Deserialize, serde::Serialize, Clone)]
enum Download {
    None,
    InProgress,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BurnApp {
    url: String,
    streaming: bool,
    download: Arc<Mutex<String>>,
}

impl Default for BurnApp {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_owned(),
            streaming: false,
            download: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl BurnApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for BurnApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Enter a URL: ");
                ui.text_edit_singleline(&mut self.url);
            });

            if ui.button("Fetch Data").clicked() {
                let url = self.url.clone();
                let download_lock = self.download.clone();
                tokio::spawn(async move {
                    match get_http(&url).await {
                        Ok(body) => {*download_lock.lock().unwrap() = body;}, 
                        Err(e) => eprintln!("HTTP error: {}", e),
                    }
                });
            }

            ui.label(self.download.lock().unwrap().clone());

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                //powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

        });

    }
}

pub async fn get_http(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Send a simple GET request to the target URL
    let body = reqwest::get(url)
        .await? // wait for the HTTP response
        .text() // read response body as text
        .await?; // wait for the full body to be collected

    Ok(body)
}


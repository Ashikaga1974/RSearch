mod file_processing;

use file_processing::{search_in_file, write_exe_files_to_file, should_update_file};
use std::sync::{Arc, Mutex};
use std::process::Command;
use eframe::egui::ViewportBuilder;

/// The main entry point of the application.
///
/// This function initializes the application by checking if the file should be updated,
/// writing executable files to a file if necessary, and setting up the GUI options.
/// It then runs the native eframe application.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the application runs successfully,
///   or an error if something goes wrong during initialization or execution.
fn main() -> Result<(), Box<dyn std::error::Error>> {

    if should_update_file()? {
        write_exe_files_to_file()?;
    }

    let viewport_builder = ViewportBuilder::default()
        .with_decorations(false)
        .with_transparent(true)
        .with_taskbar(false);

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };
    eframe::run_native(
        "Suche nach installierten Programmen",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))), // Verwende `Ok`
    )?;
    Ok(())
}

struct MyApp {
    search_term: String,
    search_results: Arc<Mutex<Vec<(String, String)>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search_term: String::new(),
            search_results: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Suche nach installierten Programmen");

            ui.horizontal(|ui| {
                ui.label("Suchbegriff:");
                ui.text_edit_singleline(&mut self.search_term);
                if ui.button("Suchen").clicked() {
                    let results = search_in_file("installierte_programme.txt", &self.search_term)
                        .unwrap_or_else(|_| Vec::new());
                    let mut search_results = self.search_results.lock().unwrap();
                    *search_results = results;
                };
                if ui.button("Close").clicked() {
                    std::process::exit(0);
                }
            });

            ui.separator();

            ui.heading("Suchergebnisse:");
            let search_results = self.search_results.lock().unwrap();
            for (name, path) in &*search_results {
                if ui.button(name).clicked() {
                    println!("Starte Programm: {} von Pfad: {}", name, path);
                    if let Err(e) = Command::new(path).spawn() {
                        eprintln!("Fehler beim Starten des Programms {}: {}", name, e);
                    }
                }
            }
        });
    }
}

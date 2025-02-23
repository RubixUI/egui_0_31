#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

// This example shows a top-level menu with a nested submenu,
// both created via ui.menu_custom_widget.

use eframe::egui::{self, CentralPanel};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Nested Menu Example",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Create a top-level custom widget menu button:
            ui.menu_custom_widget(
                // The custom widget trigger: a button labeled "Main Menu"
                |ui:&mut egui::Ui| ui.button("Main Menu"),
                // Define the contents of the main menu:
                |ui| {
                    ui.label("Main Menu Content:");

                    // Create a nested submenu within the main menu:
                    ui.menu_custom_widget(
                        // Trigger for the nested submenu:
                        |ui:&mut egui::Ui| ui.button("Nested Submenu"),
                        // Contents of the nested submenu:
                        |ui| {
                            ui.label("This is the nested submenu content.");
                            if ui.button("Nested Action").clicked() {
                                println!("Nested Action Clicked!");
                            }
                        }
                    );

                    ui.separator();
                    if ui.button("Main Action").clicked() {
                        println!("Main Action Clicked!");
                    }
                }
            );
        });
    }
}
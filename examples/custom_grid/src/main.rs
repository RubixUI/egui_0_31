#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

// This example shows a top-level menu with a nested submenu,
// both created via ui.menu_custom_widget.

use eframe::egui::{self, CentralPanel};
use eframe::egui::accesskit::Role::Grid;
use eframe::epaint::Stroke;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Fluid Grid Example",
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
            let words = [
                "random", "words", "in", "a", "random", "order", "that", "just", "keeps", "going",
                "with", "some", "more",
            ];
            egui::Grid::collapsed("my_grid")
                .max_col_width(100)
                .min_col_width(50)
                .show(ui, |ui| {
                    for row in 0..3 {
                        egui::Grid::row(ui,|ui|{
                            for col in 0..3 {
                                if col == 0 {
                                    ui.label(format!("row {row}"));
                                } else {
                                    let word_idx = row * 3 + col * 5;
                                    let word_count = (row * 5 + col * 75) % 13;
                                    let mut string = String::new();
                                    for word in words.iter().cycle().skip(word_idx).take(word_count) {
                                        string += word;
                                        string += " ";
                                    }
                                    ui.label(string);
                                }
                            }
                        });
                    }
                });

            // egui::Grid::fluid("my_grid_fluid")
            //     .percent(50)
            //     .fixed(300)
            //     .remainder()
            //     .show(ui, |ui| {
            //         for row in 0..3 {
            //             egui::Grid::row(ui,|ui|{
            //                 for col in 0..3 {
            //                     if col == 0 {
            //                         ui.label(format!("row {row}"));
            //                     } else {
            //                         let word_idx = row * 3 + col * 5;
            //                         let word_count = (row * 5 + col * 75) % 13;
            //                         let mut string = String::new();
            //                         for word in words.iter().cycle().skip(word_idx).take(word_count) {
            //                             string += word;
            //                             string += " ";
            //                         }
            //                         ui.label(string);
            //                     }
            //                 }
            //             });
            //         }
            //     });
            //
            // egui::Grid::fluid("my_grid_index")
            //     .percent(30)
            //     .percent(30)
            //     .remainder()
            //     .show(ui, |ui| {
            //             for row in 0..3 {
            //                     egui::Grid::row(ui,|ui|{
            //                         let grid_row = ui.get_grid();
            //                         if let Some(grid) = grid_row {
            //                             if grid.is_first_row() {
            //                                 let row_rect = grid.get_row_rect(ui);
            //                                 if let Some(rect) = row_rect {
            //                                     ui.painter().line(vec![rect.left_bottom(),rect.right_bottom()],Stroke::new(1.0,egui::Color32::WHITE));
            //                                 }
            //                             }
            //                         }
            //                         for col in 0..3 {
            //                             let grid = ui.get_grid();
            //                             if let Some(grid) = grid {
            //                                 let mut string = String::new();
            //                                 string += if grid.is_first_column() {
            //                                     "\nfirst col"
            //                                 } else { "" };
            //                                 string += if grid.is_first_row() {
            //                                     "\nfirst row"
            //                                 } else { "" };
            //                                 string += if grid.is_last_column() {
            //                                     "\nlast col"
            //                                 } else { "" };
            //                                 string += if grid.is_last_row() {
            //                                     "\nlast row"
            //                                 } else { "" };
            //                                 ui.label(string);
            //                             }
            //                         }
            //                     });
            //             }
            //     });
        });
    }
}

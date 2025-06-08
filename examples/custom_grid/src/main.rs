#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

// This example shows a top-level menu with a nested submenu,
// both created via ui.menu_custom_widget.

use eframe::egui::{self, vec2, CentralPanel, GridLayout, GridState, Id, InnerResponse, Ui, UiBuilder, Vec2};
use eframe::egui::{Grid};
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

struct TestTableBuilder {
    id_salt: Id,
    cells: usize,
    striped: bool,
}

struct TestTable {
    striped: bool,
}

impl TestTable {
    pub fn new(striped: bool) -> Self {
        TestTable { striped }
    }

    pub fn row<R>(&self,ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let res = add_contents(ui);
        ui.end_row();
        res
    }

    pub fn col() {}
}

impl TestTableBuilder {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id_salt: Id::new(id),
            cells: 2,
            striped: false,
        }
    }
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
    pub fn cells(mut self,cells: usize) -> Self {
        self.cells = cells;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui,TestTable) -> R) -> InnerResponse<R> {
        let Self {
            id_salt, cells, striped
        } = self;
        let min_size = ui.spacing().interact_size;
        let spacing = Vec2::ZERO;

        let id = ui.make_persistent_id(id_salt);
        let prev_state = GridState::load(ui.ctx(), id);

        // Each grid cell is aligned LEFT_CENTER.
        // If somebody wants to wrap more things inside a cell,
        // then we should pick a default layout that matches that alignment,
        // which we do here:
        let max_rect = ui.cursor().intersect(ui.max_rect());

        let mut ui_builder = UiBuilder::new().max_rect(max_rect);
        if prev_state.is_none() {
            // The initial frame will be glitchy, because we don't know the sizes of things to come.

            if ui.is_visible() {
                // Try to cover up the glitchy initial frame:
                ui.ctx().request_discard("new Grid");
            }

            // Hide the ui this frame, and make things as narrow as possible:
            ui_builder = ui_builder.sizing_pass().invisible();
        }

        ui.allocate_new_ui(ui_builder, |ui| {
            ui.horizontal(|ui| {
                let min_size = ui.spacing().interact_size;
                let max_width = ui.available_width();
                let col_width = max_width / cells as f32;
                let vector: Vec<f32> = vec![col_width; cells];
                let layout_size = GridLayout::fluid_size(min_size,vector);
                let grid = GridLayout::new(ui, id, prev_state,layout_size,spacing,0);

                ui.set_grid(grid);
                let r = add_contents(ui,TestTable::new(striped));
                ui.save_grid();
                r
            })
                .inner
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Create a top-level custom widget menu button:
            let words = [
                "random", "words", "in", "a", "random", "order", "that", "just", "keeps", "going",
                "with", "some", "more",
            ];
            TestTableBuilder::new("my_grid")
                .cells(3)
                .striped()
                .show(ui, |ui,table| {
                    for row in 0..3 {
                        table.row(ui,|ui| {
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

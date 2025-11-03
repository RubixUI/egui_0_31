use egui::{Align2, Area, CentralPanel, Context, Frame, Id, Modal, ThemeBuilder, Vec2};
use egui::panel::Side;

struct MyApp {
    modal_open: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let builder = ThemeBuilder::default();
        Self { modal_open: false }
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Fluid Grid Example",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            //test area
            Area::new(Id::new("area"))
                .sizing_pass(false)
                .show(ctx,|ui|{
                    ui.heading("Hello World!");
                    if ui.button("Open Modal").clicked() {
                        self.modal_open = true;
                    }
                });
            //test modal
            if self.modal_open {
                let modal_res = Modal::new(Id::new("modal"))
                    .show(ctx, |ui| {
                        let mut res = ui.button("Quit");
                        if res.clicked() {
                            res.set_close();
                        }
                        res
                    });
                if modal_res.should_close() {
                    self.modal_open = false;
                }
            }
        });
    }
}

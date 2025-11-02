use egui::{Align2, Area, CentralPanel, Context, Frame, Id, ThemeBuilder, Vec2};
use egui::panel::Side;

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        let builder = ThemeBuilder::default();
        Self {}
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
                })
        });
    }
}

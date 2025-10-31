use egui::{Align2, CentralPanel, Context, Frame, ThemeBuilder, Vec2};
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
            egui::SidePanel::new(Side::Left, "left_panel")
                .frame(Frame::NONE)
                .exact_width(300.)
                .show_inside(ui, |ui| {
                    ui.add_absolute_local(Align2::RIGHT_TOP, Vec2::ZERO, egui::Label::new("RIGHT_TOP"),"l1");
                    ui.add_absolute_local(Align2::RIGHT_BOTTOM, Vec2::new(10., 20.), egui::Label::new("RIGHT_BOTTOM"),"l2");
                });
            CentralPanel::default()
                .frame(Frame::NONE)
                .show_inside(ui, |ui| {
                ui.add_absolute_local(Align2::LEFT_TOP, Vec2::new(10., 20.), egui::Label::new("LEFT_TOP"),"l3");
                ui.add_absolute_local(Align2::LEFT_BOTTOM, Vec2::ZERO, egui::Label::new("LEFT_BOTTOM"),"l4");
            });
            ui.add_absolute(Align2::CENTER_CENTER,Vec2::ZERO,egui::Label::new("Center"),"l5");
            ui.add_absolute(Align2::LEFT_TOP,Vec2::ZERO,egui::Label::new("LEFT_TOP"),"l6");
            ui.add_absolute(Align2::LEFT_BOTTOM,Vec2::ZERO,egui::Label::new("LEFT_BOTTOM"),"l7");
            ui.add_absolute(Align2::RIGHT_TOP,Vec2::ZERO,egui::Label::new("RIGHT_TOP"),"l8");
            ui.add_absolute(Align2::RIGHT_BOTTOM,Vec2::new(30.,60.),egui::Label::new("RIGHT_BOTTOM"),"l9");
        });
    }
}
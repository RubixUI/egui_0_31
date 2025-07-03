use egui::{Context, Ui, Frame, WidgetText, Color32, Margin};
use eframe::{egui, App, NativeOptions};
use egui_extras::install_image_loaders;
use cyber_gui_color::{get_theme, Theme};

struct ButtonDemo;

impl App for ButtonDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = get_theme();
            ui.horizontal(|ui| {
                show_color(ui,"Primary",theme.color_theme.primary());
                show_color(ui,"On Primary",theme.color_theme.on_primary());
                show_color(ui,"Primary",theme.color_theme.primary_container());
                show_color(ui,"Primary",theme.color_theme.on_primary_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Primary Fixed",theme.color_theme.primary_fixed());
                show_color(ui,"Primary Fixed Dim",theme.color_theme.primary_fixed_dim());
                show_color(ui,"On Primary Fixed",theme.color_theme.on_primary_fixed());
                show_color(ui,"On Primary Fixed Variant",theme.color_theme.on_primary_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Secondary",theme.color_theme.secondary());
                show_color(ui,"On Secondary",theme.color_theme.on_secondary());
                show_color(ui,"Secondary Container",theme.color_theme.secondary_container());
                show_color(ui,"on Secondary Container",theme.color_theme.on_secondary_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Secondary Fixed",theme.color_theme.secondary_fixed());
                show_color(ui,"Secondary Fixed Dim",theme.color_theme.secondary_fixed_dim());
                show_color(ui,"On Secondary Fixed",theme.color_theme.on_secondary_fixed_());
                show_color(ui,"On Secondary Fixed Variant",theme.color_theme.on_secondary_fixed_variant());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Tertiary",theme.color_theme.tertiary());
                show_color(ui,"On Tertiary",theme.color_theme.on_tertiary());
                show_color(ui,"Tertiary Container",theme.color_theme.tertiary_container());
                show_color(ui,"on Tertiary Container",theme.color_theme.on_tertiary_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Tertiary Fixed",theme.color_theme.tertiary_fixed());
                show_color(ui,"Tertiary Fixed Dim",theme.color_theme.tertiary_fixed_dim());
                show_color(ui,"On Tertiary Fixed",theme.color_theme.on_tertiary_fixed());
                show_color(ui,"On Tertiary Fixed Variant",theme.color_theme.on_tertiary_fixed_variant());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Error",theme.color_theme.error());
                show_color(ui,"On Error",theme.color_theme.on_error());
                show_color(ui,"Error Container",theme.color_theme.error_container());
                show_color(ui,"on Error Container",theme.color_theme.on_error_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Success",theme.color_theme.success());
                show_color(ui,"On Success",theme.color_theme.on_success());
                show_color(ui,"Success Container",theme.color_theme.success_container());
                show_color(ui,"on Success Container",theme.color_theme.on_success_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Warning",theme.color_theme.warning());
                show_color(ui,"On Warning",theme.color_theme.on_warning());
                show_color(ui,"Warning Container",theme.color_theme.warning_container());
                show_color(ui,"on Warning Container",theme.color_theme.on_warning_container());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Surface Dim",theme.color_theme.surface_dim());
                show_color(ui,"Surface",theme.color_theme.surface());
                show_color(ui,"Surface Bright",theme.color_theme.surface_bright());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Surface Container Lowest",theme.color_theme.surface_container_lowest());
                show_color(ui,"Surface Container Low",theme.color_theme.surface_container_low());
                show_color(ui,"Surface Container",theme.color_theme.surface_container());
                show_color(ui,"Surface Container high",theme.color_theme.surface_container_high());
                show_color(ui,"Surface Container highest",theme.color_theme.surface_container_highest());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Surface Tint",theme.color_theme.surface_tint());
                show_color(ui,"Surface Variant",theme.color_theme.surface_variant());
                show_color(ui,"On Surface",theme.color_theme.on_surface());
                show_color(ui,"On Surface Variant",theme.color_theme.on_surface_variant());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Inverse Primary",theme.color_theme.inverse_primary());
                show_color(ui,"Inverse Surface",theme.color_theme.inverse_surface());
                show_color(ui,"Inverse On Surface",theme.color_theme.inverse_on_surface());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Background",theme.color_theme.background());
                show_color(ui,"On Background",theme.color_theme.on_background());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Outline",theme.color_theme.outline());
                show_color(ui,"Outline Variant",theme.color_theme.outline_variant());
            });
            ui.horizontal(|ui| {
                show_color(ui,"Scrim",theme.color_theme.scrim());
                show_color(ui,"Shadow",theme.color_theme.shadow());
            });
        });
    }
}
fn main() {
    let native_options = NativeOptions::default();
    let _ = eframe::run_native(
        "Test Button Demo",
        native_options,
        Box::new(|_cc| {
            install_image_loaders(&_cc.egui_ctx);
            Theme::apply(&_cc.egui_ctx);
            Ok(Box::new(ButtonDemo{}))
        }),
    );
}

fn show_color(ui: &mut Ui, text: impl Into<WidgetText>,color:Color32) {
    println!("====={:?}",color);
    Frame::default().fill(color).inner_margin(Margin::same(10)).corner_radius(10).show(ui, |ui| {
        ui.label(text);
    });
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]


use eframe::egui::{self, BrightnessMode, CentralPanel, Color32, ColorVariantMode, CornerRadiusMode, Frame, Margin, SpaceMode, ThemeBuilder, Ui, WidgetText};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Fluid Grid Example",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    theme_builder: ThemeBuilder,
    source_color: String
}

impl Default for MyApp {
    fn default() -> Self {
        let builder = ThemeBuilder::default();
        Self {
            theme_builder: Default::default(),
            source_color: builder.get_source_color().to_hex(),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_builder(ui);
            self.show_colors(ui);
            self.show_theme(ui);
        });
    }
}

impl MyApp {
    fn show_builder(&mut self,ui: &mut egui::Ui) {
        let mut builder = &mut self.theme_builder;
        ui.horizontal(|ui|{
            ui.label("Source Color:");
            egui::TextEdit::singleline(&mut self.source_color);
        });
        ui.horizontal(|ui|{
            ui.label("Color Variant:");
            egui::ComboBox::from_label("Color Variant")
                .selected_text(builder.get_color_variant_mode().to_string())
                .show_ui(ui, |ui| {
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::TonalSpot, "TonalSpot").clicked() {
                        builder.set_color_variant(ColorVariantMode::TonalSpot);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Rainbow, "Rainbow").clicked() {
                        builder.set_color_variant(ColorVariantMode::Rainbow);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Fidelity, "Fidelity").clicked() {
                        builder.set_color_variant(ColorVariantMode::Fidelity);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::FruitSalad, "FruitSalad").clicked() {
                        builder.set_color_variant(ColorVariantMode::FruitSalad);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Monochrome, "Monochrome").clicked() {
                        builder.set_color_variant(ColorVariantMode::Monochrome);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Content, "Content").clicked() {
                        builder.set_color_variant(ColorVariantMode::Content);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Expressive, "Expressive").clicked() {
                        builder.set_color_variant(ColorVariantMode::Expressive);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Vibrant, "Vibrant").clicked() {
                        builder.set_color_variant(ColorVariantMode::Vibrant);
                    };
                    if ui.selectable_value(builder.get_color_variant_mode_mut(), ColorVariantMode::Neutral, "Neutral").clicked() {
                        builder.set_color_variant(ColorVariantMode::Neutral);
                    };
                });
        });
        ui.horizontal(|ui| {
            ui.label("Brightness:");
            if ui.radio(builder.is_dark_mode(),"Dark").clicked() {
                builder.set_brightness(BrightnessMode::Dark);
            };
            if ui.radio(!builder.is_dark_mode(),"Light").clicked() {
                builder.set_brightness(BrightnessMode::Light);
            };
        });
        ui.horizontal(|ui| {
            ui.label("Corner Radius:");
            if ui.radio(builder.is_full_size_corner_radius(),"Full").clicked(){
                builder.set_corner_radius_mode(CornerRadiusMode::Full);
            };
            if ui.radio(builder.is_large_corner_radius(),"Large").clicked(){
                builder.set_corner_radius_mode(CornerRadiusMode::Large);
            };
            if ui.radio(builder.is_small_corner_radius(),"Small").clicked(){
                builder.set_corner_radius_mode(CornerRadiusMode::Small);
            };
            if ui.radio(builder.is_none_corner_radius(),"None").clicked(){
                builder.set_corner_radius_mode(CornerRadiusMode::None);
            };
        });
        ui.horizontal(|ui| {
            ui.label("Space:");
            if ui.radio(!builder.is_compact(),"Loose").clicked() {
                builder.set_space_mode(SpaceMode::Loose)
            }
            if ui.radio(builder.is_compact(),"Compact").clicked() {
                builder.set_space_mode(SpaceMode::Compact)
            }
        });
        if ui.button("Update Theme").clicked() {
            ui.ctx().set_color_theme(builder.clone().build_theme());
        }
    }

    fn show_colors(&mut self,ui: &mut egui::Ui) {
        let theme = ui.ctx().style_theme(|theme|theme.clone());
        ui.horizontal(|ui| {
            show_color(ui,"Primary",theme.color.primary());
            show_color(ui,"On Primary",theme.color.on_primary());
            show_color(ui,"Primary",theme.color.primary_container());
            show_color(ui,"Primary",theme.color.on_primary_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Primary Fixed",theme.color.primary_fixed());
            show_color(ui,"Primary Fixed Dim",theme.color.primary_fixed_dim());
            show_color(ui,"On Primary Fixed",theme.color.on_primary_fixed());
            show_color(ui,"On Primary Fixed Variant",theme.color.on_primary_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Secondary",theme.color.secondary());
            show_color(ui,"On Secondary",theme.color.on_secondary());
            show_color(ui,"Secondary Container",theme.color.secondary_container());
            show_color(ui,"on Secondary Container",theme.color.on_secondary_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Secondary Fixed",theme.color.secondary_fixed());
            show_color(ui,"Secondary Fixed Dim",theme.color.secondary_fixed_dim());
            show_color(ui,"On Secondary Fixed",theme.color.on_secondary_fixed_());
            show_color(ui,"On Secondary Fixed Variant",theme.color.on_secondary_fixed_variant());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Tertiary",theme.color.tertiary());
            show_color(ui,"On Tertiary",theme.color.on_tertiary());
            show_color(ui,"Tertiary Container",theme.color.tertiary_container());
            show_color(ui,"on Tertiary Container",theme.color.on_tertiary_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Tertiary Fixed",theme.color.tertiary_fixed());
            show_color(ui,"Tertiary Fixed Dim",theme.color.tertiary_fixed_dim());
            show_color(ui,"On Tertiary Fixed",theme.color.on_tertiary_fixed());
            show_color(ui,"On Tertiary Fixed Variant",theme.color.on_tertiary_fixed_variant());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Error",theme.color.error());
            show_color(ui,"On Error",theme.color.on_error());
            show_color(ui,"Error Container",theme.color.error_container());
            show_color(ui,"on Error Container",theme.color.on_error_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Success",theme.color.success());
            show_color(ui,"On Success",theme.color.on_success());
            show_color(ui,"Success Container",theme.color.success_container());
            show_color(ui,"on Success Container",theme.color.on_success_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Warning",theme.color.warning());
            show_color(ui,"On Warning",theme.color.on_warning());
            show_color(ui,"Warning Container",theme.color.warning_container());
            show_color(ui,"on Warning Container",theme.color.on_warning_container());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Surface Dim",theme.color.surface_dim());
            show_color(ui,"Surface",theme.color.surface());
            show_color(ui,"Surface Bright",theme.color.surface_bright());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Surface Container Lowest",theme.color.surface_container_lowest());
            show_color(ui,"Surface Container Low",theme.color.surface_container_low());
            show_color(ui,"Surface Container",theme.color.surface_container());
            show_color(ui,"Surface Container high",theme.color.surface_container_high());
            show_color(ui,"Surface Container highest",theme.color.surface_container_highest());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Surface Tint",theme.color.surface_tint());
            show_color(ui,"Surface Variant",theme.color.surface_variant());
            show_color(ui,"On Surface",theme.color.on_surface());
            show_color(ui,"On Surface Variant",theme.color.on_surface_variant());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Inverse Primary",theme.color.inverse_primary());
            show_color(ui,"Inverse Surface",theme.color.inverse_surface());
            show_color(ui,"Inverse On Surface",theme.color.inverse_on_surface());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Background",theme.color.background());
            show_color(ui,"On Background",theme.color.on_background());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Outline",theme.color.outline());
            show_color(ui,"Outline Variant",theme.color.outline_variant());
        });
        ui.horizontal(|ui| {
            show_color(ui,"Scrim",theme.color.scrim());
            show_color(ui,"Shadow",theme.color.shadow());
        });
    }

    fn show_theme(&mut self,ui: &mut egui::Ui) {
        let theme = ui.ctx().style_theme(|theme|theme.clone());
        let corner_radius = theme.corner_radius;
        ui.label("Corner Radius:");
        ui.label(format!("Huge: {}", corner_radius.huge.ne));
        ui.label(format!("Extra Large: {}", corner_radius.extra_large.ne));
        ui.label(format!("Large: {}", corner_radius.large.ne));
        ui.label(format!("Medium: {}", corner_radius.medium.ne));
        ui.label(format!("Small: {}", corner_radius.small.ne));
        ui.label(format!("Extra Small: {}", corner_radius.extra_small.ne));
        ui.label(format!("Mini: {}", corner_radius.mini.ne));
        ui.add_space(40.);
        let padding = theme.padding;
        ui.label("Padding:");
        ui.label(format!("Huge: {} {}", padding.huge.left,padding.huge.top));
        ui.label(format!("Extra Large: {} {}", padding.extra_large.left,padding.extra_large.top));
        ui.label(format!("Large: {} {}", padding.large.left,padding.large.top));
        ui.label(format!("Medium: {} {}", padding.medium.left,padding.medium.top));
        ui.label(format!("Small: {} {}", padding.small.left,padding.small.top));
        ui.label(format!("Extra Small: {} {}", padding.extra_small.left,padding.extra_small.top));
        ui.label(format!("Mini: {} {}", padding.mini.left,padding.mini.top));
        ui.add_space(40.);
        let gap_h = theme.gap.horizontal;
        ui.label("Gap Horizontal:");
        ui.label(format!("Huge: {}", gap_h.huge));
        ui.label(format!("Extra Large: {}", gap_h.extra_large));
        ui.label(format!("Large: {}", gap_h.large));
        ui.label(format!("Medium: {}", gap_h.medium));
        ui.label(format!("Small: {}", gap_h.small));
        ui.label(format!("Extra Small: {}", gap_h.extra_small));
        ui.label(format!("Mini: {}", gap_h.mini));
        ui.add_space(40.);
        let gap_v = theme.gap.vertical;
        ui.label("Gap Vertical:");
        ui.label(format!("Huge: {}", gap_v.huge));
        ui.label(format!("Extra Large: {}", gap_v.extra_large));
        ui.label(format!("Large: {}", gap_v.large));
        ui.label(format!("Medium: {}", gap_v.medium));
        ui.label(format!("Small: {}", gap_v.small));
        ui.label(format!("Extra Small: {}", gap_v.extra_small));
        ui.label(format!("Mini: {}", gap_v.mini));
    }
}

fn show_color(ui: &mut Ui, text: impl Into<WidgetText>,color:Color32) {
    Frame::default().fill(color).inner_margin(Margin::same(10)).corner_radius(10).show(ui, |ui| {
        ui.label(text);
    });
}


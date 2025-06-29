#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

// This example shows a top-level menu with a nested submenu,
// both created via ui.menu_custom_widget.

use eframe::egui::{self, vec2, Align, CentralPanel, Color32, FontFamily, GridLayout, GridState, Id, InnerResponse, TextBuffer, TextFormat, Ui, UiBuilder, Vec2};
use eframe::egui::Key::S;
use eframe::egui::text::LayoutJob;
use eframe::epaint::{FontId, Stroke};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Layout Job Example",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut job = LayoutJob::default();
            let text = "fdhsjkfdsjvjdkajdksajdskaldjskajdksfjdklsfdksnjkdnvjfdkfndjksnfjdksngjkdnfjrksngjfkdngjkfdngmdnmgfd";
            let text_b = "rksngjfkdngjkfdngmdnmgfd";
            let text_format = TextFormat {
                font_id: FontId::new(16.,FontFamily::Proportional),
                line_height: Some(36.),
                valign: Align::Center,
                ..Default::default()
            };
            let text_format_with_bg = TextFormat {
                font_id: FontId::new(16.,FontFamily::Proportional),
                line_height: Some(36.),
                background: Color32::YELLOW,
                valign: Align::Center,
                underline: Stroke::new(1.,Color32::BLUE),
                strikethrough: Stroke::new(1.,Color32::BLACK),
                ..Default::default()
            };
            let text_format_small = TextFormat {
                font_id: FontId::new(12.,FontFamily::Proportional),
                line_height: Some(16.),
                valign: Align::Center,
                ..Default::default()
            };
            let text_format_small_with_bg = TextFormat {
                font_id: FontId::new(12.,FontFamily::Proportional),
                line_height: Some(16.),
                background: Color32::GREEN,
                valign: Align::Center,
                ..Default::default()
            };
            // job.append(text, 0.0, text_format);
            // job.append(text_b, 0.0, text_format_small_with_bg);
            // job.append(text, 0.0, text_format_small);
            job.append(text_b, 0.0, text_format_with_bg);

            ui.add(egui::Label::new(job).wrap());
        });
    }
}


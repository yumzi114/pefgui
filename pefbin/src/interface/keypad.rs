use eframe::{egui::{Ui, self, InnerResponse, RichText}, epaint::Vec2};

pub fn keypad_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.horizontal_wrapped(|ui|{
        let mut number = String::new();
        ui.max_rect();
        
        
        ui.columns(4, |columns|{
            columns[0].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("7").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    number.push('7');
                }
                if ui.add(egui::Button::new(RichText::new("4").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("1").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
            columns[1].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("8").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("5").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("2").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("0").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
            columns[2].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("9").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("6").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("3").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new(".").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
            columns[3].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new(number.as_str()).color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("SET").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("CLEAR").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
        })
    })
}
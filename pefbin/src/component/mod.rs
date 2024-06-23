use eframe::egui::{self, Color32, Pos2, RichText, Rounding, Sense, Vec2};

use crate::interface::UserUi;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            // "../files/Pilseung_Gothic.ttf"
            "../../files/hankuk_bold.ttf"
        )),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}
pub fn warring_window(center_rect:Pos2,ctx:&egui::Context, user_ui:&mut UserUi){
    let mut is_open=user_ui.warning_pop.clone();
    let newwindow = egui::Window::new("warring");
        newwindow
        .open(&mut is_open)
        .movable(false)
        // .min_height(150.)
        // .min_width(300.)
        .resizable(false)
        .title_bar(false)
        .frame(egui::Frame{fill:egui::Color32::TRANSPARENT,rounding:Rounding{ nw: 100., ne: 100., sw: 100., se: 100. }, ..Default::default()})
        .current_pos(center_rect)
        .show(ctx, |ui|{
            // ctx.set_visuals(egui::Visuals::light());
            ui.vertical_centered(|ui|{
                // let (rect, _response) =ui.allocate_at_least(Vec2::new(450., 80.), Sense::click_and_drag());
                ui.label("");
                ui.horizontal(|ui|{
                    ui.add_space(50.);
                    let mut warning_file = egui::include_image!("../../files/error/yellow/basewarring.png");
                    let (temp_rect, _response) =ui.allocate_at_least(Vec2::new(950., 450.), Sense::click_and_drag());
                    match &user_ui.status_str[0..=4] {
                        "Limit"=>{
                            warning_file=egui::include_image!("../../files/error/yellow/value_error.png");
                        },
                        "Frequ"=>{
                            warning_file=egui::include_image!("../../files/error/yellow/value_error.png");
                        },
                        _=>{
                            user_ui.warning_pop=false;
                        }
                    }
                    egui::Image::new(warning_file)
                        .paint_at(ui, temp_rect);
                    if _response.clicked(){
                        user_ui.warning_pop=false;
                    }
                });
                
                
            });
        });
}
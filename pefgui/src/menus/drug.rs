use std::f32::consts::TAU;

use eframe::{egui::{self,Ui, InnerResponse, RichText, Slider, collapsing_header::CollapsingState, Response, Sense}, epaint::{Color32, Stroke, vec2, Vec2}};
use egui_extras::{StripBuilder, Size, Column, TableBuilder};

pub struct DrugList {
    pub carbonated:f32,
    pub water:f32,
}
impl ::std::default::Default for DrugList {
    fn default() -> Self { 
        Self { 
            carbonated: 0.0, 
            water:0.0,
    } }
}
impl DrugList {
    pub fn drug_table_view(&mut self, ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        ui.vertical_centered(|ui|{
            ui.columns(2, |columns|{
                columns[0].vertical_centered(|ui|{
                    ui.add_space(10.0);
                    ui.label(RichText::new("원 수").strong().size(20.0));
                    ui.add_space(10.0);
                    // let stroke = ui.style().interact(&response).fg_stroke;
                    // ui.painter().circle_filled(response.rect.center(), radius, stroke.color);
                    // ui.label(RichText::new("EXECUTE").strong().size(20.0));
                    
                    // ui.painter()
                    // .circle_filled(ui.clip_rect().center(), radius, Color32::LIGHT_GREEN);
                    circle_ui(ui, Color32::from_rgb(26, 255, 0),Vec2::splat(40.0));
                });
                columns[1].vertical_centered(|ui|{
                    ui.add_space(10.0);
                    ui.label(RichText::new("탄 산").strong().size(20.0));
                    ui.add_space(10.0);
                    circle_ui(ui, Color32::from_rgb(255, 0, 21),Vec2::splat(40.0));
                    
                });
            });
        })
        
    }
}


fn circle_ui (ui:&mut Ui,color:Color32,size:Vec2){
    // let size = Vec2::splat(40.0);
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let rect = response.rect;
    let c = rect.center();
    let r = rect.width() / 2.0 - 1.0;
    // let color = Color32::from_gray(128);
    // let color = Color32::from_rgb(26, 255, 0);
    painter.circle_filled(c, r, color);
}
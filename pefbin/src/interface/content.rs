use eframe::{egui::{Ui, self, InnerResponse, RichText, Sense, TextFormat}, epaint::{Vec2, Color32, text::{LayoutJob, TextWrapping}, FontId}, emath::Align};
use egui_extras::{TableBuilder, Column};
use super::{UserUi,MenuList};
pub fn content_view(ui: &mut Ui,ctx: &egui::Context,uui:&mut UserUi)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.columns(2, |columns|{
            columns[0].vertical_centered(|ui|{
                ui.add_space(100.);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        if ui.label(RichText::new("High Voltage").strong().size(50.0).color(Color32::from_rgb(38, 150, 255))).clicked(){
                            
                            // if uui.keypad.popon && uui.keypad.sellist==Some(MenuList::SetVoltage){
                            //     uui.keypad.popon=false;
                            //     uui.keypad.sellist=None;
                            // }
                            // else if uui.keypad.popon==false||uui.keypad.sellist!=Some(MenuList::SetVoltage){
                            //     uui.keypad.popon=true;
                            //     uui.keypad.sellist=Some(MenuList::SetVoltage);
                            // };
                        };
                        
                        if ui.add(egui::Button::new(RichText::new("입력값").strong().size(50.0))).clicked() {
                            click_voltage(uui,MenuList::SetVoltage);
                        };
                        // if ui.label(RichText::new("입력값").strong().size(50.0)).clicked(){

                        
                        // let faded_color = ui.visuals().window_fill();
                        // let faded_color = |color: Color32| -> Color32 {
                        //     use egui::Rgba;
                        //     let t =  { 0.0 };
                        //     egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
                        // };
                        // ui.painter().rect_filled(
                        //     ui.available_rect_before_wrap(),
                        //     0.2,
                        //     faded_color(Color32::LIGHT_BLUE),
                        // );
                        // ui.label("First row, first column");
                        // ui.heading("My egui Application");
                        // ui.colored_label(Color32::WHITE, "text");
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Frequency").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        if ui.add(egui::Button::new(RichText::new("입력값").strong().size(50.0))).clicked() {
                            click_voltage(uui,MenuList::PulseFreq);
                        };
                    });
                });
                ui.add_space(100.);
                ui.label(RichText::new("Pulse Time").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("ON").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.label(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN));
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("OFF").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        if ui.add(egui::Button::new(RichText::new("입력값").strong().size(50.0))).clicked() {
                            click_voltage(uui,MenuList::PulseTime);
                        };
                    });
                });
            });
            columns[1].vertical_centered(|ui|{
                // let (rect, _response) =ui.allocate_at_least(Vec2::new(150., 150.), Sense::hover());
                //     // let (rect, _response) =ui.allocate_at_least(Vec2::new(150., 150.), Sense::hover());
                // let temp = egui::Image::new(egui::include_image!("../../files/pngegg1.png"))
                //     .rounding(5.0)
                //     // .tint(egui::Color32::LIGHT_BLUE)
                //     .paint_at(ui, rect);
                // let dd =egui::Image::new(egui::include_image!("../../files/pngegg1.png")).rounding(480.0).fit_to_exact_size(Vec2::new(400., 80.));
                
                ui.add_space(70.);
                ui.push_id(1, |ui| {
                    TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .striped(false)
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .header(60.0, |mut header| {
                        header.col(|ui| {
                            ui.add_space(10.0);
                            ui.label(RichText::new("High Voltage").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        });
                        header.col(|ui| {
                            ui.add_space(10.0);
                            ui.label(RichText::new("Pulse").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        });
                    })
                    .body(|mut body| {
                        body.row(140.0, |mut row| {
                            row.col(|ui| {
                                // ui.add_space(50.0);
                                ui.add_space(10.0);
                                let dd =egui::Image::new(egui::include_image!("../../files/pngegg.png")).rounding(360.0);
                                ui.add_sized([120.0, 120.0], egui::ImageButton::new(dd));
                                // ui.label(RichText::new("원 수").strong().size(50.0).color(Color32::LIGHT_GREEN));
                            });
                            row.col(|ui| {
                                ui.add_space(10.0);
                                let bb =egui::Image::new(egui::include_image!("../../files/pngegg2.png")).rounding(360.0);
                                // if ui.add(egui::ImageButton::new(dd)).clicked(){
                                //     ui.label("asdasdasd");
                                // }
                                ui.add_sized([120.0, 120.0], egui::ImageButton::new(bb));
                                // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                            });
                        });
                    })
                });
                
                // let img = egui::include_image!("../../files/pngegg1.png");
                
            });
            
        })
    })
}


// let mut job = LayoutJob::single_section("asdasd".to_owned(), 
//     TextFormat{
//         font_id: FontId::proportional(45.),
//         background:Color32::WHITE,
//         valign: Align::TOP ,
//         color: Color32::from_rgb(38, 150, 255),
//         ..Default::default()
//     });
// job.wrap = TextWrapping {
//     max_rows: 1,
    
//     break_anywhere: true,
//     // overflow_character: Some('…'),
//     ..Default::default()
// ui.label(job);
// };

fn click_voltage(uui:&mut UserUi, selmenu:MenuList){
    if uui.keypad.popon && uui.keypad.sellist==Some(selmenu){
        uui.keypad.popon=false;
        uui.keypad.sellist=None;
    }
    else if uui.keypad.popon==false||uui.keypad.sellist!=Some(selmenu){
        uui.keypad.popon=true;
        uui.keypad.sellist=Some(selmenu);
    };
}
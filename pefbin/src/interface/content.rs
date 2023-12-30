use eframe::{egui::{Ui, self, InnerResponse, RichText, Sense, TextFormat}, epaint::{Vec2, Color32, text::{LayoutJob, TextWrapping}, FontId}, emath::Align};
use egui_extras::{TableBuilder, Column};
use super::{UserUi,MenuList,PulseInfo,VolatageInfo};

pub fn content_view(ui: &mut Ui,ctx: &egui::Context,uui:&mut UserUi, pulse_info:&mut PulseInfo, vol_info:&mut VolatageInfo)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        
        ui.columns(2, |columns|{
            columns[0].vertical_centered(|ui|{
                ui.add_space(100.);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        if ui.label(RichText::new("High Voltage").strong().size(50.0).color(Color32::from_rgb(38, 150, 255))).clicked(){
                        };
                        
                        if ui.add(egui::Button::new(RichText::new("입력값").strong().size(80.0))).clicked() {
                            click_voltage(uui,MenuList::SetVoltage);
                        };
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Frequency").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        if ui.add(egui::Button::new(RichText::new("입력값").strong().size(80.0))).clicked() {
                            click_voltage(uui,MenuList::PulseFreq);
                        };
                    });
                });
                ui.add_space(100.);
                ui.label(RichText::new("Pulse Time").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("ON").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        if pulse_info.power {
                            ui.label(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN));
                        }
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("OFF").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        let open =ui.add(egui::Button::new(RichText::new("입력값").strong().size(80.0)));
                        if open.clicked(){
                            let dd = open.interact_pointer_pos().unwrap();
                            ui.label(dd.x.to_string());
                            click_voltage(uui,MenuList::PulseTime);
                        }
                    });
                });
            });
            columns[1].vertical_centered(|ui|{
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
                            ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        });
                        header.col(|ui| {
                            ui.add_space(10.0);
                            ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        });
                    })
                    .body(|mut body| {
                        body.row(140.0, |mut row| {
                            row.col(|ui| {
                                ui.add_space(10.0);
                                if ui.add_sized([120.0, 120.0], egui::ImageButton::new(check_on(vol_info.power))).clicked(){
                                    vol_info.power=!vol_info.power
                                };
                            });
                            row.col(|ui| {
                                ui.add_space(10.0);
                                if ui.add_sized([120.0, 120.0], egui::ImageButton::new(check_on(pulse_info.power))).clicked(){
                                    pulse_info.power=!pulse_info.power;
                                };
                            });
                        });
                    })
                });
            });
            
        })
    })
}


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

fn check_on(on_off:bool)->eframe::egui::Image<'static>{
    match on_off{
        false=>{egui::Image::new(egui::include_image!("../../files/pngegg.png")).rounding(360.0)},
        true=>{egui::Image::new(egui::include_image!("../../files/pngegg2.png")).rounding(360.0)} 
     }
}
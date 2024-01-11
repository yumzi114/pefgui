use eframe::{egui::{Ui, self, InnerResponse, RichText, Sense, TextFormat, PointerState, Widget}, epaint::{Vec2, Color32, text::{LayoutJob, TextWrapping}, FontId, Pos2, vec2}, emath::Align, glow::TRUE};
use egui_extras::{TableBuilder, Column};
use super::{UserUi,MenuList,PulseInfo,VolatageInfo};
use crate::{keypad_view};
pub fn content_view(ui: &mut Ui,ctx: &egui::Context,uui:&mut UserUi, pulse_info:&mut PulseInfo, vol_info:&mut VolatageInfo)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.columns(2, |columns|{
            columns[0].vertical_centered(|ui|{
                ui.add_space(100.);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("High Voltage").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        // ui.style_mut().text_styles.insert(
                        //     egui::TextStyle::Button,
                        //     egui::FontId::new(100.0, eframe::epaint::FontFamily::Proportional),
                        // );
                        let b_response = ui.add(egui::Button::new(RichText::new(format!("{} Kv",vol_info.value.to_string())).strong().size(90.0)).min_size(Vec2{x:50.0,y:130.0}).sense(Sense::click()));
                            if b_response.clicked(){
                                // ui.style_mut().spacing.button_padding = (0.0, 500.0).into();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::SetVoltage,pos);
                            }
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Frequency").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        let b_response = ui.add(egui::Button::new(RichText::new(format!("{} Hz",pulse_info.freq_value.to_string())).strong().size(90.0)).min_size(Vec2{x:50.0,y:130.0}).sense(Sense::click()));
                        if b_response.clicked(){
                            let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                            click_voltage(uui,MenuList::PulseFreq,pos);
                        }
                    });
                });
                ui.add_space(100.);
                ui.label(RichText::new("Pulse Time").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("ON").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        
                        // if pulse_info.power {
                        //     ui.label(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN));
                        // }
                        let b_response = ui.add(egui::Button::new(RichText::new(format!("{} ms",pulse_info.on_time_value.to_string())).strong().size(90.0)).min_size(Vec2{x:50.0,y:130.0}).sense(Sense::click()));
                        if b_response.clicked(){
                            let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                            click_voltage(uui,MenuList::PulseOnTime,pos);
                        }
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("OFF").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                        let b_response = ui.add(egui::Button::new(RichText::new(format!("{} ms",pulse_info.off_time_value.to_string())).strong().size(90.0)).min_size(Vec2{x:50.0,y:130.0}).sense(Sense::click()));
                        if b_response.clicked(){
                            let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                            click_voltage(uui,MenuList::PulseOffTime,pos);
                        }
                    });
                });
            });
            columns[1].vertical_centered(|ui|{
                if !uui.keypad.popon{
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
                }else {
                    ui.add_space(70.);
                    let mut test = String::new();
                    keypad_view(ui, ctx, pulse_info, vol_info, &uui.keypad.sellist, &mut uui.set_value, &mut uui.keypad.popon);
                }

            });
            
        })
    })
}


fn click_voltage(uui:&mut UserUi, selmenu:MenuList, get_pos:Pos2){
    if uui.keypad.popon && uui.keypad.sellist==Some(selmenu){
        uui.keypad.popon=false;
        uui.keypad.sellist=None;
        uui.keypad.uipost=get_pos;
    }
    else if uui.keypad.popon==false||uui.keypad.sellist!=Some(selmenu){
        uui.keypad.popon=true;
        uui.keypad.sellist=Some(selmenu);
        uui.keypad.uipost=get_pos;
    };
}

fn check_on(on_off:bool)->eframe::egui::Image<'static>{
    match on_off{
        false=>{egui::Image::new(egui::include_image!("../../files/pngegg.png")).rounding(360.0)},
        true=>{egui::Image::new(egui::include_image!("../../files/pngegg2.png")).rounding(360.0)} 
     }
}
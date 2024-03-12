use std::{fmt::format, sync::{Arc,Mutex}};

use crossbeam_channel::Sender;
use eframe::{egui::{Ui, self, InnerResponse, RichText, Sense, TextFormat, PointerState, Widget}, epaint::{Vec2, Color32, text::{LayoutJob, TextWrapping}, FontId, Pos2, vec2}};
use egui_extras::{TableBuilder, Column};
use pefapi::{device::AppState, LineCodec, RequestData, RequestDataList};
use super::{UserUi,MenuList,PulseInfo,VolatageInfo,};
use crate::{keypad_view};
pub fn setting_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    pulse_info:&mut PulseInfo, 
    vol_info:&mut VolatageInfo,
    request:&mut RequestData, 
    sender:&mut Sender<RequestData>,
    response:&Arc<Mutex<Vec<RequestDataList>>>,
    app_state:&mut Arc<Mutex<AppState>>,
    // sys_time:&Arc<Mutex<String>>
)->InnerResponse<()>{
    let mem = response.clone();
    let app_state_mem = app_state.clone();
    if uui.warning_pop{
        ui.set_enabled(false);
    }
    ui.vertical_centered(|ui|{
        ui.columns(2, |columns|{
            //좌측패널 컴퍼넌트
            columns[0].vertical_centered(|ui|{
                ui.add_space(100.);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response = button_respone(ui, uui, &MenuList::SetVoltage, format!("{} kV",vol_info.value.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="Voltage Value Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::SetVoltage,pos);
                            }
                        });
                        
                        let value = format!("Device : {}",mem.lock().unwrap()[11]);
                        ui.label(RichText::new(value.as_str()).strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Frequency").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response: egui::Response = button_respone(ui, uui, &MenuList::PulseFreq, format!("{} Hz",pulse_info.freq_value.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="Pulse Value Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::PulseFreq,pos);
                            }
                        });
                        let value = format!("Device : {}",mem.lock().unwrap()[7]);
                        ui.label(RichText::new(value.as_str()).strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                    });
                });
                ui.add_space(60.);
                ui.label(RichText::new("Time Value").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Time").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response = button_respone(ui, uui, &MenuList::PulseOnTime, format!("{} ms",pulse_info.on_time_value.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="Pulse ON_TIME Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::PulseOnTime,pos);
                            }
                        });
                        let value = format!("Device : {}",mem.lock().unwrap()[8]);
                        ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Running Time").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response = button_respone(ui, uui, &MenuList::RunningTime, format!("{} M",(*app_state_mem.lock().unwrap()).set_time.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="App Run Time Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::RunningTime,pos);
                            }
                        });
                        if (*app_state_mem.lock().unwrap()).set_time>0{
                            ui.label(RichText::new(
                                format!("Limit Time : {}",(*app_state_mem.lock().unwrap()).limit_time.to_string())
                            ).strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                        }
                        else {
                            ui.label(RichText::new("Set Time : None").strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                        }
                        // match (*app_state_mem.lock().unwrap()).set_time {
                        //     Some(time)=>{
                        //         ui.label(RichText::new(
                        //             format!("Set Time : {}",time.to_string())
                        //         ).strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                        //     },
                        //     None=>{
                        //         ui.label(RichText::new("Set Time : None").strong().size(50.0).color(Color32::from_rgb(184, 184, 184)));
                        //     }
                        // }
                    });
                });
            });
            //우측패널 컴퍼넌트
            columns[1].vertical_centered(|ui|{
                if !uui.keypad.popon{
                    ui.add_space(95.);
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
                                        vol_info.power=!vol_info.power;
                                        vol_info.save(request,sender);
                                    };
                                });
                                row.col(|ui| {
                                    ui.add_space(10.0);
                                    if ui.add_sized([120.0, 120.0], egui::ImageButton::new(check_on(pulse_info.power))).clicked(){
                                        pulse_info.power=!pulse_info.power;
                                        pulse_info.save(request,sender);
                                    };
                                });
                            });
                        })
                    });
                    ui.add_space(15.);
                    ui.label(RichText::new("Monitoring").strong().size(45.0).color(Color32::from_rgb(255, 200, 36)));
                    ui.push_id(2, |ui| {
                        TableBuilder::new(ui)
                        .cell_layout(egui::Layout::top_down(egui::Align::Center))
                        .striped(true)
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .header(70.0, |mut header| {
                            header.col(|ui| {
                                ui.add_space(10.);
                                
                                ui.label(RichText::new("Voltage").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            header.col(|ui| {
                                ui.add_space(10.);
                                ui.label(RichText::new(format!("{}",mem.lock().unwrap()[11])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                        })
                        .body(|mut body| {
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new(format!("{}",mem.lock().unwrap()[7])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                            });
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("Power\nConsumption").strong().size(30.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new(format!("{}",mem.lock().unwrap()[14])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                            });
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("Device\nLimit Time").strong().size(30.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new(format!("{}",mem.lock().unwrap()[4])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                            });
                        });
                    });
                    
                }else {
                    ui.add_space(50.);
                    keypad_view(
                        ui, 
                        ctx, 
                        pulse_info, 
                        vol_info, 
                        &mut uui.keypad.sellist, 
                        &mut uui.set_value, 
                        &mut uui.keypad.popon, 
                        &mut uui.status_str,
                        request,
                        sender,
                        app_state,
                        &mut uui.warning_pop,
                    );
                }
            });
        })
    })
}

//메뉴리스트 중복처리
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

//버튼 클릭색상처리
fn button_respone(ui: &mut Ui, uui:&UserUi, check_sel:&MenuList,value_str:String)->egui::Response{
    let b_response: egui::Response = 
    if uui.keypad.sellist==Some(*check_sel){
        ui.add(egui::Button::new(RichText::new(value_str).strong().size(90.0)).min_size(Vec2{x:420.0,y:130.0}).sense(Sense::click()).fill(Color32::from_rgb(133, 255, 115)).rounding(egui::Rounding{nw:50.,ne:50.,sw:50.,se:50.,}))
    }
    else {
        ui.add(egui::Button::new(RichText::new(value_str).strong().size(90.0)).min_size(Vec2{x:420.0,y:130.0}).rounding(egui::Rounding{nw:40.,ne:40.,sw:40.,se:40.,}).sense(Sense::click()))
    };
    b_response
}

fn check_on(on_off:bool)->eframe::egui::Image<'static>{
    match on_off{
        false=>{egui::Image::new(egui::include_image!("../../files/pngegg.png")).rounding(360.0)},
        true=>{egui::Image::new(egui::include_image!("../../files/pngegg2.png")).rounding(360.0)} 
     }
}
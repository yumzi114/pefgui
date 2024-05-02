
use std::sync::{Arc, Mutex};

use super::MenuList;
use crossbeam_channel::Sender;
use pefapi::{device::{AppState, PulseInfo, VolatageInfo}, RequestData};
use eframe::egui::PointerState;
use eframe::{egui::{Ui, self, InnerResponse, RichText}, epaint::Vec2};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;
use super::{UserUi};
pub fn keypad_view(
    ui: &mut Ui,
    ctx: &egui::Context, 
    pulse:&mut PulseInfo, 
    volat:&mut VolatageInfo, 
    selmenu:&mut Option<MenuList>, 
    setvalue:&mut String, 
    open:&mut bool, 
    status_str:&mut String,
    request:&mut RequestData,
    sender:&mut Sender<RequestData>,
    app_state:&mut Arc<Mutex<AppState>>,
    warring_open:&mut bool,
    timer_sender:&mut Sender<usize>,
    k_timer_sender:&mut Sender<u8>,
)->InnerResponse<()>{
    let title = match selmenu {
        Some(MenuList::SetVoltage)=>"High Voltage Set",
        Some(MenuList::PulseFreq)=>"Pulse Frequency Set",
        Some(MenuList::PulseOnTime)=>"Pulse ON_Time Set",
        Some(MenuList::RunningTime)=>"App Runtime Set",
        _=>""
    };
    // let status_mem = status_str.clone();
    // ui.add_space(5.);
    ui.vertical_centered_justified(|ui|{
        ui.add(egui::Label::new(RichText::new(format!("{}",title)).color(egui::Color32::WHITE).strong().size(50.0)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
            ui.add_space(70.);
            ui.add(egui::Label::new(RichText::new(format!("Value : {}",setvalue)).color(egui::Color32::WHITE).strong().size(40.0)));
        });
    });
    
    ui.horizontal_wrapped(|ui|{
        ui.columns(4, |columns|{
            columns[0].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("7").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('7');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("4").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('4');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("1").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('1');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("Close").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    *selmenu=None;
                    *open=false;
                }
            });
            columns[1].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("8").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('8');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("5").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('5');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("2").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('2');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("0").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('0');
                    k_timer_sender.send(5).unwrap();
                }
            });
            columns[2].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("9").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('9');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("6").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('6');
                    k_timer_sender.send(5).unwrap();
                }
                if ui.add(egui::Button::new(RichText::new("3").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('3');
                    k_timer_sender.send(5).unwrap();
                }
                match selmenu {
                    Some(MenuList::SetVoltage)=>{
                        if ui.add(egui::Button::new(RichText::new(".").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                            setvalue.push('.');
                        }
                    },
                    Some(MenuList::RunningTime)=>{
                        if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        }
                    },
                    _=>{
                        if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                            k_timer_sender.send(5).unwrap();
                        }
                    }
                }
                
            });
            columns[3].vertical_centered_justified(|ui|{
                ui.horizontal_wrapped(|ui|{
                    if ui.add(egui::Button::new(RichText::new("Set").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(180., 242.5)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        match selmenu {
                            Some(MenuList::PulseFreq)=>{
                                if setvalue.parse::<u64>().unwrap_or(0) >1000||
                                setvalue.parse::<u64>().unwrap_or(0)==0{
                                    *status_str="Limit value (1 ~ 1000 Hz)".to_string();
                                    *warring_open=true;
                                    setvalue.clear();
                                }else {
                                    pulse.freq_value=setvalue.parse::<u16>().unwrap_or(0);
                                    pulse.max_value_change();
                                    if setvalue.parse::<u64>().unwrap_or(0)==0{
                                        pulse.save(request,sender);
                                    }
                                    if let Some(value)=pulse.max_time_value{
                                        if pulse.on_time_value >=u16::from(value){
                                            pulse.on_time_value=0;
                                            *status_str=format!("Frequency Value out of range (1 ~ {}us)",value-1);
                                            *warring_open=true;
                                            
                                            setvalue.clear();
                                            *selmenu=Some(MenuList::PulseOnTime);
                                        }
                                        else{
                                            *status_str=format!("Set Done Value : {} ", pulse.freq_value.to_string());
                                            pulse.save(request,sender);
                                        }
                                    };
                                    setvalue.clear();
                                }
                                k_timer_sender.send(5).unwrap();
                            },
                            Some(MenuList::PulseOffTime)=>{
                      
                            },
                            Some(MenuList::PulseOnTime)=>{
                                if let Some(value)=pulse.max_time_value{
                                    if setvalue.parse::<u64>().unwrap_or(0) >=u64::from(value)||
                                    setvalue.parse::<u64>().unwrap_or(0) <1{
                                        *status_str=format!("Frequency Value out of range (1 ~ {}us)",value-1);
                                        *warring_open=true;
                                        setvalue.clear();
                                    }else {
                                        pulse.on_time_value=setvalue.parse::<u16>().unwrap_or(0);
                                        pulse.save(request,sender);
                                        
                                        *status_str=format!("Set Done Value : {} ", pulse.on_time_value.to_string());
                                        setvalue.clear();
                                    }
                                    k_timer_sender.send(5).unwrap();
                                };
                                
                            },
                            Some(MenuList::RunningTime)=>{
                                if setvalue.parse::<u64>().unwrap_or(0) >5000{
                                    *status_str="Limit value (0 ~ 5000M)".to_string();
                                    *warring_open=true;
                                    setvalue.clear();
                                }else {
                                    let num = format!("{}", setvalue.parse::<u16>().unwrap_or(0));
                                    let mut temp = (*app_state.lock().unwrap()).clone();
                                    temp.set_time=num.parse::<u16>().unwrap_or(0);
                                    temp.limit_time=num.parse::<u16>().unwrap_or(0);
                                    timer_sender.send(temp.set_time as usize).unwrap();
                                    *app_state.lock().unwrap()=temp;
                                    *status_str=format!("Set Done Value : {} ", num.to_string());
                                    setvalue.clear();
                                }
                                k_timer_sender.send(5).unwrap();
                            },
                            Some(MenuList::SetVoltage)=>{
                                if setvalue.parse::<f32>().unwrap_or(0.) >20.0
                                ||setvalue.parse::<f32>().unwrap_or(0.) <4.&&setvalue.parse::<f32>().unwrap_or(0.)>=1.{
                                    *status_str="Limit value (4 ~ 20 Kv)".to_string();
                                    *warring_open=true;
                                    setvalue.clear();
                                }else {
                                    // let num = format!("{:.01}", setvalue.parse::<u16>().unwrap_or(0.));
                                    volat.value=setvalue.parse::<f32>().unwrap_or(0.0);
                                    volat.save(request,sender);
                                    *status_str=format!("Set Done Value : {} ", volat.value.to_string());
                                    setvalue.clear();
                                }
                                k_timer_sender.send(5).unwrap();
                            },
                            _=>{}
                        }
                    }
                    if ui.add(egui::Button::new(RichText::new("Cancel").color(egui::Color32::BLACK).strong().size(40.0)).min_size(Vec2::new(180., 242.5)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        match selmenu {
                            Some(_)=>{
                                setvalue.clear();
                            },
                            _=>{}
                        }
                    }
                });
            });
        });
        
        
    })
    
    
}
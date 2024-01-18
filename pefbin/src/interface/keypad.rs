use std::sync::{Arc,Mutex};

use super::{PulseInfo,VolatageInfo};
use super::MenuList;
use futures::stream::SplitSink;
use pefapi::{LineCodec, AppChannel};
use eframe::egui::PointerState;
use eframe::{egui::{Ui, self, InnerResponse, RichText}, epaint::Vec2};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;
use super::{UserUi};
pub fn keypad_view(ui: &mut Ui,ctx: &egui::Context, pulse:&mut PulseInfo, volat:&mut VolatageInfo, selmenu:&mut Option<MenuList>, setvalue:&mut String, open:&mut bool, status_str:&mut String,app_chennel:&mut AppChannel)->InnerResponse<()>{
    let title = match selmenu {
        Some(MenuList::SetVoltage)=>"High Voltage Set",
        Some(MenuList::PulseFreq)=>"Pulse Frequency Set",
        Some(MenuList::PulseOffTime)=>"Pulse OFF_Time Set",
        Some(MenuList::PulseOnTime)=>"Pulse ON_Time Set",
        _=>""
    };
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
                }
                if ui.add(egui::Button::new(RichText::new("4").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('4');
                }
                if ui.add(egui::Button::new(RichText::new("1").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('1');
                }
                if ui.add(egui::Button::new(RichText::new("Close").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    *selmenu=None;
                    *open=false;
                }
            });
            columns[1].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("8").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('8');
                }
                if ui.add(egui::Button::new(RichText::new("5").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('5');
                }
                if ui.add(egui::Button::new(RichText::new("2").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('2');
                }
                if ui.add(egui::Button::new(RichText::new("0").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('0');
                }
            });
            columns[2].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("9").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('9');
                }
                if ui.add(egui::Button::new(RichText::new("6").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('6');
                }
                if ui.add(egui::Button::new(RichText::new("3").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('3');
                }
                if ui.add(egui::Button::new(RichText::new(".").color(egui::Color32::BLACK).strong().size(80.0)).min_size(Vec2::new(50., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('.');
                }
            });
            columns[3].vertical_centered_justified(|ui|{
                ui.horizontal_wrapped(|ui|{
                    if ui.add(egui::Button::new(RichText::new("Set").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(180., 242.5)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        match selmenu {
                            Some(MenuList::PulseFreq)=>{
                                if (setvalue.parse::<f32>().unwrap_or(0.) >1000.0){
                                    *status_str="Limit value (0 ~ 1000 Hz)".to_string();
                                    setvalue.clear();
                                    // *status_str="Insert Value".to_string();
                                }else {
                                    // pulse.freq_value=;
                                    let num = format!("{:.01}", setvalue.parse::<f32>().unwrap_or(0.));
                                    pulse.freq_value=num.parse::<f32>().unwrap_or(0.);
                                    pulse.save(app_chennel);
                                    *status_str=format!("Set Done Value : {} ", pulse.freq_value.to_string());
                                    setvalue.clear();
                                }
                            },
                            Some(MenuList::PulseOffTime)=>{
                                if (setvalue.parse::<f32>().unwrap_or(0.) >100.0){
                                    *status_str="Limit value (0 ~ 100 ms)".to_string();
                                    setvalue.clear();
                                    // *status_str="Insert Value".to_string();
                                }else {
                                    let num = format!("{:.01}", setvalue.parse::<f32>().unwrap_or(0.));
                                    pulse.off_time_value=num.parse::<f32>().unwrap_or(0.);
                                    pulse.save(app_chennel);
                                    *status_str=format!("Set Done Value : {} ", pulse.off_time_value.to_string());
                                    setvalue.clear();
                                }
                                // setvalue.clear();
                            },
                            Some(MenuList::PulseOnTime)=>{
                                if (setvalue.parse::<f32>().unwrap_or(0.) >100.0){
                                    *status_str="Limit value (0 ~ 100 ms)".to_string();
                                    setvalue.clear();
                                    // *status_str="Insert Value".to_string();b
                                }else {
                                    let num = format!("{:.01}", setvalue.parse::<f32>().unwrap_or(0.));
                                    pulse.on_time_value=num.parse::<f32>().unwrap_or(0.);
                                    pulse.save(app_chennel);
                                    *status_str=format!("Set Done Value : {} ", pulse.on_time_value.to_string());
                                    setvalue.clear();
                                }
                                // setvalue.clear();
                            },
                            Some(MenuList::SetVoltage)=>{
                                if (setvalue.parse::<f32>().unwrap_or(0.) >20.0){
                                    *status_str="Limit value (0 ~ 20 Kv)".to_string();
                                    setvalue.clear();
                                }else {
                                    let num = format!("{:.01}", setvalue.parse::<f32>().unwrap_or(0.));
                                    volat.value=num.parse::<f32>().unwrap_or(0.);
                                    volat.save(app_chennel);
                                    *status_str=format!("Set Done Value : {} ", volat.value.to_string());
                                    setvalue.clear();
                                }
                            },
                            _=>{}
                        }
                    }
                    if ui.add(egui::Button::new(RichText::new("Cancel").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(180., 242.5)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        match selmenu {
                            Some(MenuList::PulseFreq)=>{
                                // pulse.freq_value=0.;
                                setvalue.clear();
                            },
                            Some(MenuList::PulseOffTime)=>{
                                // pulse.off_time_value=0.;
                                setvalue.clear();
                            },
                            Some(MenuList::PulseOnTime)=>{
                                // pulse.on_time_value=0.;
                                setvalue.clear();
                            },
                            Some(MenuList::SetVoltage)=>{
                                // volat.value=0.;
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
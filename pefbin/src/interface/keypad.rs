use super::{PulseInfo,VolatageInfo};
use super::MenuList;
use eframe::egui::PointerState;
use eframe::{egui::{Ui, self, InnerResponse, RichText}, epaint::Vec2};

pub fn keypad_view(ui: &mut Ui,ctx: &egui::Context, pulse:&mut PulseInfo, volat:&mut VolatageInfo, selmenu:&Option<MenuList>, setvalue:&mut String, open:&mut bool)->InnerResponse<()>{
    // let mut number = String::new();
    match selmenu {
        Some(MenuList::PulseFreq)=>{
            if !(pulse.freq_value==0.){
                // *setvalue=pulse.freq_value.to_string();
            }
        },
        Some(MenuList::SetVoltage)=>{
            if !(volat.value==0.){
                // *setvalue=volat.value.to_string();
            }
        }
        Some(MenuList::PulseOffTime)=>{
            if !(pulse.off_time_value==0.){
                // *setvalue=pulse.time_value.to_string();
            }
        },
        Some(MenuList::PulseOnTime)=>{
            if !(pulse.on_time_value==0.){
                // *setvalue=pulse.time_value.to_string();
            }
        },
        _=>{}
    }
    // ui.max_rect();
    let title = match selmenu {
        Some(MenuList::SetVoltage)=>"High Voltage",
        Some(MenuList::PulseFreq)=>"Pulse Frequency",
        Some(MenuList::PulseOffTime)=>"Pulse OFF_Time",
        Some(MenuList::PulseOnTime)=>"Pulse ON_Time",
        _=>""
    };
    ui.horizontal_wrapped(|ui|{
        ui.add(egui::Label::new(RichText::new(format!("{}",title)).color(egui::Color32::WHITE).strong().size(60.0)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
            ui.add(egui::Label::new(RichText::new(format!("Value : {}  |",setvalue)).color(egui::Color32::WHITE).strong().size(40.0)));
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
                                    pulse.freq_value=setvalue.parse::<f32>().unwrap_or(0.);
                                    setvalue.clear();
                                },
                                Some(MenuList::PulseOffTime)=>{
                                    pulse.off_time_value=setvalue.parse::<f32>().unwrap_or(0.);
                                    setvalue.clear();
                                },
                                Some(MenuList::PulseOnTime)=>{
                                    pulse.on_time_value=setvalue.parse::<f32>().unwrap_or(0.);
                                    setvalue.clear();
                                },
                                Some(MenuList::SetVoltage)=>{
                                    volat.value=setvalue.parse::<f32>().unwrap_or(0.);
                                    setvalue.clear();
                                },
                                _=>{}
                            }
                    }
                    if ui.add(egui::Button::new(RichText::new("Cancel").color(egui::Color32::BLACK).strong().size(50.0)).min_size(Vec2::new(180., 242.5)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                        match selmenu {
                                Some(MenuList::PulseFreq)=>{
                                    pulse.freq_value=0.;
                                    setvalue.clear();
                                },
                                Some(MenuList::PulseOffTime)=>{
                                    pulse.off_time_value=0.;
                                    setvalue.clear();
                                },
                                Some(MenuList::PulseOnTime)=>{
                                    pulse.on_time_value=0.;
                                    setvalue.clear();
                                },
                                Some(MenuList::SetVoltage)=>{
                                    volat.value=0.;
                                    setvalue.clear();
                                },
                                _=>{}
                            }
                    }
                    // ui.horizontal_centered(|ui|{
                        
                    // });
                    // ui.horizontal(|ui|{
                    //     ui.label("text");
                    // });
                });
            });
            
            // columns[3].vertical_centered_justified(|ui|{
            //     if ui.add(egui::Button::new(RichText::new(setvalue.as_str()).color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
            //     }
            //     if ui.add(egui::Button::new(RichText::new("SET").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
            //         match selmenu {
            //             Some(MenuList::PulseFreq)=>{
            //                 pulse.freq_value=setvalue.parse::<f32>().unwrap_or(0.);
            //             },
            //             Some(MenuList::PulseOffTime)=>{
            //                 pulse.time_value=setvalue.parse::<f32>().unwrap_or(0.);
            //             },
            //             Some(MenuList::SetVoltage)=>{
            //                 volat.value=setvalue.parse::<f32>().unwrap_or(0.);
            //             },
            //             _=>{}
            //         }
            //     }
            //     if ui.add(egui::Button::new(RichText::new("CANCLE").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
            //         match selmenu {
            //             Some(MenuList::PulseFreq)=>{
            //                 pulse.freq_value=0.;
            //                 setvalue.clear();
            //             },
            //             Some(MenuList::PulseOffTime)=>{
            //                 pulse.time_value=0.;
            //                 setvalue.clear();
            //             },
            //             Some(MenuList::SetVoltage)=>{
            //                 volat.value=0.;
            //                 setvalue.clear();
            //             },
            //             _=>{}
            //         }
            //     }
            //     if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
            //     }
            // });
        });
        
        
    })
    
    
}
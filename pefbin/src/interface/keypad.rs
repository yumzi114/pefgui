use super::{PulseInfo,VolatageInfo};
use super::MenuList;
use eframe::egui::PointerState;
use eframe::{egui::{Ui, self, InnerResponse, RichText}, epaint::Vec2};

pub fn keypad_view(ui: &mut Ui,ctx: &egui::Context, pulse:&mut PulseInfo, volat:&mut VolatageInfo, selmenu:&Option<MenuList>, setvalue:&mut String)->InnerResponse<()>{
    // let mut number = String::new();
    match selmenu {
        Some(MenuList::PulseFreq)=>{
            if !(pulse.freq_value==0.){
                *setvalue=pulse.freq_value.to_string();
            }
        },
        Some(MenuList::PulseTime)=>{
            if !(pulse.time_value==0.){
                *setvalue=pulse.time_value.to_string();
            }
        },
        Some(MenuList::SetVoltage)=>{
            if !(volat.value==0.){
                *setvalue=volat.value.to_string();
            }
        }
        _=>{}
    }
    ui.horizontal_wrapped(|ui|{
        ui.max_rect();
        ui.columns(4, |columns|{
            columns[0].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("7").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('7');
                }
                if ui.add(egui::Button::new(RichText::new("4").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('4');
                }
                if ui.add(egui::Button::new(RichText::new("1").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('1');
                }
                if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
            columns[1].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("8").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('8');
                }
                if ui.add(egui::Button::new(RichText::new("5").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('5');
                }
                if ui.add(egui::Button::new(RichText::new("2").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('2');
                }
                if ui.add(egui::Button::new(RichText::new("0").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('0');
                }
            });
            columns[2].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new("9").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('9');
                }
                if ui.add(egui::Button::new(RichText::new("6").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('6');
                }
                if ui.add(egui::Button::new(RichText::new("3").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('3');
                }
                if ui.add(egui::Button::new(RichText::new(".").color(egui::Color32::BLACK).strong().size(60.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    setvalue.push('.');
                }
            });
            columns[3].vertical_centered_justified(|ui|{
                if ui.add(egui::Button::new(RichText::new(setvalue.as_str()).color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
                if ui.add(egui::Button::new(RichText::new("SET").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    match selmenu {
                        Some(MenuList::PulseFreq)=>{
                            pulse.freq_value=setvalue.parse::<f32>().unwrap_or(0.);
                        },
                        Some(MenuList::PulseTime)=>{
                            pulse.time_value=setvalue.parse::<f32>().unwrap_or(0.);
                        },
                        Some(MenuList::SetVoltage)=>{
                            volat.value=setvalue.parse::<f32>().unwrap_or(0.);
                        },
                        _=>{}
                    }
                }
                if ui.add(egui::Button::new(RichText::new("CLEAR").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                    match selmenu {
                        Some(MenuList::PulseFreq)=>{
                            pulse.freq_value=0.;
                            setvalue.clear();
                        },
                        Some(MenuList::PulseTime)=>{
                            pulse.time_value=0.;
                            setvalue.clear();
                        },
                        Some(MenuList::SetVoltage)=>{
                            volat.value=0.;
                            setvalue.clear();
                        },
                        _=>{}
                    }
                }
                if ui.add(egui::Button::new(RichText::new("").color(egui::Color32::BLACK).strong().size(30.0)).min_size(Vec2::new(120., 120.)).fill(egui::Color32::from_rgb(234, 237, 173))).clicked() {
                }
            });
        })
    })
}
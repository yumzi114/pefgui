use std::default;
use std::sync::{Arc, Mutex,atomic::{AtomicBool, Ordering}};
use eframe::{egui::{self,Ui, InnerResponse, RichText, Slider, collapsing_header::CollapsingState, Response, Sense, TextStyle}, epaint::{Color32, Stroke, vec2, Vec2}};
use egui_extras::{StripBuilder, Size, Column, TableBuilder};
use super::MenuStatus;
pub struct ExecuteSettings{
    pub power:Arc<AtomicBool>,
    pub power_time:Arc<Mutex<u64>>,
    pub water:bool,
    pub medicine:bool,
    pub carbonated:bool,
    pub carboption:Carbonauto,
    pub run:Arc<AtomicBool>,
    pub run_time:Arc<Mutex<u64>>
}
pub struct Carbonauto{
    pub auto:bool,
    pub data:f32
}
impl ::std::default::Default for ExecuteSettings {
    fn default() -> Self { 
        Self { 
            power:Arc::new(AtomicBool::new(false)), 
            power_time:Arc::new(Mutex::new(0)),
            water:false,
            medicine:false,
            carbonated:false,
            carboption:Default::default(),
            run:Arc::new(AtomicBool::new(false)),
            run_time:Arc::new(Mutex::new(0)),
    } }
}
impl ::std::default::Default for Carbonauto {
    fn default() -> Self { 
        Self { 
            auto:false, 
            data:0.0,
    } }
}
impl ExecuteSettings{
    pub fn exec_menu(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        ui.columns(5, |columns|{
            columns[0].vertical_centered_justified(|ui|{
                // ui.add_space(40.0);
                ui.add_sized([0., 40.],egui::Label::new(RichText::new("전 원").strong().size(50.0)));
                if ui.add_sized([0., 75.], self.fontcolorbutton(self.power.load(Ordering::Relaxed))).clicked(){
                    self.power.store(!self.power.load(Ordering::Relaxed), Ordering::Relaxed);
                    self.run.store(false, Ordering::Relaxed);
                    *self.power_time.lock().unwrap()=0;
                };
                ui.add_space(10.0);
            });
            columns[1].vertical_centered_justified(|ui|{
                ui.add_sized([0., 40.],egui::Label::new(RichText::new("원 수").strong().size(50.0)));
                if ui.add_enabled(self.power.load(Ordering::Relaxed)&&!self.run.load(Ordering::Relaxed), self.fontcolorbutton(self.water)).clicked() {
                    self.water=!self.water;
                }
                // if ui.add_sized([0., 80.], self.fontcolorbutton(self.water)).clicked(){
                //     self.water=!self.water;
                // };
            });
            columns[2].vertical_centered_justified(|ui|{
                ui.add_sized([0., 40.],egui::Label::new(RichText::new("약 품").strong().size(50.0)));
                if ui.add_enabled(self.power.load(Ordering::Relaxed)&&!self.run.load(Ordering::Relaxed), self.fontcolorbutton(self.medicine)).clicked() {
                    self.medicine=!self.medicine;
                }
                // if ui.add_sized([0., 80.], self.fontcolorbutton(self.medicine)).clicked(){
                //     self.medicine=!self.medicine;
                // };
                
            });
            columns[3].vertical_centered_justified(|ui|{
                ui.add_sized([0., 40.],egui::Label::new(RichText::new("탄 산").strong().size(50.0)));
                if ui.add_enabled(self.power.load(Ordering::Relaxed)&&!self.run.load(Ordering::Relaxed), self.fontcolorbutton(self.carbonated)).clicked() {
                    self.carbonated=!self.carbonated;
                }
                // if ui.add_sized([0., 80.], self.fontcolorbutton(self.carbonated)).clicked(){
                //     self.carbonated=!self.carbonated;
                // };
            });
            columns[4].vertical_centered_justified(|ui|{
                if self.carbonated&&self.power.load(Ordering::Relaxed){
                    ui.add_space(10.0);
                    if ui.add_enabled(self.power.load(Ordering::Relaxed)&&!self.run.load(Ordering::Relaxed), self.carbonated_button(self.carboption.auto).min_size(Vec2 { x: 30., y: 40. })).clicked() {
                        self.carboption.auto=!self.carboption.auto;
                    }
                    // if ui.add_sized([0., 30.], self.carbonated_button(self.carboption.auto)).clicked(){
                    //     self.carboption.auto=!self.carboption.auto;
                    // };
                    if !self.carboption.auto{
                        ui.add_space(10.0);
                        
                        ui.style_mut().spacing.slider_width = (480.0);
                        // ui.add(egui::Slider::new(&mut self.carboption.data, 0.0..=10.0).show_value(false));
                        ui.add_enabled(self.power.load(Ordering::Relaxed)&&!self.run.load(Ordering::Relaxed), egui::Slider::new(&mut self.carboption.data, 0.0..=10.0).show_value(false));
                        ui.add(egui::Label::new(RichText::new(self.carboption.data.to_string()).strong().size(50.0)));
                        
                    }
                }
            })
        })
    }
    fn fontcolorbutton (&self, mode:bool)->egui::Button{
        if !mode{
            egui::Button::new(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN)).min_size(Vec2 { x: 30., y: 75. })    
        }else {
            egui::Button::new(RichText::new("OFF").strong().size(50.0).color(Color32::LIGHT_RED)).min_size(Vec2 { x: 30., y: 75. })
        }
    }
    fn carbonated_button (&self, mode:bool)->egui::Button{
        if !mode{
            egui::Button::new(RichText::new("자 동").strong().size(30.0))    
        }else {
            egui::Button::new(RichText::new("수 동").strong().size(30.0))
        }
    }
}
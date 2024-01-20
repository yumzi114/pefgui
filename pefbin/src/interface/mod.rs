use std::sync::{Mutex, Arc};

use crossbeam_channel::Sender;
use eframe::{egui::{Ui, self, InnerResponse}, epaint::{Pos2}};
use pefapi::RequestData;
mod head_bar;
mod bottom_bar;
mod content;
pub mod keypad;
use super::{PulseInfo,VolatageInfo};
#[derive(PartialEq,Debug,Clone, Copy)]
pub enum  MenuList{
    SetVoltage,
    PulseFreq,
    PulseOffTime,
    PulseOnTime,
}
#[derive(PartialEq)]
pub struct OpenMenu{
    pub popon:bool,
    pub sellist:Option<MenuList>,
    pub uipost:Pos2,
}
impl ::std::default::Default for OpenMenu {
    fn default() -> Self { 
        Self{
            popon:false,
            sellist:None,
            uipost:Pos2 { x: 50., y: 50. },
        }
    }
}

#[derive(PartialEq)]
pub struct UserUi{
    pub keypad: OpenMenu,
    set_value:String,
    status_str:String,
}
impl ::std::default::Default for UserUi {
    fn default() -> Self { 
        Self{
            keypad:OpenMenu::default(),
            set_value:String::new(),
            status_str:String::new()
        }
    }
}

//전체 UI에 대한 출력 메소드정의
impl UserUi {
    pub fn head_view(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::top("top_view").show_separator_line(false).show(ctx, |ui| {
            head_bar::top_logo_view(ui, ctx);
        })
    }
    pub fn content_view(&mut self,ui: &mut Ui,ctx: &egui::Context,pulse_info:&mut PulseInfo, vol_info:&mut VolatageInfo, request:&mut RequestData,sender:&mut Sender<RequestData>)->InnerResponse<()>{
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            content::content_view(ui, ctx,self,pulse_info,vol_info,request,sender);
        })
    }
    pub fn bottom_view(&mut self,ui: &mut Ui,ctx: &egui::Context, mem:&Arc<Mutex<usize>>)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::bottom("bottom_view").show_separator_line(false).show(ctx, |ui| {
            bottom_bar::bottom_view(ui, ctx,mem, self);
        })
    }
}
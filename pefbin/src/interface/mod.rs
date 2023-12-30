use std::sync::{Mutex, Arc};

use eframe::{egui::{Ui, self, InnerResponse}, epaint::Color32};

mod head_bar;
mod bottom_bar;
mod content;
pub mod keypad;
use super::{PulseInfo,VolatageInfo};
#[derive(PartialEq,Debug,Clone, Copy)]
pub enum  MenuList{
    SetVoltage,
    PulseFreq,
    PulseTime,
}
#[derive(PartialEq)]
pub struct OpenMenu{
    pub popon:bool,
    pub sellist:Option<MenuList>
}
impl ::std::default::Default for OpenMenu {
    fn default() -> Self { 
        Self{
            popon:false,
            sellist:None
        }
    }
}

#[derive(PartialEq)]
pub struct UserUi{
    pub keypad: OpenMenu,
}
impl ::std::default::Default for UserUi {
    fn default() -> Self { 
        Self{
            keypad:OpenMenu::default()
        }
    }
}
impl UserUi {
    pub fn head_view(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::top("top_view").show_separator_line(false).show(ctx, |ui| {
            head_bar::top_logo_view(ui, ctx);
        })
    }
    pub fn content_view(&mut self,ui: &mut Ui,ctx: &egui::Context,pulse_info:&mut PulseInfo, vol_info:&mut VolatageInfo)->InnerResponse<()>{
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            content::content_view(ui, ctx,self,pulse_info,vol_info);
        })
    }
    pub fn bottom_view(&mut self,ui: &mut Ui,ctx: &egui::Context, mem:&Arc<Mutex<usize>>)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::bottom("bottom_view").show_separator_line(false).show(ctx, |ui| {
            bottom_bar::bottom_view(ui, ctx,mem);
        })
    }
}
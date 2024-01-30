use std::sync::{Mutex, Arc};

use crossbeam_channel::Sender;
use eframe::{egui::{Ui, self, InnerResponse}, epaint::{Pos2}};
use pefapi::{RequestData, RequestDataList};
use super::ErrorList;
mod head_bar;
mod bottom_bar;
mod content;
mod main_view;
pub mod keypad;
use super::{PulseInfo,VolatageInfo};
#[derive(PartialEq,Debug,Clone, Copy)]
pub enum  MenuList{
    SetVoltage,
    PulseFreq,
    PulseOffTime,
    PulseOnTime,
}
#[derive(Default,Clone,Copy,PartialEq)]
pub enum ViewList{
    Setting,
    #[default]
    Main
}
#[derive(PartialEq,Clone,Copy)]
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

#[derive(Clone)]

pub struct UserUi{
    pub keypad: OpenMenu,
    set_value:String,
    pub status_str:String,
    pub view_menu:ViewList,
}
impl ::std::default::Default for UserUi {
    fn default() -> Self { 
        Self{
            keypad:OpenMenu::default(),
            set_value:String::new(),
            status_str:String::new(),
            view_menu:ViewList::default()
        }
    }
}

//전체 UI에 대한 출력 메소드정의
impl UserUi {
    pub fn head_view(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::top("top_view").show_separator_line(false).show(ctx, |ui| {
            head_bar::top_logo_view(ui, ctx,self);
        })
    }
    pub fn content_view(&mut self,ui: &mut Ui,ctx: &egui::Context,pulse_info:&mut PulseInfo, vol_info:&mut VolatageInfo, request:&mut RequestData,sender:&mut Sender<RequestData>,response:&Arc<Mutex<Vec<RequestDataList>>>)->InnerResponse<()>{
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            match self.view_menu {
                ViewList::Setting=>{
                    content::content_view(ui, ctx,self,pulse_info,vol_info,request,sender,response);
                },
                ViewList::Main=>{
                    main_view::main_view(ui, ctx, self);
                }
            }
            
        })
    }
    pub fn bottom_view(&mut self,ui: &mut Ui,ctx: &egui::Context, mem:&Arc<Mutex<usize>>,err_type:&Arc<Mutex<ErrorList>>)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::bottom("bottom_view").show_separator_line(false).show(ctx, |ui| {
            bottom_bar::bottom_view(ui, ctx,mem, self,err_type);
        })
    }
}
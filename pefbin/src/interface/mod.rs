use std::sync::{Mutex, Arc};

use chrono::{DateTime, Local};
use crossbeam_channel::Sender;
use eframe::{egui::{Ui, self, InnerResponse}, epaint::{Pos2}};
use pefapi::{device::AppState, RequestData, RequestDataList};
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
    RunningTime,
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
    pub table_sel:Vec<bool>,
    
}
impl ::std::default::Default for UserUi {
    fn default() -> Self { 
        Self{
            keypad:OpenMenu::default(),
            set_value:String::new(),
            status_str:String::new(),
            view_menu:ViewList::default(),
            table_sel:vec![false,false,false,false,false,false,false,false,false],
            
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
    pub fn content_view(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        pulse_info:&mut PulseInfo, 
        vol_info:&mut VolatageInfo, 
        request:&mut RequestData,
        sender:&mut Sender<RequestData>,
        response:&Arc<Mutex<Vec<RequestDataList>>>,
        sys_time:&Arc<Mutex<String>>,
        app_state:&mut Arc<Mutex<AppState>>,
    )->InnerResponse<()>{
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            match self.view_menu {
                ViewList::Setting=>{
                    content::content_view(ui, ctx,self,pulse_info,vol_info,request,sender,response,app_state);
                },
                ViewList::Main=>{
                    main_view::main_view(ui, ctx, self,response,sys_time,app_state);
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
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::time;
use std::{thread, time::Duration, sync::{Arc, Mutex}};
use serde_derive::{Serialize, Deserialize};
use eframe::{egui::{self, Sense, Ui, RichText, ViewportBuilder}, Theme, epaint::{Vec2, Color32}};
mod interface;
use interface::{UserUi,MenuList,keypad::keypad_view};

fn main() -> Result<(), eframe::Error> {
    //윈도우 사이즈
    let windows = ViewportBuilder{
        title: Some(String::from("Chorusing App")),
        app_id: Some(String::from("Chorusing App")),
        fullsize_content_view: Some(true),
        titlebar_shown: Some(false),
        resizable: Some(false),
        fullscreen:Some(true),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport:windows,
        default_theme:Theme::Dark,
        ..Default::default()
    };
    
    eframe::run_native(
        "PEF Board App",
        options,
        Box::new(|cc| {
            let mut app = PEFApp::new(cc);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let mem = app.thread_time.clone();
            thread::spawn(move||{
                loop{
                    thread::sleep(Duration::from_secs(1));
                    if *mem.lock().unwrap()<5{
                        *mem.lock().unwrap()+=1;
                    }
                    else {
                        *mem.lock().unwrap()=0;
                    }
                }
            });
            Box::<PEFApp>::new(app)
        }),
    )
}
#[derive(PartialEq, Serialize, Deserialize)]
pub struct VolatageInfo{
    power:bool,
    value:f32,
}
impl ::std::default::Default for VolatageInfo {
    fn default() -> Self { 
        Self{
            power: false,
            value: 0.,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PulseInfo{
    power:bool,
    freq_value:f32,
    time_value:f32
}
impl ::std::default::Default for PulseInfo {
    fn default() -> Self { 
        Self{
            power: false,
            freq_value: 0.,
            time_value: 0.,
        }
    }
}


struct PEFApp {
    mainui:UserUi,
    voltage:VolatageInfo,
    PulseInfo:PulseInfo,
    setvalue:String,
    thread_time:Arc<Mutex<usize>>
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let voltage = confy::load("pefapp", "vol").unwrap_or_default();
        let PulseInfo = confy::load("pefapp", "pulse").unwrap_or_default();
        let thread_time = Arc::new(Mutex::new(1));
        Self{
            mainui:UserUi::default(),
            voltage,
            PulseInfo,
            setvalue:String::new(),
            thread_time
        }
    }
    pub fn new_windows(&mut self, ctx: &egui::Context,){
        let title = match self.mainui.keypad.sellist {
            Some(MenuList::PulseFreq)=>"Set Pulse Freq Data",
            Some(MenuList::PulseTime)=>"Set Pulse Stop time",
            Some(MenuList::SetVoltage)=>"Set Voltage Data",
            _=>"UNKNOW"
        };
        let temp = egui::Window::new(
            RichText::new(title)
            .color(egui::Color32::LIGHT_YELLOW)
            .strong()
            .size(30.0))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(450., 450.))
            .id("pad".into())
            .open(&mut self.mainui.keypad.popon)
            .vscroll(false);
            temp.show(ctx, |ui| {
                keypad_view(ui, ctx, &mut self.PulseInfo, &mut self.voltage, &self.mainui.keypad.sellist, &mut self.setvalue);
            });
    }
    fn pop_window(&mut self, ui: &mut Ui,ctx: &egui::Context){
        self.new_windows(ctx);
    }   
}

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.pop_window(ui, ctx);
            self.mainui.head_view(ui, ctx);
            self.mainui.content_view(ui, ctx,&mut self.PulseInfo,&mut self.voltage);
            self.mainui.bottom_view(ui, ctx,&self.thread_time);
        });
    }
}
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../files/Pilseung_Gothic.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

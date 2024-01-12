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
            //신규 스레드
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
    off_time_value:f32,
    on_time_value:f32,
}
impl ::std::default::Default for PulseInfo {
    fn default() -> Self { 
        Self{
            power: false,
            freq_value: 0.,
            off_time_value: 0.,
            on_time_value: 0.,
        }
    }
}


struct PEFApp {
    mainui:UserUi,
    voltage:VolatageInfo,
    PulseInfo:PulseInfo,
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
            thread_time
        }
    }
}

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.pop_window(ui, ctx);
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

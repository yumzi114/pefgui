use std::{thread, sync::atomic::Ordering, time::Duration};

use eframe::{egui, Theme};

mod menus;

use crate::menus::MenuStatus;

#[derive(Default)]
struct PEFApp {
    menu:MenuStatus,
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for PEFApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
           self.menu.top_menu_bar(ui, ctx);
           self.menu.bottom_menu_bar(ui, ctx);
           self.menu.content_view(ui, ctx);
       });
   }
}
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../BMHANNAAir_ttf.ttf"
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
fn main() {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        resizable:false,
        fullscreen:true,
        // min_window_size: Some(egui::vec2(850.0, 800.0)),
        // initial_window_size: Some(egui::vec2(850.0, 800.0)),
        //MSAA(Multisampling Anti-Aliasing) 수준을 설정합니다.
        //2의 거듭제곱이어야 합니다. 높음 = 더 부드러운 3D.
        // multisampling: 8,
        //하드웨어가속
        // hardware_acceleration:HardwareAcceleration::Required,
        //OS테마따르기
        // follow_system_theme: true,
        //종료후에도 실행
        // run_and_return: true,
        // centered:true,
        default_theme:Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "PEF Control Board",
        options, 
        Box::new(|cc| {
            let mut app = PEFApp::new(cc);
            
            
            // let test = app.menu
            {
                let run_temp = app.menu.exec.run.clone();
                let power_temp = app.menu.exec.power.clone();
                let run_time = app.menu.exec.run_time.clone();
                let power_time = app.menu.exec.power_time.clone();
                thread::spawn(move || {
                    loop {
                        thread::sleep(Duration::from_secs(1));
                        if run_temp.load(Ordering::Relaxed){
                            *run_time.lock().unwrap()+=1;
                        }
                        if power_temp.load(Ordering::Relaxed){
                            *power_time.lock().unwrap()+=1;
                        } 
                        
                        // let mut test = 1;
                        // test +=1; 
                        // let mut num = time.load(Ordering::Relaxed);
                        // num +=1;
                        // time.store(num, Ordering::Relaxed);
                    }
                });
            }
            Box::new(app)
        }));
}
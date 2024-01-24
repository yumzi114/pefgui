#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread, time::Duration, sync::{Arc, Mutex}};
use serde_derive::{Serialize, Deserialize};
use eframe::{egui::{self, ViewportBuilder}, Theme};
mod interface;
use crossbeam_channel::{unbounded,Receiver,Sender};
use interface::{UserUi,keypad::keypad_view};
use pefapi::{{ChageList},device::{PulseInfo,VolatageInfo}, LineCodec, RequestData, RequestDataList};
use tokio::runtime::Runtime;
use tokio_serial::{StopBits, SerialPortBuilderExt, SerialPort};
use tokio_util::codec::Decoder;
use futures::{ StreamExt, SinkExt};



#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";
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
            //UI상태변경용 스레드
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
            //시리얼 통신을 위한 스레드
            let recv = app.app_receiver.clone();
            let respone_mem= app.response.clone();
            
            thread::spawn(move||{
                let rt  = Runtime::new().unwrap();
                rt.block_on(async {
                    let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
                    #[cfg(unix)]
                    port.set_stop_bits(StopBits::One).unwrap();
                    port.set_exclusive(false)
                        .expect("Unable to set serial port exclusive to false");
                    let (mut tx, mut rx) =LineCodec.framed(port).split();
                    loop {
                        thread::sleep(Duration::from_millis(1));
                        if let Ok(mut req_data)=recv.try_recv(){
                            req_data.getter();
                            req_data.checksum();
                            let list = req_data.to_list();
                            tx.send(list).await.unwrap();
                        }
                        // if let Some(data)=rx.next().await{
                        //     *mem.lock().unwrap()+=1;
                        //     // let dd = data; 
                        // }
                    }
                });
            });
            // let mem = app.thread_time.clone();
            let respone_mem= app.response.clone();
            thread::spawn(move||{
                let test  = Runtime::new().unwrap();
                test.block_on(async {
                    let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
                    #[cfg(unix)]
                    port.set_stop_bits(StopBits::One).unwrap();
                    port.set_exclusive(false)
                        .expect("Unable to set serial port exclusive to false");
                    let mut reader =LineCodec.framed(port);
                    loop{
                        thread::sleep(Duration::from_millis(1));
                        // *mem.lock().unwrap()+=1;
                        if let Some(line_result)=reader.next().await {
                            // *mem.lock().unwrap()+=1;
                            if let Ok(datalist)=line_result{
                                let mut responese_data = RequestData::default();
                                *respone_mem.lock().unwrap()=responese_data.parser(&datalist).unwrap();
                            }
                            // let ddd=line_result.unwrap();
                        }
                    }
                    // while let Some(line_result)=reader.next().await {
                    //     let ddd=line_result.unwrap();
                    // }
                });
                    
            });
            
            Box::<PEFApp>::new(app)
        }),
    )
}


#[derive(Clone)]
struct PEFApp {
    mainui:UserUi,
    voltage:VolatageInfo,
    PulseInfo:PulseInfo,
    thread_time:Arc<Mutex<usize>>,
    request:RequestData,
    //스레드로 데이터전송을위한 앱채널
    app_sender:Sender<RequestData>,
    app_receiver:Receiver<RequestData>,
    response:Arc<Mutex<Vec<RequestDataList>>>,
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        //시작시 기본설정으로 설정파일을 생성하고 읽어들여서 구조체생성 ~/.config/pefapp/{pulse.toml, vol.toml}
        confy::store("pefapp", "pulse", PulseInfo::default()).unwrap();
        confy::store("pefapp", "vol", VolatageInfo::default()).unwrap();
        let voltage = confy::load("pefapp", "vol").unwrap_or_default();
        let PulseInfo = confy::load("pefapp", "pulse").unwrap_or_default();
        let thread_time = Arc::new(Mutex::new(1));
        let request = RequestData::default();
        let (tx, rx) = unbounded();
        let mut response=RequestData::default().to_list();
        let respon_data = Arc::new(Mutex::new(response));
        Self{
            mainui:UserUi::default(),
            voltage,
            PulseInfo,
            thread_time,
            request,
            app_sender:tx,
            app_receiver:rx,
            response:respon_data
        }
    }
}

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // 인터페이스의 정의된 메서드실행 구문
            self.mainui.head_view(ui, ctx);
            self.mainui.content_view(ui, ctx,&mut self.PulseInfo,&mut self.voltage,&mut self.request,&mut self.app_sender,&mut self.response);
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

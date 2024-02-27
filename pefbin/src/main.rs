#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread, time::Duration, sync::{Arc, Mutex}};
use chrono::{DateTime, Local, TimeZone};
use defaults::Defaults;
use serde_derive::{Serialize, Deserialize};
use eframe::{egui::{self, ViewportBuilder}, Theme};
mod interface;
mod applog;
use crossbeam_channel::{unbounded,Receiver,Sender};
use interface::{UserUi,keypad::keypad_view};
use pefapi::{{ChageList},device::{PulseInfo,VolatageInfo, AppState}, LineCodec, RequestData, RequestDataList};
use tokio::{runtime::Runtime, time};
use tokio_serial::{StopBits, SerialPortBuilderExt, SerialPort};
use tokio_util::codec::Decoder;
use futures::{ StreamExt, SinkExt};

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

const TRIGGER_FILE_SIZE: u64 = 10 * 1024;
const LOG_FILE_COUNT: u32 = 10;
const FILE_PATH: &str = "/tmp/test.log";
const ARCHIVE_PATTERN: &str = "/tmp/response/data.{}.log";

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";
// const DEFAULT_TTY: &str = "/dev/ttyAMA0";
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
            //작업시간타이머
            let app_state_mem = app.app_state.clone();
            let app_state_rec = app.state_receiver.clone();
            thread::spawn(move||{
                loop{
                    if let Ok(mut app_state)=app_state_rec.try_recv(){
                        let mut time:usize = 0;
                        if app_state.limit_time!=0||app_state.set_time!=0{
                            thread::sleep(Duration::from_secs(1));
                            time+=1;
                            if time ==60{
                                app_state.limit_time-=1;
                                (*app_state_mem.lock().unwrap())=app_state;
                                time=0;
                            }else {
                                continue;
                            }
                        }
                    }
                    if (*app_state_mem.lock().unwrap()).limit_time!=0{
                        let num = (*app_state_mem.lock().unwrap()).limit_time;
                        
                        match  num.checked_sub(1){
                            Some(time)=>{
                                (*app_state_mem.lock().unwrap()).limit_time=time;
                                // if time==0{
                                //     (*app_state_mem.lock().unwrap()).limit_time=time;
                                // }
                            },
                            _=>{
                                (*app_state_mem.lock().unwrap()).set_time=0;
                                (*app_state_mem.lock().unwrap()).limit_time=0;
                                // num=0;
                                // (*app_state_mem.lock().unwrap()).limit_time=num;
                                // (*app_state_mem.lock().unwrap()).set_time=num;
                            }
                        }
                        // if let Some(time)=num.checked_sub(1){
                        //     (*app_state_mem.lock().unwrap()).limit_time=num-1;
                        // }
                    }
                }
            });
            //시리얼 통신을 위한 스레드
            let recv = app.app_receiver.clone();
            // let respone_mem= app.response.clone();
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
                    }
                });
            });
            let respone_mem= app.response.clone();
            let err_type = app.err_type.clone();
            let sys_time_mem = app.sys_time.clone();
            let _handle: log4rs::Handle = log4rs::init_config(logconfig((*sys_time_mem.lock().unwrap()).clone())).unwrap();
            
            thread::spawn(move||{
                let test  = Runtime::new().unwrap();
                test.block_on(async {
                    let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
                    #[cfg(unix)]
                    port.set_stop_bits(StopBits::One).unwrap();
                    port.set_exclusive(false)
                        .expect("Unable to set serial port exclusive to false");
                    let mut reader =LineCodec.framed(port);
                    let time = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
                    loop{
                        if *sys_time_mem.lock().unwrap()!=time{
                            _handle.set_config(logconfig(time.clone()));
                        }
                        thread::sleep(Duration::from_millis(1));
                        //데이터 수신확인
                        if let Some(line_result)=reader.next().await {
                            //통신버퍼를 확인
                            if let Ok(datalist)=line_result{
                                let mut responese_data = RequestData::default();
                                //데이터 파싱확인
                                if let Ok(req_data)=responese_data.parser(&datalist){
                                    let ttt = format!("{:?}",datalist);
                                    info!("{}",ttt);
                                    *respone_mem.lock().unwrap()=req_data;
                                    //체크섬과 에러타입을 화인
                                    if let Err(e)=responese_data.check_all(){
                                        match &e[..] {
                                            "Over Limit"=>{
                                                *err_type.lock().unwrap()=ErrorList::OverLimit;
                                            },
                                            "Non Response"=>{
                                                *err_type.lock().unwrap()=ErrorList::NonResponse;
                                            },
                                            "CRC Error"=>{
                                                *err_type.lock().unwrap()=ErrorList::CRCError;
                                            },
                                            "Fail checksum Err"=>{
                                                *err_type.lock().unwrap()=ErrorList::CheckSumErr;
                                            },
                                            _=>{}
                                        }
                                    }
                                    else {
                                        *err_type.lock().unwrap()=ErrorList::None;
                                    }
                                }
                            }
                        }
                    }
                });
                    
            });
            Box::<PEFApp>::new(app)
        }),
    )
}
#[derive(Clone,Default)]
pub enum ErrorList{
    CheckSumErr,
    ResponesErr,
    OverLimit,
    NonResponse,
    CRCError,
    #[default]
    None
}
#[derive(Clone)]
struct PEFApp {
    mainui:UserUi,
    voltage:VolatageInfo,
    PulseInfo:PulseInfo,
    thread_time:Arc<Mutex<usize>>,
    // run_time:Arc<Mutex<Option<u16>>>,
    request:RequestData,
    //송신스레드로 데이터전송을위한 앱채널
    app_sender:Sender<RequestData>,
    app_receiver:Receiver<RequestData>,
    app_state:Arc<Mutex<AppState>>,
    state_sender:Sender<AppState>,
    state_receiver:Receiver<AppState>,
    response:Arc<Mutex<Vec<RequestDataList>>>,
    err_type:Arc<Mutex<ErrorList>>,
    sys_time:Arc<Mutex<String>>,
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        //시작시 기본설정으로 설정파일을 생성하고 읽어들여서 구조체생성 ~/.config/pefapp/{pulse.toml, vol.toml}
        confy::store("pefapp", "pulse", PulseInfo::default()).unwrap();
        confy::store("pefapp", "vol", VolatageInfo::default()).unwrap();
        confy::store("pefapp", "appstate", AppState::default()).unwrap();
        let voltage = confy::load("pefapp", "vol").unwrap_or_default();
        let PulseInfo = confy::load("pefapp", "pulse").unwrap_or_default();
        let app_state = Arc::new(Mutex::new(confy::load("pefapp", "appstate").unwrap_or_default()));
        let thread_time = Arc::new(Mutex::new(1));
        let request = RequestData::default();
        let (tx, rx) = unbounded();
        let (state_sender, state_receiver) = unbounded();
        let response=RequestData::default().to_list();
        let respon_data = Arc::new(Mutex::new(response));
        let err_type = Arc::new(Mutex::new(ErrorList::default()));
        let time = chrono::offset::Local::now().format("%Y-%m-%d");
        let sys_time = Arc::new(Mutex::new(format!("{}",time)));
        Self{
            mainui:UserUi::default(),
            voltage,
            PulseInfo,
            app_state,
            thread_time,
            request,
            app_sender:tx,
            app_receiver:rx,
            response:respon_data,
            state_sender,
            state_receiver,
            err_type,
            sys_time
        }
    }
}

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // 인터페이스의 정의된 메서드실행 구문
            self.mainui.head_view(ui, ctx);
            self.mainui.content_view(ui, 
                ctx,
                &mut self.PulseInfo,
                &mut self.voltage,
                &mut self.request,
                &mut self.app_sender,
                &mut self.response,
                &self.sys_time,
                &mut self.app_state,
            );
            self.mainui.bottom_view(ui, ctx,&self.thread_time,&self.err_type);
        });
    }
}
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../files/Pilseung_Gothic.ttf"
            // "../files/FULLDOZER Extra Bold.ttf"
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

fn logconfig(date:String)->Config{
    let mut pattern =format!("/tmp/response/{}.",date);
    pattern.push_str("{}.log");
    let level = log::LevelFilter::Info;
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build(pattern.as_str(), LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(FILE_PATH, Box::new(policy))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .unwrap();
    config
}
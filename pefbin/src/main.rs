#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{net::TcpStream, sync::{Arc, Mutex}, thread, time::Duration};
use eframe::{egui::{self, Pos2, Rounding, Sense, Vec2, ViewportBuilder}, Theme};
mod interface;
mod applog;
mod component;
mod app_error;
mod app_threads;
use app_error::ErrorList;
use component::{setup_custom_fonts, warring_window};
use app_threads::{ui_timer,run_timer,serial_receiver,serial_sender,socket_sender};
use crossbeam_channel::{unbounded,Receiver,Sender};
use interface::{UserUi,keypad::keypad_view};
use pefapi::{device::{PulseInfo,VolatageInfo, AppState}, LineCodec, RequestData, RequestDataList};
use tokio::{runtime::Runtime};
use tokio_serial::{StopBits, SerialPortBuilderExt, SerialPort};
use tokio_util::codec::Decoder;
use futures::{ StreamExt, SinkExt};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;

// use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";

const SOCKET_URL: &'static str = "wss://yumi.town/socket";
// const SOCKET_URL: &'static str = "ws://192.168.0.10:8080/socket";
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
            let mem: Arc<Mutex<usize>> = app.thread_time.clone();
            let socket = app.socket.clone();
            //UI상태변경용 스레드/소켓핑
            ui_timer(socket,mem);
            //작업시간타이머
            let app_state_mem: Arc<Mutex<AppState>> = app.app_state.clone();
            run_timer(app_state_mem);
            //시리얼 통신(송신)을 위한 스레드
            let recv: Receiver<RequestData> = app.app_receiver.clone();
            // let respone_mem= app.response.clone();
            serial_sender(recv);
            let respone_mem: Arc<Mutex<Vec<RequestDataList>>>= app.response.clone();
            let err_type: Arc<Mutex<ErrorList>> = app.err_type.clone();
            // let sys_time_mem: Arc<Mutex<String>> = app.sys_time.clone();
            // let _handle: log4rs::Handle = log4rs::init_config(logconfig((*sys_time_mem.lock().unwrap()).clone())).unwrap();
            serial_receiver(respone_mem,err_type);
            let state_mem= app.app_state.clone();
            let respone_mem= app.response.clone();
            // let socket_onoff=app.socket_onoff.clone();
            let socket = app.socket.clone();
            socket_sender(socket,state_mem,respone_mem);
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
    // run_time:Arc<Mutex<Option<u16>>>,
    request:RequestData,
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    // socket_onoff:bool,
    //송신스레드로 데이터전송을위한 앱채널
    app_sender:Sender<RequestData>,
    app_receiver:Receiver<RequestData>,
    app_state:Arc<Mutex<AppState>>,
    response:Arc<Mutex<Vec<RequestDataList>>>,
    err_type:Arc<Mutex<ErrorList>>,
    sys_time:Arc<Mutex<String>>,
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        //소켓설정
        let socket_mem=
        if let Ok((mut socket,res))=connect(Url::parse(SOCKET_URL).unwrap()){
            Arc::new(Mutex::new(Some(socket)))
        }else{
            Arc::new(Mutex::new(None))
        };
        // let (mut socket, resp) =
        // connect(Url::parse(SOCKET_URL).unwrap()).expect("Can't connect");
        // let socket_mem = Arc::new(Mutex::new(socket));
        //시작시 기본설정으로 설정파일을 생성하고 읽어들여서 구조체생성 ~/.config/pefapp/{pulse.toml, vol.toml}
        confy::store("pefapp", "pulse", PulseInfo::default()).unwrap();
        confy::store("pefapp", "vol", VolatageInfo::default()).unwrap();
        confy::store("pefapp", "appstate", AppState::default()).unwrap();
        let voltage = confy::load("pefapp", "vol").unwrap_or_default();
        let PulseInfo = confy::load("pefapp", "pulse").unwrap_or_default();
        let app_state = Arc::new(Mutex::new(confy::load("pefapp", "appstate").unwrap_or_default()));
        let thread_time = Arc::new(Mutex::new(1));
        let request = RequestData::default();
        let socket_req = Arc::new(Mutex::new(request));
        let (tx, rx) = unbounded();
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
            err_type,
            socket:socket_mem,
            sys_time,
        }
    }
    //임시 네트워크 갱신 함수
    fn update_net(&mut self){
        if let None =(*self.socket.lock().unwrap()).as_mut(){
            if let Ok((mut socket,res))=connect(Url::parse(SOCKET_URL).unwrap()){
                // (*self.socket.lock().unwrap())=Some(socket);
                // let asd=self.socket.lock();
                // drop(self.socket.lock().unwrap());
                let mut lock = self.socket.try_lock();
                if let Ok(ref mut mutex)=lock{
                    **mutex=Some(socket);
                }
                // (*self.socket.try_lock().unwrap()) = Some(socket);
                
            }else{
                // (*self.socket.lock().unwrap())=None;
            };
        }
        // if let Some(sender)=(*socket.lock().unwrap()).as_mut(){
        //     (*sender).send(Message::Text(name)).unwrap();
        // }
    }
}

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        //경고창 활성화부분
        let rect = ctx.screen_rect();
        let center_rect = Pos2{
            x:(rect.max.x/2.0)-450.0,
            y:(rect.max.y/2.0)-250.0,
        };
        //임시 네트워크 갱신
        // self.update_net();
        //경고윈도우창
        warring_window(center_rect,ctx,&mut self.mainui);
        // if let None =(*self.socket.lock().unwrap()).as_mut(){
        //     if let Ok((mut socket,res))=connect(Url::parse(SOCKET_URL).unwrap()){
        //         (*self.socket.lock().unwrap())=Some(socket);
        //     }else{
        //         // (*self.socket.lock().unwrap())=None;
        //     };
        // }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // 인터페이스의 정의된 메서드실행 구문
            // if let Some(asd)=(*self.socket.lock().unwrap()).as_mut(){
            //     ui.label("연결됨");
            // }
            
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
            self.mainui.bottom_view(ui, ctx,&self.thread_time,&self.err_type,&mut self.app_state);
        });
    }
}


// fn logconfig(date:String)->Config{
//     let mut pattern =format!("/tmp/response/{}.",date);
//     pattern.push_str("{}.log");
//     let level = log::LevelFilter::Info;
//     let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
//     let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
//     let roller = FixedWindowRoller::builder()
//         .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
//         .build(pattern.as_str(), LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
//         .unwrap();
//     let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
//     let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
//         // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
//         .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
//         .build(FILE_PATH, Box::new(policy))
//         .unwrap();
//     let config = Config::builder()
//         .appender(Appender::builder().build("logfile", Box::new(logfile)))
//         .appender(
//             Appender::builder()
//                 .filter(Box::new(ThresholdFilter::new(level)))
//                 .build("stderr", Box::new(stderr)),
//         )
//         .build(
//             Root::builder()
//                 .appender("logfile")
//                 .appender("stderr")
//                 .build(LevelFilter::Info),
//         )
//         .unwrap();
//     config
// }
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{net::TcpStream, sync::{Arc, Mutex}, thread, time::Duration};
use eframe::{egui::{self, Pos2, Rounding, Sense, Vec2, ViewportBuilder}, Theme};
mod interface;
mod applog;
mod component;

mod app_threads;

use pefapi::app_error::ErrorList;
use component::{setup_custom_fonts, warring_window};
use app_threads::{ui_timer,run_timer,serial_receiver,serial_sender,socket_sender,keypad_timer,mqtt_sender,job_timer};
use crossbeam_channel::{unbounded,Receiver,Sender};
use interface::{UserUi,keypad::keypad_view};
use pefapi::{device::{PulseInfo,VolatageInfo, AppState}, LineCodec, RequestData, RequestDataList};
use tokio::{runtime::Runtime};
use tokio_serial::{StopBits, SerialPortBuilderExt, SerialPort};
use tokio_util::codec::Decoder;
use futures::{ StreamExt, SinkExt};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;
use thread_timer::ThreadTimer;
// use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

#[cfg(unix)]
const DEFAULT_TTY: &'static str  = env!("DEFAULT_TTY");
// const DEFAULT_TTY: &'static str="/dev/ttyAMA3";
const SOCKET_URL: &'static str=env!("SOCKET_URL");
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
            let timer_t = app.timer.clone();
            let timer_receiver: Receiver<usize> = app.timer_receiver.clone();
            let time_sender= app.time_sender.clone();
            run_timer(timer_t,timer_receiver,time_sender);
            let pad_timer = app.keypad_timer.clone();
            let k_timer_receiver = app.k_timer_receiver.clone();
            let k_time_sender = app.k_time_sender.clone();
            keypad_timer(pad_timer,k_timer_receiver,k_time_sender);
            let app_state = app.app_state.clone();
            job_timer(app_state);
            
            //시리얼 통신(송신)을 위한 스레드
            let recv: Receiver<RequestData> = app.app_receiver.clone();
            // let respone_mem= app.response.clone();
            serial_sender(recv);
            let respone_mem: Arc<Mutex<Vec<RequestDataList>>>= app.response.clone();
            let report_mem: Arc<Mutex<Vec<RequestDataList>>>= app.report.clone();
            let err_report_mem =app.err_report.clone();
            let err_type: Arc<Mutex<ErrorList>> = app.err_type.clone();
            let rep_err_type: Arc<Mutex<ErrorList>> = app.repo_err_type.clone();
            // let sys_time_mem: Arc<Mutex<String>> = app.sys_time.clone();
            // let _handle: log4rs::Handle = log4rs::init_config(logconfig((*sys_time_mem.lock().unwrap()).clone())).unwrap();
            serial_receiver(respone_mem,report_mem,err_report_mem,err_type,rep_err_type);
            let state_mem= app.app_state.clone();
            let report= app.report.clone();
            // let socket_onoff=app.socket_onoff.clone();
            let socket = app.socket.clone();
            socket_sender(socket,state_mem,report);
            let report= app.report.clone();
            mqtt_sender(report);
            Box::<PEFApp>::new(app)
        }),
    )
}

#[derive(Clone)]
struct PEFApp {
    mainui:UserUi,
    voltage:VolatageInfo,
    pulse_Info:PulseInfo,
    thread_time:Arc<Mutex<usize>>,
    //타이머
    
    //작업시간 타이머 스레드,채널
    timer:ThreadTimer,
    timer_sender:Sender<usize>,
    timer_receiver:Receiver<usize>,
    time_sender:Sender<usize>,
    time_receiver:Receiver<usize>,
    //키패드 타이머 스레드,채널
    keypad_timer:ThreadTimer,
    k_time:u8,
    k_timer_sender:Sender<u8>,
    k_timer_receiver:Receiver<u8>,
    k_time_sender:Sender<u8>,
    k_time_receiver:Receiver<u8>,
    // run_time:Arc<Mutex<Option<u16>>>,
    request:RequestData,
    
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    // socket_onoff:bool,
    //송신스레드로 데이터전송을위한 앱채널
    app_sender:Sender<RequestData>,
    app_receiver:Receiver<RequestData>,
    app_state:Arc<Mutex<AppState>>,
    response:Arc<Mutex<Vec<RequestDataList>>>,
    report:Arc<Mutex<Vec<RequestDataList>>>,
    err_report:Arc<Mutex<Vec<RequestDataList>>>,
    err_type:Arc<Mutex<ErrorList>>,
    repo_err_type:Arc<Mutex<ErrorList>>,
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
        //시작시 기본설정으로 설정파일을 생성하고 읽어들여서 구조체생성 ~/.config/pefapp/{pulse.toml, vol.toml}
        confy::store("pefapp", "pulse", PulseInfo::default()).unwrap();
        confy::store("pefapp", "vol", VolatageInfo::default()).unwrap();
        confy::store("pefapp", "appstate", AppState::default()).unwrap();
        let voltage = confy::load("pefapp", "vol").unwrap_or_default();
        let mut pulse_Info: PulseInfo = confy::load("pefapp", "pulse").unwrap_or_default();
        pulse_Info.max_value_change();
        let app_state = Arc::new(Mutex::new(confy::load("pefapp", "appstate").unwrap_or_default()));
        let thread_time = Arc::new(Mutex::new(1));
        let request = RequestData::default();
        let (tx, rx) = unbounded();
        let response=RequestData::default().to_list();
        let report = RequestData::default().to_list();
        let err_report = RequestData::default().to_list();
        let respon_data = Arc::new(Mutex::new(response));
        let report = Arc::new(Mutex::new(report));
        let err_report=Arc::new(Mutex::new(err_report));
        let err_type = Arc::new(Mutex::new(ErrorList::default()));
        let repo_err_type= Arc::new(Mutex::new(ErrorList::default()));
        //작업시간
        let timer = ThreadTimer::new();
        let (timer_sender,timer_receiver)=unbounded();
        let (time_sender,time_receiver)=unbounded();
        let (k_timer_sender,k_timer_receiver)=unbounded();
        let (k_time_sender,k_time_receiver)=unbounded();
        //키패드
        let keypad_timer = ThreadTimer::new();
        
        Self{
            mainui:UserUi::default(),
            voltage,
            pulse_Info,
            app_state,
            thread_time,
            timer,
            timer_sender,
            timer_receiver,
            time_sender,
            time_receiver,
            keypad_timer,
            k_time:0,
            k_timer_sender,
            k_timer_receiver,
            k_time_sender,
            k_time_receiver,
            request,
            report,
            err_report,
            app_sender:tx,
            app_receiver:rx,
            response:respon_data,
            err_type,
            repo_err_type,
            socket:socket_mem,
        }
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
        //타임어택 -1
        if let Ok(data)=self.time_receiver.try_recv(){
            self.app_state.lock().unwrap().limit_time=data as u16;
            if data!=0{
                self.timer_sender.send(data).unwrap();
            }
        }
        //키패드타이머
        if let Ok(num)=self.k_time_receiver.try_recv(){
            self.k_time=num;
            if num ==0&&!self.mainui.warning_pop{
                self.mainui.keypad.popon=false;
                self.mainui.keypad.sellist=None;
            }
            else if num !=0{
                self.k_timer_sender.send(num).unwrap();
            }
        }
        //경고윈도우창
        warring_window(center_rect,ctx,&mut self.mainui);
  
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mainui.head_view(ui, ctx);
            self.mainui.content_view(ui, 
                ctx,
                &mut self.pulse_Info,
                &mut self.voltage,
                &mut self.request,
                &mut self.app_sender,
                &self.response,
                &self.report,
                &self.err_report,
                &self.repo_err_type,
                &mut self.app_state,
                &mut self.timer_sender,
                &mut self.k_timer_sender,
            );
            self.mainui.bottom_view(ui, ctx,&self.thread_time,&self.err_type,&mut self.app_state);
        });
    }
}


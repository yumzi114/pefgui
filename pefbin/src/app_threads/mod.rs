use core::time;
use std::{io::ErrorKind, net::TcpStream, sync::{Arc, Mutex}, thread::{self, sleep}, time::{Duration, Instant, SystemTime}};
use futures::{SinkExt, StreamExt};
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;
use pefapi::{device::AppState, LineCodec, RequestData, RequestDataList};
use crossbeam_channel::{unbounded,Receiver,Sender};
use tokio::runtime::Runtime;
use tokio_serial::{SerialPort, SerialPortBuilderExt, StopBits};
use tokio_util::codec::Decoder;
use url::Url;
use mosquitto_rs::*;
use std::sync::mpsc::channel;
use futures_timer::Delay;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use thread_timer::ThreadTimer;
use pefapi::app_error::ErrorList;
// use crate::app_error::ErrorList;



#[cfg(unix)]
const DEFAULT_TTY: &'static str = env!("DEFAULT_TTY");
const MQTT_URL: &'static str = env!("MQTT_URL");
const MQTT_PORT: &'static str = env!("MQTT_PORT");
const SERIAL_NUMBER: &'static str = env!("SERIAL_NUMBER");
const MQTT_USER: &'static str = env!("MQTT_USER");
const MQTT_PSW: &'static str = env!("MQTT_PSW");
const MQTT_TOPIT: &'static str = env!("MQTT_TOPIT");


//UI상태변경 스레드/소켓핑
pub fn ui_timer(
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    // socket_onoff:bool,
    mem:Arc<Mutex<usize>>
){
    // (*socket.lock().unwrap()).
    thread::spawn(move||{
        loop{
            thread::sleep(Duration::from_secs(1));
            if let Some(sender)=(*socket.lock().unwrap()).as_mut(){
                (*sender).send(Message::Ping(vec![1_u8])).unwrap();
            }
            if *mem.lock().unwrap()<5{
                *mem.lock().unwrap()+=1;
            }
            else {
                *mem.lock().unwrap()=0;
            }
        }
    });
}

pub fn run_timer(
    timer:ThreadTimer,
    timer_receiver:Receiver<usize>,
    time_sender:Sender<usize>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop{
                thread::sleep(Duration::from_micros(1));
                if let Ok(data)=timer_receiver.try_recv(){
                    let time_sender = time_sender.clone();
                    if timer.cancel().is_ok(){
                        timer.start(Duration::from_secs(60), move || {
                            time_sender.send(data-1).unwrap();
                         }).unwrap();
                    }
                    else{
                        let time_sender = time_sender.clone();
                        timer.start(Duration::from_secs(60), move || {
                            time_sender.send(data-1).unwrap();
                        }).unwrap();
                    }
                }
            }
        });
    });
}
pub fn job_timer(
    app_state:Arc<Mutex<AppState>>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop{
                (*app_state.lock().unwrap()).job_time=0;
                thread::sleep(Duration::from_micros(1));
                while (*app_state.lock().unwrap()).job_time_bool {
                    thread::sleep(Duration::from_millis(1));
                    (*app_state.lock().unwrap()).job_time+=1;
                }
            }
        });
    });

}
pub fn keypad_timer(
    pad_timer:ThreadTimer,
    k_timer_receiver:Receiver<u8>,
    k_time_sender:Sender<u8>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop{
                thread::sleep(Duration::from_micros(1));
                if let Ok(data)=k_timer_receiver.try_recv(){
                    let k_time_sender = k_time_sender.clone();
                    if pad_timer.cancel().is_ok(){
                        pad_timer.start(Duration::from_secs(1), move || {
                            if data!=0{
                                k_time_sender.send(data-1).unwrap();
                            }
                            else{
                                k_time_sender.send(0).unwrap();
                            }
                         }).unwrap();
                    }
                    else{
                        let k_time_sender = k_time_sender.clone();
                        pad_timer.start(Duration::from_secs(1), move || {
                            if data!=0{
                                k_time_sender.send(data-1).unwrap();
                            }
                            else{
                                k_time_sender.send(0).unwrap();
                            }
                        }).unwrap();
                    }
                    
                }
            }
        });
    });
}



// 시리얼송신 스레드
pub fn serial_sender(recv: Receiver<RequestData>){
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
                    println!("-------SENDDATA-----");
                    req_data.is_checksum().unwrap();
                    let list = req_data.to_list();
                    tx.send(list).await.unwrap();
                }
            }
        });
    });
}

//시리얼수신 스레드
pub fn serial_receiver(
    respone_mem: Arc<Mutex<Vec<RequestDataList>>>,
    report_mem: Arc<Mutex<Vec<RequestDataList>>>,
    err_report_mem: Arc<Mutex<Vec<RequestDataList>>>,
    err_type: Arc<Mutex<ErrorList>>,
    repo_err_type: Arc<Mutex<ErrorList>>,
){
    thread::spawn(move||{
        let task  = Runtime::new().unwrap();
        task.block_on(async {
            let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
            #[cfg(unix)]
            port.set_stop_bits(StopBits::One).unwrap();
            port.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");
            let mut reader =LineCodec.framed(port);
            loop{
                
                thread::sleep(Duration::from_millis(1));
                //데이터 수신확인
                if let Some(line_result)=reader.next().await {
                    //통신버퍼를 확인
                    match line_result{
                        Ok(datalist)=>{
                            let mut responese_data = RequestData::default();
                            //데이터 파싱확인
                            if let Ok(req_data)=responese_data.parser(&datalist){
                                match responese_data.check_all(err_type.clone(),repo_err_type.clone()) {
                                    Ok(command)=>{
                                        if command==0x03{
                                            let dd =  RequestData::default();
                                            let clear = dd.to_list();
                                            *err_report_mem.lock().unwrap()=clear;
                                            *report_mem.lock().unwrap()=req_data;
                                        }
                                        else if command==0x02{
                                            *respone_mem.lock().unwrap()=req_data;
                                        }
                                    },
                                    Err(command)=>{
                                        if command==0x03{
                                            let dd =  RequestData::default();
                                            let clear = dd.to_list();
                                            *report_mem.lock().unwrap()=clear;
                                            *err_report_mem.lock().unwrap()=req_data;
                                        }
                                        else if command==0x02{
                                            *respone_mem.lock().unwrap()=req_data;
                                        }
                                    }
                                }
                            }
                        }
                        Err(error)=>{
                            match error.kind() {
                                ErrorKind::Other=>{
                                    *err_type.lock().unwrap()=ErrorList::DeviceSNErr;
                                    let dd =  RequestData::default();
                                    let clear = dd.to_list();
                                    *report_mem.lock().unwrap()=clear;
                                    continue;
                                }
                                ErrorKind::NotFound=>{  
                                    *err_type.lock().unwrap()=ErrorList::StandByMode;
                                    let dd =  RequestData::default();
                                    let clear = dd.to_list();
                                    *report_mem.lock().unwrap()=clear;
                                    continue;
                                }
                                _=>{
                                    *err_type.lock().unwrap()=ErrorList::BoardErr;
                                    let dd =  RequestData::default();
                                    let clear = dd.to_list();
                                    *report_mem.lock().unwrap()=clear;
                                    continue;
                                }
                            }

                        }
                    }
                }
            }
        });
            
    });
}

pub fn socket_sender(
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    app_state:Arc<Mutex<AppState>>,
    response:Arc<Mutex<Vec<RequestDataList>>>){
    let mem = response.clone();
 
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
            rt.block_on(async {
                loop{
                    let mut payload =String::new();
                    let list = (*response.lock().unwrap()).clone();
                    let asd =[1.2];
                    for (num,i) in list.iter().enumerate(){
                        if let Ok(data)=i.to_paylod(){
                            if !payload.is_empty(){
                                payload.push_str(",");                                    
                            }
                            payload.push_str(&data[..]);
                        }
                    }
                    if let Some(sender)=(*socket.lock().unwrap()).as_mut(){
                        (*sender).send(Message::Text(payload)).unwrap();
                    }

                    sleep(Duration::new(1, 0));
                        continue;
        
                }
                
            });
    });
}


pub fn mqtt_sender(
    response:Arc<Mutex<Vec<RequestDataList>>>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop{
                thread::sleep(Duration::from_secs(1));
                let client: Client = Client::with_id(SERIAL_NUMBER, true).unwrap();
                client.set_username_and_password(Some(MQTT_USER), Some(MQTT_PSW)).unwrap();
                
                let rc = client.connect(
                    MQTT_URL, i32::from_str_radix(MQTT_PORT, 10).unwrap(),
                                std::time::Duration::from_secs(5), None).await;
                while let Ok(_)=rc{
                    thread::sleep(Duration::from_secs(1));
                    let mut payload =String::new();
                    let list = (*response.lock().unwrap()).clone();
                    let asd =[1.2];
                    for (num,i) in list.iter().enumerate(){
                        if let Ok(data)=i.to_paylod(){
                            if !payload.is_empty(){
                                payload.push_str(",");                                    
                            }
                            payload.push_str(&data[..]);
                        }
                    }
                    let topic = format!("{}/{}",MQTT_TOPIT,list[2].to_paylod().unwrap());
                    let result = client.publish(topic, payload.as_bytes(), QoS::AtMostOnce, false).await;
                    if let Ok(_)=result{
                        continue;
                    }else{
                        break;
                    }
                }
            }
        });
    });
}
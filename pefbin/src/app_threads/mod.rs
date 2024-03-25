use core::time;
use std::{net::TcpStream, sync::{Arc, Mutex}, thread::{self, sleep}, time::{Duration, Instant, SystemTime}};
use futures::{SinkExt, StreamExt};
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;
use pefapi::{device::AppState, LineCodec, RequestData, RequestDataList};
use crossbeam_channel::{unbounded,Receiver,Sender};
use tokio::runtime::Runtime;
use tokio_serial::{SerialPort, SerialPortBuilderExt, StopBits};
use tokio_util::codec::Decoder;
use url::Url;
use std::sync::mpsc::channel;
use futures_timer::Delay;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use thread_timer::ThreadTimer;
use pefapi::app_error::ErrorList;
// use crate::app_error::ErrorList;



#[cfg(unix)]
const DEFAULT_TTY: &'static str = env!("DEFAULT_TTY");

// const SOCKET_URL: &'static str = "wss://yumi.town/socket";
// const SOCKET_URL: &'static str = "ws://192.168.0.10:8080/socket";

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
            
            // if let Some(sender)=socket{
            //     (*sender.lock().unwrap()).send(Message::Ping(vec![1_u8])).unwrap();
            // }
            // (*socket.lock().unwrap()).send(Message::Ping(vec![1_u8])).unwrap();
            // if socket_onoff {
            //     socket.lock().unwrap().as_mut().unwrap().send(Message::Ping(vec![1_u8])).unwrap();
            // }
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
//작업시간 타이머스레드
// pub fn run_timer(
//     app_state_mem: Arc<Mutex<AppState>>,
//     timer:ThreadTimer,
// ){
    
    
//     thread::spawn(move||{
//         timer.start(Duration::from_secs(30), move || { }).unwrap();
//         let rt  = Runtime::new().unwrap();
//         rt.block_on(async {
//             loop{
//                 thread::sleep(Duration::from_millis(1));
//                 if (*app_state_mem.lock().unwrap()).limit_time!=0{
//                     let mut app_state = (*app_state_mem.lock().unwrap()).clone();
//                     sleep(Duration::new(60, 0));
//                     app_state.limit_time-=1;
//                     confy::store("pefapp", "appstate", app_state).unwrap();
//                     *app_state_mem.lock().unwrap()=app_state;
//                 }
//             }
//         });
//     });
// }
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
            // let time = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
            loop{
                // if *sys_time_mem.lock().unwrap()!=time{
                //     _handle.set_config(logconfig(time.clone()));
                // }
                thread::sleep(Duration::from_millis(1));
                //데이터 수신확인
                if let Some(line_result)=reader.next().await {
                    //통신버퍼를 확인
                    if let Ok(datalist)=line_result{
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
                                // _=>{}
                            }
                            // if req_data[4]==RequestDataList::COMMAND(0x03){
                            //     *report_mem.lock().unwrap()=req_data;
                            //     println!("-------REPORT DATA-----");
                            // }
                            // else if req_data[4]==RequestDataList::COMMAND(0x02){
                            //     *respone_mem.lock().unwrap()=req_data;
                            //     println!("-------RESPONE DATA-----");
                            //     if let Err(e)=responese_data.check_all(){
                            //         match &e[..] {
                            //             "Over Limit"=>{
                            //                 *err_type.lock().unwrap()=ErrorList::OverLimit;
                            //             },
                            //             "Non Response"=>{
                            //                 *err_type.lock().unwrap()=ErrorList::NonResponse;
                            //             },
                            //             "CRC Error"=>{
                            //                 *err_type.lock().unwrap()=ErrorList::CRCError;
                            //             },
                            //             "Fail checksum Err"=>{
                            //                 *err_type.lock().unwrap()=ErrorList::CheckSumErr;
                            //             },
                            //             _=>{}
                            //         }
                            //     }
                            //     else {
                            //         *err_type.lock().unwrap()=ErrorList::None;
                            //     }
                            // }
                            
                            
                        }
                    }
                }
            }
        });
            
    });
}
//소켓 송신스레드
pub fn socket_sender(
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    app_state:Arc<Mutex<AppState>>,
    response:Arc<Mutex<Vec<RequestDataList>>>){
    let mem = response.clone();
    // let (mut socket, resp) =
    //     connect(Url::parse(SOCKET_URL).unwrap()).expect("Can't connect");
    // let mem = socket.clone();
    // let mut dd;
    // if let Some(soc_mem)=socket{
    //     dd = soc_mem.clone();
    // }
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
            rt.block_on(async {
                loop{
                    sleep(Duration::from_millis(1));
                    
                    // socket.send(Message::Ping(vec![1_u8])).unwrap();
                    if (*app_state.lock().unwrap()).limit_time!=0{
                        let mut name = String::new();
                        let list =(*response.lock().unwrap()).clone();
                        for i in list {
                            name.push_str(format!("{}",i).as_str());
                        }
                    // if socket_onoff{
                    //     socket.lock().unwrap().as_mut().unwrap().send(Message::Text(name)).unwrap();
                    // }
                    if let Some(sender)=(*socket.lock().unwrap()).as_mut(){
                        (*sender).send(Message::Text(name)).unwrap();
                    }
                    
                    
                    //     sleep(Duration::from_secs(5));
                        // let test = String::from("value");
                        // let tesdd =test.as_bytes();
                        // socket.send(Message::binary(tesdd)).unwrap();
                        // let strt = (*mem.lock().unwrap()).socket_fmt();
                        // let dd = *socket_req.lock().unwrap();
                        // let ddd=dd.to_list()[6];
                        // if let Some(sender)=socket{
                        //     // let ad = sender.clone();
                        //     // sender.lock().unwrap().send
                        //     // (*sender.lock().unwrap()).send(Message::Text(name)).unwrap();
                        // }
                        // (*socket.lock().unwrap()).send(Message::Text(name)).unwrap();
                        // socket.send(Message::Text(name)).unwrap();
                        // let str = (*response.lock().unwrap())
                        sleep(Duration::new(5, 0));
                        continue;
                    }
                    //헤어확인시
                    // for (ref header, _value) in response.headers() {
                    //     let str = format!("{}",header);
                    //     socket.send(Message::Text(str)).unwrap();
                    // }
                }
                
            });
    });
}



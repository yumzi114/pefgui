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
use crate::app_error::ErrorList;



#[cfg(unix)]
const DEFAULT_TTY: &'static str = "/dev/ttyAMA3";

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
pub fn run_timer(
    app_state_mem: Arc<Mutex<AppState>>,
){
    thread::spawn(move||{
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop{
                thread::sleep(Duration::from_millis(1));
                if (*app_state_mem.lock().unwrap()).limit_time!=0{
                    let mut app_state = (*app_state_mem.lock().unwrap()).clone();
                    sleep(Duration::new(60, 0));
                    app_state.limit_time-=1;
                    confy::store("pefapp", "appstate", app_state).unwrap();
                    *app_state_mem.lock().unwrap()=app_state;
                }
            }
        });
    });
}
// pub fn run_timer(
//     app_state_mem: Arc<Mutex<AppState>>,
// ){
//     thread::spawn(move||{
//         let rt  = Runtime::new().unwrap();
//         rt.block_on(async {
//             while  (*app_state_mem.lock().unwrap()).limit_time!=0{
//                 let mut app_state = (*app_state_mem.lock().unwrap()).clone();
//                 sleep(Duration::new(60, 0));
//                 app_state.limit_time-=1;
//                 confy::store("pefapp", "appstate", app_state).unwrap();
//                 *app_state_mem.lock().unwrap()=app_state;
//             }
//         });
//     });
// }



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
    err_type: Arc<Mutex<ErrorList>>
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
            let time = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
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
                            let ttt = format!("{:?}",datalist);
                            // info!("{}",ttt);
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



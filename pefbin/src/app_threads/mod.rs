use core::time;
use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::{Duration, Instant, SystemTime}};
use futures::{SinkExt, StreamExt};
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;
use pefapi::{device::AppState, LineCodec, RequestData, RequestDataList};
use crossbeam_channel::{unbounded,Receiver,Sender};
use tokio::runtime::Runtime;
use tokio_serial::{SerialPort, SerialPortBuilderExt, StopBits};
use tokio_util::codec::Decoder;
use std::sync::mpsc::channel;
use futures_timer::Delay;
use crate::app_error::ErrorList;



#[cfg(unix)]
const DEFAULT_TTY: &'static str = "/dev/ttyAMA3";

const SOCKET_URL: &'static str = "ws://192.168.0.10:8080/socket";

//UI상태변경 스레드
pub fn ui_timer(mem:Arc<Mutex<usize>>){
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
}
//작업시간 타이머스레드
pub fn run_timer(app_state_mem: Arc<Mutex<AppState>>){
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
            
            // loop{
            //     // thread::sleep(Duration::from_nanos(1));
            //     if let Ok(mut app_state)=app_state_rec.try_recv(){
            //         *app_state_mem.lock().unwrap()=app_state;
            //         confy::store("pefapp", "appstate", app_state).unwrap();
            //         if app_state.limit_time!=0||app_state.set_time!=0{
            //             thread::sleep(Duration::from_secs(1));
            //             app_state.limit_time-=1;
            //             (*app_state_mem.lock().unwrap())=app_state;
            //             confy::store("pefapp", "appstate", app_state).unwrap();
            //         }
            //     }
            // }

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
pub fn socket_sender(){

}



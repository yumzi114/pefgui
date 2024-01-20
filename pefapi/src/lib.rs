

use futures::{ StreamExt, SinkExt};
use tokio_util::codec::{Decoder, Encoder};
use tokio_serial::{SerialPortBuilderExt, SerialPort, StopBits};
use bytes::{BytesMut, BufMut};
use std::{io, str,u8};
use serde::{Serialize, Deserialize};
use serde_hex::{SerHex,StrictPfx,CompactPfx};
use defaults::Defaults;


#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";


pub enum ChageList{
    HighVolValue,
    PulseFreq,
    Pulse_ON_OFF_Time,
    HighVol_ON_OFF,
    Pulse_ON_OFF
}

pub struct AppChannel;


impl AppChannel {
    pub fn new()->Self{
        Self{
        }
    }
    #[tokio::main]
    pub async fn tx_send(&mut self, change_value:ChageList)->tokio_serial::Result<()>{
        //변경된 사용자 조작값을 전달하고 매치처리
        let c_value:u16 = match change_value {
            ChageList::HighVolValue=>0b0000_0010_0000_0000,
            ChageList::PulseFreq=>0b1000_0000_0000_0000,
            ChageList::Pulse_ON_OFF_Time=>0b1000_0000_0000_0000,
            ChageList::Pulse_ON_OFF=>0b0001_0000_0000_0000,
            ChageList::HighVol_ON_OFF=>0b0000_0100_0000_0000,
        };
        //리퀘스트 구조체생성
        let mut r_data_test = RequestData::default();
        //변경된 비트값전달하여 리퀘스트값 변경
        r_data_test.into_change_value(c_value);
        //처리할 데이터를 리스트로 받아옴
        let list = r_data_test.to_list();
        // sender.lock().unwrap().send("item".to_string()).await.unwrap();
        //비동기 통신을 위해 스레드실행
        let join_handle = tokio::spawn(async move {
                let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
                #[cfg(unix)]
                port.set_stop_bits(StopBits::One).unwrap();
                port.set_exclusive(false)
                    .expect("Unable to set serial port exclusive to false");
                let (mut tx, mut rx) =LineCodec.framed(port).split();
                tx.send(list).await.unwrap();
        });
        //스레드의 종료대기 종료후 Result Ok를 보냄
        join_handle.await.unwrap();
        Ok(())
    }
}



#[derive(Clone,Copy)]
pub struct LineCodec ;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}
impl Encoder<Vec<RequestDataList>> for LineCodec {
    type Error = io::Error;
    fn encode(&mut self, code:Vec<RequestDataList>, buf: &mut BytesMut) -> Result<(), io::Error> {
        for i in code{
            //리스트의 순서대로 바이트크기별로 데이터전송
            match i {
                RequestDataList::START(data)|
                RequestDataList::LENGHTH(data)|
                RequestDataList::COMMAND(data)|
                RequestDataList::PULSE_ONOFF(data)|
                RequestDataList::HV_ONOFF(data)|
                RequestDataList::OPEN_SENSOR_MONI(data)|
                RequestDataList::CHECKSUM(data)|
                RequestDataList::END(data)
                =>{
                    buf.put_u8(data);
                },
                RequestDataList::DEVICE_SN(data)|
                RequestDataList::RESERCED(data)|
                RequestDataList::CHANGE_VALUE(data)|
                RequestDataList::SET_PULSE_FREQ(data)|
                RequestDataList::PULSE_MONI(data)|
                RequestDataList::SET_VOL(data)|
                RequestDataList::HV_MONI(data)|
                RequestDataList::POWER_CONSUM_MONI(data)
                =>{
                    buf.put_u16(data);
                },
                RequestDataList::L_RESERVED(data)|
                RequestDataList::L2_RESERVED(data)
                =>{
                    buf.put_u32(data);
                },
                RequestDataList::SET_PULSE_TIME(data)
                =>{
                    for i in data{
                        buf.put_u16(i);
                    }
                }
                _=>{
                }
            }
        }
        Ok(())
    }
}

//리스트타입을 합치기위한 열거형
enum RequestDataList{
    
    START(u8),
    LENGHTH(u8),
    DEVICE_SN(u16),
    RESERCED(u16),
    COMMAND(u8),
    CHANGE_VALUE(u16),
    PULSE_ONOFF(u8),
    SET_PULSE_FREQ(u16),
    SET_PULSE_TIME([u16;2]),
    PULSE_MONI(u16),
    HV_ONOFF(u8),
    SET_VOL(u16),
    HV_MONI(u16),
    OPEN_SENSOR_MONI(u8),
    POWER_CONSUM_MONI(u16),
    L_RESERVED(u32),
    L2_RESERVED(u32),
    CHECKSUM(u8),
    END(u8),
}
#[derive(Debug,PartialEq,Eq,Serialize,Deserialize,Defaults)]
pub struct RequestData{
    //175 0xAF고정
    #[serde(with = "SerHex::<StrictPfx>")]
    #[def = "0xAF"]
    start: u8,
    //33고정
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "33"]
    length: u8,
    //0001임시
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0001"]
    device_sn: u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0000"]
    reserved: u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x01"]
    command: u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    change_value:u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "00"]
    pulse_onoff:u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0001"]
    set_pulse_freq:u16,
    #[serde(with = "SerHex::<StrictPfx>")]
    #[def = "[0x0000,0x0000]"]
    set_pulse_time:[u16;2],
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    pulse_moni:u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x00"]
    hv_onoff:u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0000"]
    set_vol:u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0000"]
    hv_moni:u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x00"]
    o_sens_moni:u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x0000"]
    p_consum_moni:u16,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x00000000"]
    r_reserved:u32,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x00000000"]
    t_reserved:u32,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0x00"]
    chechksum:u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0xFC"]
    end:u8,
}
impl RequestData {
    //변경된 값을 구조체에서 변경
    fn into_change_value(&mut self, change_value:u16){
        self.change_value=change_value;
    }
    //리스트로 반환
    fn to_list(&self)->Vec<RequestDataList>
    {
        let mut list=vec![
            RequestDataList::START(self.start),
            RequestDataList::LENGHTH(self.length),
            RequestDataList::DEVICE_SN(self.device_sn),
            RequestDataList::RESERCED(self.reserved),
            RequestDataList::COMMAND(self.command),
            RequestDataList::CHANGE_VALUE(self.change_value),
            RequestDataList::PULSE_ONOFF(self.pulse_onoff),
            RequestDataList::SET_PULSE_FREQ(self.set_pulse_freq),
            RequestDataList::SET_PULSE_TIME(self.set_pulse_time),
            RequestDataList::PULSE_MONI(self.pulse_moni),
            RequestDataList::HV_ONOFF(self.hv_onoff),
            RequestDataList::SET_VOL(self.set_vol),
            RequestDataList::HV_MONI(self.hv_moni),
            RequestDataList::OPEN_SENSOR_MONI(self.o_sens_moni),
            RequestDataList::POWER_CONSUM_MONI(self.p_consum_moni),
            RequestDataList::L_RESERVED(self.r_reserved),
            RequestDataList::L2_RESERVED(self.t_reserved),
            RequestDataList::CHECKSUM(self.chechksum),
            RequestDataList::END(self.end),
        ];
        list
    }
    //구조체내에 데이터를 계산하고 체크섬변경
    fn checksum(&mut self){
        let mut list = self.to_list();
        let mut sumdata:u64=0;
        for i in &list {
            match *i {
                RequestDataList::LENGHTH(data)|
                RequestDataList::COMMAND(data)|
                RequestDataList::PULSE_ONOFF(data)|
                RequestDataList::HV_ONOFF(data)|
                RequestDataList::OPEN_SENSOR_MONI(data)
                =>{
                    sumdata+=u64::from(data);
                },
                RequestDataList::DEVICE_SN(data)|
                RequestDataList::RESERCED(data)|
                RequestDataList::CHANGE_VALUE(data)|
                RequestDataList::SET_PULSE_FREQ(data)|
                RequestDataList::PULSE_MONI(data)|
                RequestDataList::SET_VOL(data)|
                RequestDataList::HV_MONI(data)|
                RequestDataList::POWER_CONSUM_MONI(data)
                =>{
                    sumdata+=u64::from(data);
                },
                RequestDataList::SET_PULSE_TIME(data)
                =>{
                    for i in data{
                        sumdata+=u64::from(i);
                    }
                },
                RequestDataList::L_RESERVED(data)|
                RequestDataList::L2_RESERVED(data)
                =>{
                    sumdata+=u64::from(data);
                },
                _=>{
                    // buf.put_u8(_);
                }
            }
        }
        let hex_str = format!("{:#x}",sumdata);
        let test =hex::decode(hex_str.as_str()).unwrap();
        self.chechksum=test[test.len()];
    }
}


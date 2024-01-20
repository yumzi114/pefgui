

use futures::{ StreamExt, SinkExt};
use tokio_util::codec::{Decoder, Encoder};
use tokio_serial::{SerialPortBuilderExt, SerialPort, StopBits};
use bytes::{BytesMut, BufMut};
use std::{io, str,u8};
use serde::{Serialize, Deserialize};
use serde_hex::{SerHex,StrictPfx,CompactPfx};
use defaults::Defaults;
pub mod device;
use device::{PulseInfo,VolatageInfo};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";


pub enum ChageList{
    HighVolValue,
    PulseFreq,
    Pulse_ON_OFF_Time,
    HighVol_ON_OFF,
    Pulse_ON_OFF
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
pub enum RequestDataList{
    
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
#[derive(Debug,PartialEq,Eq,Serialize,Deserialize,Defaults,Clone,Copy)]
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
    pub fn into_change_value(&mut self, change_value:ChageList){
        let c_value:u16 = match change_value {
            ChageList::HighVolValue=>0b0000_0010_0000_0000,
            ChageList::PulseFreq=>0b1000_0000_0000_0000,
            ChageList::Pulse_ON_OFF_Time=>0b1000_0000_0000_0000,
            ChageList::Pulse_ON_OFF=>0b0001_0000_0000_0000,
            ChageList::HighVol_ON_OFF=>0b0000_0100_0000_0000,
        };
        self.change_value=c_value;
        self.checksum();
    }
    pub fn getter(&mut self){
        let pulse_info:PulseInfo = confy::load("pefapp", "pulse").unwrap();
        let vol_info:VolatageInfo = confy::load("pefapp", "vol").unwrap();
        //pulse set
        self.pulse_onoff=if pulse_info.power {1}else{0};
        self.set_pulse_freq=pulse_info.freq_value as u16;
        self.set_pulse_time[0]=pulse_info.on_time_value as u16;
        self.set_pulse_time[1]=pulse_info.off_time_value as u16;
        //voltage set
        self.hv_onoff=if vol_info.power{1}else{0};
        self.set_vol=vol_info.value as u16;
    }
    //리스트로 반환
    pub fn to_list(&self)->Vec<RequestDataList>
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
    pub fn checksum(&mut self){
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
        let test =hex::decode(&hex_str[hex_str.len()-2..]).unwrap();
        self.chechksum=test[0];
    }
}


use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::{borrow::BorrowMut, io::Cursor, ops::Index};

use futures::{ StreamExt, SinkExt};
use tokio_util::codec::{Decoder, Encoder};
use tokio_serial::{SerialPortBuilderExt, SerialPort, StopBits};
use bytes::{BytesMut, BufMut};
use std::{fmt::Debug, io, str, u8};
use serde::{Serialize, Deserialize, de::value};
use serde_hex::{SerHex,StrictPfx,CompactPfx};
use defaults::Defaults;
pub mod device;
use device::{PulseInfo,VolatageInfo};
use std::fmt;
#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";
// const DEFAULT_TTY: &str = "/dev/ttyAMA0";

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
    type Item = Vec<u8>;
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let start = src.as_ref().iter().position(|x| *x == 0xFC);
        if let Some(n) = start {
            let line = src.split_to(n+1);
            let line_list = line.to_vec();
            if line_list.len()==36&&line_list[0]==0xAF&&line_list[1]==33{
                println!("-BUFFER-{:?}",line_list);
                println!("-LEN-{:?}",line_list.len());
                return Ok(Some(line_list));
            }
      
        }
 
        Ok(None)
    }
}
impl Encoder<Vec<RequestDataList>> for LineCodec {
    type Error = io::Error;
    fn encode(&mut self, code:Vec<RequestDataList>, buf: &mut BytesMut) -> Result<(), io::Error> {
        // buf.put_u8(b'\r');
        // buf.put_u8(b'\n');
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
                RequestDataList::RESERVED(data)|
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
                    data.map(|x|buf.put_u16(x));
                }
                _=>{
                    continue;
                }
            }
        }
        Ok(())
    }
}

//리스트타입을 합치기위한 열거형\
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum RequestDataList{
    START(u8),
    LENGHTH(u8),
    DEVICE_SN(u16),
    RESERVED(u16),
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
impl fmt::Display for RequestDataList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            RequestDataList::START(value) => write!(f,"{}",value),
            RequestDataList::LENGHTH(value)=>write!(f, "{}",value),
            RequestDataList::DEVICE_SN(value)=> write!(f,"{}",value),
            RequestDataList::RESERVED(value)=> write!(f,"{}",value),
            RequestDataList::COMMAND(value)=> write!(f,"{}",value),
            RequestDataList::CHANGE_VALUE(value)=> write!(f,"{}",value),
            RequestDataList::PULSE_ONOFF(value)=> write!(f,"{}",value),
            RequestDataList::SET_PULSE_FREQ(value)=> write!(f,"{}",value),
            // RequestDataList::SET_PULSE_TIME([value,value1])=> write!(f,"{},{}",value,value1),
            RequestDataList::SET_PULSE_TIME([value,value1])=> write!(f,"{}",value),
            RequestDataList::PULSE_MONI(value)=> write!(f,"{}",value),
            RequestDataList::HV_ONOFF(value)=> write!(f,"{}",value),
            RequestDataList::SET_VOL(value)=> write!(f,"{}",value),
            RequestDataList::HV_MONI(value)=> write!(f,"{}",value),
            RequestDataList::OPEN_SENSOR_MONI(value)=> write!(f,"{}",value),
            RequestDataList::POWER_CONSUM_MONI(value)=> write!(f,"{}",value),
            RequestDataList::L_RESERVED(value)=> write!(f,"{}",value),
            RequestDataList::L2_RESERVED(value)=> write!(f,"{}",value),
            RequestDataList::CHECKSUM(value)=> write!(f,"{}",value),
            RequestDataList::END(value)=> write!(f,"{}",value),
       }
    }
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
    #[def = "0x0000"]
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
    checksum:u8,
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0xFC"]
    end:u8,
}
impl RequestData {
    //변경된 값을 구조체에서 변경
    pub fn into_change_value(&mut self, change_value:ChageList){
        let c_value:u16 = match change_value {
            
            ChageList::Pulse_ON_OFF_Time=>0b0000_0000_1000_0000,
            //
            ChageList::Pulse_ON_OFF=>0b0000_0000_0001_0000,
            ChageList::PulseFreq=>0b0000_0000_1000_0000,
            ChageList::HighVol_ON_OFF=>0b0000_0000_0000_0100,

            ChageList::HighVolValue=>0b0000_0000_0000_0010,
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
    pub fn parser(&mut self, buf: &Vec<u8>)->Result<Vec<RequestDataList>,String>{
  
        if buf.len()==36{
            self.start=buf[0] as u8;
            self.length=buf[1] as u8;
            self.device_sn=u16::from_be_bytes([buf[2],buf[3]]);
            self.reserved=u16::from_be_bytes([buf[4],buf[5]]);
            self.command=buf[6] as u8;
            self.change_value=u16::from_be_bytes([buf[7],buf[8]]);
            self.pulse_onoff=buf[9] as u8;
            self.set_pulse_freq=u16::from_be_bytes([buf[10],buf[11]]);
            self.set_pulse_time=[
                u16::from_be_bytes([buf[12],buf[13]]),
                u16::from_be_bytes([buf[14],buf[15]])
            ];
            self.pulse_moni=u16::from_be_bytes([buf[16],buf[17]]);
            self.hv_onoff=buf[18] as u8;
            self.set_vol=u16::from_be_bytes([buf[19],buf[20]]);
            self.hv_moni=u16::from_be_bytes([buf[21],buf[22]]);
            self.o_sens_moni=buf[23] as u8;
            self.p_consum_moni=u16::from_be_bytes([buf[24],buf[25]]);
            self.r_reserved=u32::from_be_bytes([buf[26],buf[27],buf[28],buf[29]]);
            self.t_reserved=u32::from_be_bytes([buf[30],buf[31],buf[32],buf[33]]);
            self.checksum=u8::from_be_bytes([buf[34]]);
            self.end=u8::from_be_bytes([buf[35]]);
            return Ok(self.to_list())
        }else{
            return Err("Fail Parsing".to_string());
        }
    }
    //리스트로 반환
    pub fn to_list(&self)->Vec<RequestDataList>
    {
        let list=vec![
            RequestDataList::START(self.start),
            RequestDataList::LENGHTH(self.length),
            RequestDataList::DEVICE_SN(self.device_sn),
            RequestDataList::RESERVED(self.reserved),
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
            RequestDataList::CHECKSUM(self.checksum),
            RequestDataList::END(self.end),
        ];
        list
    }
    pub fn socket_fmt(&self)->String{
        let mut fmt_string = String::new();
        let list = self.to_list();
        let mut iter = list.iter();
        if let Some(name)=iter.next(){
            fmt_string.push_str(&name.to_string()[..]);
        };
        for i in iter{
            let str = format!(",{}",*i);
            fmt_string.push_str(str.as_str());
        }
        fmt_string
    }
    //구조체내에 데이터를 계산하고 체크섬변경
    pub fn checksum(&mut self){
        let list = self.to_list();
        let sumdata: u128 =list_add(&list);
        let hex_str = format!("{:#x}",sumdata);
        let test=hex::decode(&hex_str[hex_str.len()-2..]);
        if let Ok(data)=test{
            self.checksum=data[0];
        }
        else{
            let hex_str = hex_str.trim_start_matches("0x");
            self.checksum=u8::from_str_radix(hex_str,16).unwrap();
        }
    }
    pub fn is_checksum(&self)->Result<String, String>{
        let list = self.to_list();
        let sumdata: u128 =list_add(&list);
        println!("sumdata{}",sumdata);
        let hex_str = format!("{:#x}",sumdata);
        println!("hex{}",hex_str);
        let check_sum =hex::decode(&hex_str[hex_str.len()-2..]);
        if let Ok(data)=check_sum{
            println!("check{:?}",data);
            println!("-----------------------");
            if self.checksum!=data[0]{
                return Err("Fail checksum Err".to_string());
            }
            let num = data[0].to_string();
            return Ok(num);
        }
        else{
            let hex_str = hex_str.trim_start_matches("0x");
            // println!("{:?}",hex_str);
            let checksum=u8::from_str_radix(hex_str,16).unwrap();
            println!("{}",checksum);
            if self.checksum!=checksum{
                return Err("Fail checksum Err".to_string());
            }
            let num = checksum.to_string();
            return Ok(num);
        }
        
        
    }
    fn is_err_response(&self)->Result<String,String>{
        match self.command {
            0xE0=>Err("Over Limit".to_string()),
            0xE1=>Err("Non Response".to_string()),
            0xE2=>Err("CRC Error".to_string()),
            //임시에러
            0xE3=>Err("Not Start".to_string()),
            0xE4=>Err("Not DD".to_string()),
            0x01|0x02|0x03=>Ok("success".to_string()),
            _=>{
                Err("unknown".to_string())
            }
        }
    }
    pub fn check_all(&self)->Result<String,String>{
        self.is_err_response()?;
        self.is_checksum()?;
        Ok("success".to_string())
    }
}


pub fn list_add(list:&Vec<RequestDataList>)->u128{
    let mut sumdata:u128=0;
        for i in list {
            match *i {
                // RequestDataList::LENGHTH(data)|
                RequestDataList::COMMAND(data)|
                RequestDataList::PULSE_ONOFF(data)|
                RequestDataList::HV_ONOFF(data)|
                RequestDataList::OPEN_SENSOR_MONI(data)
                =>{
                    sumdata+=u128::from(data);
                    println!("COMMAND:{}",data);
                },
                RequestDataList::DEVICE_SN(data)|
                RequestDataList::RESERVED(data)|
                RequestDataList::CHANGE_VALUE(data)|
                RequestDataList::SET_PULSE_FREQ(data)|
                RequestDataList::PULSE_MONI(data)|
                RequestDataList::SET_VOL(data)|
                RequestDataList::HV_MONI(data)|
                RequestDataList::POWER_CONSUM_MONI(data)
                =>{
                    println!("DEVICE_SN:{}",data);
                    data.to_be_bytes().map(|x| sumdata+=u128::from(x));
                },
      
                RequestDataList::SET_PULSE_TIME(data)
                =>{
                    for i in data{
                        println!("SET_PULSE_TIME:{}",i);
                        i.to_be_bytes().map(|x| sumdata+=u128::from(x));
                    }
                },
                RequestDataList::L_RESERVED(data)|
                RequestDataList::L2_RESERVED(data)
                =>{
                    println!("L_RESERVED:{}",data);
                    data.to_be_bytes().map(|x| sumdata+=u128::from(x));
                    
                },
                _=>{
                    // continue;
                }
            }
        }
        println!("ALL SUM{}",sumdata);
        println!("HEX{:#x}",sumdata);
        println!("---------sumdata---");
    sumdata
}



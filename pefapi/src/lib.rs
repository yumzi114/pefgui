

use futures::{stream::{SplitStream, SplitSink}, StreamExt, SinkExt, FutureExt};
use tokio_util::codec::{Decoder, Encoder, Framed};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use bytes::{BytesMut, BufMut};
use std::{io, str, time::Duration,thread, sync::{Arc,Mutex}};
use serde::{Serialize, Deserialize};
use serde_hex::{SerHex,StrictPfx,CompactPfx};
use binascii::bin2hex;
use defaults::Defaults;
use std::error::Error;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyAMA3";
// const DEFAULT_TTY: &str = "/dev/tty0";


// #[derive(Clone)]
// #[derive(Clone)]

// pub struct AppChannel{
//     tx:Arc<Mutex<SplitSink<Framed<SerialStream, LineCodec>, String>>>,
//     rx:Arc<Mutex<SplitStream<Framed<SerialStream, LineCodec>>>>
// }
pub struct AppChannel{
    // tx:SplitSink<Framed<SerialStream, LineCodec>, String>,
    // rx:SplitStream<Framed<SerialStream, LineCodec>>
}


impl AppChannel {
    // #[tokio::main]
    pub fn new()->Self{
        // let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
        // // let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native().unwrap();
        // #[cfg(unix)]
        // port.set_exclusive(false)
        //     .expect("Unable to set serial port exclusive to false");
        
        // let (mut tx, mut rx) =LineCodec.framed(port).split();
        // let tx = Arc::new(Mutex::new(tx));
        // let rx = Arc::new(Mutex::new(rx));
        // let test1 = tokio_serial::new(DEFAULT_TTY, 115_200).open_native().unwrap();
        Self{
            // rx,
            // tx
        }
    }
    #[tokio::main]
    pub async fn tx_send(&mut self)->tokio_serial::Result<()>{
        // let sender = self.tx.clone();
        // sender.lock().unwrap().send("item".to_string()).await.unwrap();
        let join_handle = tokio::spawn(async move {
            // let sss = *sender.lock().unwrap();
                let mut port = tokio_serial::new(DEFAULT_TTY, 115_200).open_native_async().unwrap();
                #[cfg(unix)]
                port.set_exclusive(false)
                    .expect("Unable to set serial port exclusive to false");
                let (mut tx, mut rx) =LineCodec.framed(port).split();
                tx.send("ssss".to_string()).await.unwrap();
        });
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
impl Encoder<String> for LineCodec {
    type Error = io::Error;

    // fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
    //     Ok(())
    // }
    fn encode(&mut self, line: String, buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(line.len() + 2);
        buf.put(line.as_bytes());
        buf.put_u8(b'\r');
        buf.put_u8(b'\n');
        Ok(())
    }
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
    #[def = "0001"]
    device_sn: u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    reserved: u16,
    
    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "01"]
    command: u8,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    change_value:u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "00"]
    pulse_onoff:u8,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "1"]
    set_pulse_freq:u16,

    #[serde(with = "SerHex::<StrictPfx>")]
    #[def = "[00,00]"]
    set_pulse_time:[u16;2],

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    pulse_moni:u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "1"]
    hv_onoff:u8,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "20000"]
    set_vol:u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    hv_moni:u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    o_sens_moni:u8,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0000"]
    p_consum_moni:u16,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0"]
    r_reserved:u32,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0"]
    t_reserved:u32,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0"]
    chechksum:u8,

    #[serde(with = "SerHex::<CompactPfx>")]
    #[def = "0xFC"]
    end:u8,
}


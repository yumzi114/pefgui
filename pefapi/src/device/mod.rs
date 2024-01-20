use serde_derive::{Serialize, Deserialize};
use super::{ChageList,RequestData};
use crossbeam_channel::{Sender};
#[derive(PartialEq, Serialize, Deserialize)]
pub struct VolatageInfo{
    pub power:bool,
    pub value:f32,
}
impl ::std::default::Default for VolatageInfo {
    fn default() -> Self { 
        Self{
            power: false,
            value: 0.,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PulseInfo{
    pub power:bool,
    pub freq_value:f32,
    pub off_time_value:f32,
    pub on_time_value:f32,
}
impl ::std::default::Default for PulseInfo {
    fn default() -> Self { 
        Self{
            power: false,
            freq_value: 0.,
            off_time_value: 0.,
            on_time_value: 0.,
        }
    }
}
//각각 구조체별로 변경사항을 체크하고 변경사항이 있을 경우, 파일로 저장 및 데이터처리
impl PulseInfo {
    pub fn save(&self, req_data:&mut RequestData, sender:&mut Sender<RequestData>){
        let file_PulseInfo:PulseInfo = confy::load("pefapp", "pulse").unwrap();
        
        if file_PulseInfo!=*self{
            let value = 
            if self.power!=file_PulseInfo.power{Some(ChageList::Pulse_ON_OFF)}
            else if self.freq_value!=file_PulseInfo.freq_value {Some(ChageList::PulseFreq)}
            else if self.on_time_value!=file_PulseInfo.on_time_value{Some(ChageList::Pulse_ON_OFF_Time)}
            else if self.off_time_value!=file_PulseInfo.off_time_value{Some(ChageList::Pulse_ON_OFF_Time)}
            else {None};
            
            match value {Some(value)=>{
                confy::store("pefapp", "pulse", self).unwrap();
                req_data.into_change_value(value);
                let data = req_data.clone();
                let save_sender = sender.clone();
                save_sender.send(data).unwrap();

            },
                _=>{}
            }
            
        }
    }
}
impl VolatageInfo {
    pub fn save(&self,req_data:&mut RequestData,sender:&mut Sender<RequestData>){
        let file_VolatageInfo:VolatageInfo = confy::load("pefapp", "vol").unwrap();
        if file_VolatageInfo!=*self{
            let value = 
            if self.power!=file_VolatageInfo.power {Some(ChageList::HighVol_ON_OFF)}
            else if self.value!=file_VolatageInfo.value {Some(ChageList::HighVolValue)}
            else {None};
            match value {
                Some(value)=>{
                    confy::store("pefapp", "vol", self).unwrap();
                    req_data.into_change_value(value);
                    let data = req_data.clone();
                    let save_sender = sender.clone();
                    save_sender.send(data).unwrap();
                },
                _=>{}
            }
            
        }
    }
}

use serde_derive::{Serialize, Deserialize};
use super::{ChageList,RequestData};
use crossbeam_channel::{Sender};
#[derive(PartialEq, Serialize, Deserialize,Clone,Copy)]
pub struct AppState{
    pub set_time:u16,
    pub limit_time:u16,
}
impl ::std::default::Default for AppState {
    fn default() -> Self { 
        Self{
            set_time: 0,
            limit_time: 0,
        }
    }
}
impl AppState {
    pub fn get_set_time_fmt(&self)->String{
        let time = self.set_time.checked_div(60);
        match time {
            Some(num)=>{
                if num==0{
                    return format!("{}M",self.set_time)    
                }
                let m_time = self.set_time-(num*60);
                return format!("{}H {}M",num,m_time)
            },
            None=>{ 
                // format!("{}",self.set_time).as_str()
                return format!("{}",self.set_time)
            }
        }
    }
    pub fn get_limit_time_fmt(&self)->String{
        let time = self.limit_time.checked_div(60);
        match time {
            Some(num)=>{
                if num==0{
                    return format!("{}M",self.limit_time)    
                }
                let m_time = self.limit_time-(num*60);
                    return format!("{}H {}M",num,m_time)
            },
            None=>{ 
                // format!("{}",self.set_time).as_str()
                return format!("{}",self.limit_time)
            }
        }
        
    }
    // pub fn get_limit_time_fmt()->&str{

    // }
    // pub fn set_time_save(&mut self,value:Option<u16>){
    //     self.set_time=value;
    //     confy::store("pefapp", "appstate", self).unwrap();
    // }
    // pub fn limit_time_save(&mut self,value:Option<u16>){
    //     self.limit_time=value;
    //     confy::store("pefapp", "appstate", self).unwrap();
    // }
}
#[derive(PartialEq, Serialize, Deserialize,Clone,Copy)]
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

#[derive(PartialEq, Serialize, Deserialize,Clone,Copy)]
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

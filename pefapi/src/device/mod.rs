use std::sync::{atomic::AtomicBool, Arc, Mutex};

use serde_derive::{Serialize, Deserialize};
use super::{ChageList,RequestData};
use crossbeam_channel::{Sender};
#[derive(Serialize, Deserialize,Clone)]
pub struct AppState{
    pub set_time:u16,
    pub limit_time:u16,
    pub job_time_bool:bool,
    pub job_time:u64,
}
impl ::std::default::Default for AppState {
    fn default() -> Self { 
        Self{
            set_time: 0,
            limit_time: 0,
            job_time:0,
            job_time_bool:false
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
                return format!("{}",self.limit_time)
            }
        }
        
    }
    pub fn get_job_time_fmt(&self)->String{
        let time = self.job_time.checked_div(1000);
        match time {
            Some(num)=>{
                if num==0{
                    return format!("00 : 00 : {:03}",self.job_time)    
                }
                let s_time = self.job_time-(num*1000);
                if let Some(m_num)=num.checked_div(60){
                    if m_num !=0{
                        let s_time = self.job_time-(num*1000);
                        let dd_time =num-(m_num*60);
                        return format!("{:02} : {:02} : {:03}",m_num,dd_time,s_time)
                    }
                    else{
                        return format!("00 : {:02} : {:03}",num,s_time)
                    }
                }
                
                return format!("00 : {:02} : {:03}",num,s_time)
            },
            None=>{ 
                return format!("00 : 00 : {:03}",self.job_time)
            }
        }
    }
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
            value: 0.0,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize,Clone,Copy)]
pub struct PulseInfo{
    pub power:bool,
    pub freq_value:u16,
    pub off_time_value:u16,
    pub on_time_value:u16,
    pub max_time_value:Option<u16>,
    pub pwm:Option<f32>,
}
impl ::std::default::Default for PulseInfo {
    fn default() -> Self { 
        Self{
            power: false,
            freq_value: 0,
            off_time_value: 0,
            on_time_value: 0,
            max_time_value:None,
            pwm:None
        }
    }
}
//각각 구조체별로 변경사항을 체크하고 변경사항이 있을 경우, 파일로 저장 및 데이터처리
impl PulseInfo {
    pub fn save(&mut self, req_data:&mut RequestData, sender:&mut Sender<RequestData>){
        let file_PulseInfo:PulseInfo = confy::load("pefapp", "pulse").unwrap();
        
        if file_PulseInfo!=*self{
            let value = 
            if self.power!=file_PulseInfo.power{Some(ChageList::Pulse_ON_OFF)}
            else if self.freq_value!=file_PulseInfo.freq_value {Some(ChageList::PulseFreq)}
            else if self.on_time_value!=file_PulseInfo.on_time_value{Some(ChageList::Pulse_ON_OFF_Time)}
            else {None};
            match value {
                Some(value)=>{
                let save_sender = sender.clone();
                if let Ok(_)=self.change_pwm(){
                    req_data.into_change_value(value);
                    let req=req_data.clone();
                    confy::store("pefapp", "pulse", self.clone()).unwrap();
                    save_sender.send(req).unwrap();
                }
                //자동설정아닐시
                else{
                    req_data.into_change_value(value);
                    self.pwm=Some(0.);
                    let req=req_data.clone();
                    confy::store("pefapp", "pulse", self.clone()).unwrap();
                    save_sender.send(req).unwrap();
                }
                req_data.change_value=0b0000_0000_0000_0000;
            },
                _=>{
                    
                }
            }
            
        }
    }
    pub fn pulse_on_off_set(&mut self)->Result<(),()>{
        let mut temp = self.power.clone();
        if self.freq_value!=0&&self.on_time_value!=0{
            if let Some(max_time)=self.max_time_value{
                if self.on_time_value <=max_time{
                    temp=true;
                }
                else{
                    temp=false;
                }
            }
            else{
                temp=false;
            }
        }else{
            temp=false;
        }
        if self.power!=temp{
            self.power=temp;
            return Ok(());
        }
        else {
            self.power=temp;
            return Err(())
        }
      
    }
    pub fn max_value_change(&mut self){
        let dd =50_0000_u32.checked_div(self.freq_value as u32 );
        if let Some(num)=dd{
            self.max_time_value=Some((num+1) as u16);
        }
        else {
            self.max_time_value=None;
        }
       
    }
    pub fn change_pwm(&mut self)->Result<(),()>{

        if let Some(max_num)=self.max_time_value{
            if self.freq_value!=0&&self.on_time_value!=0{
                let m_num=max_num as f32;
                let t_num = self.on_time_value as f32;
                self.pwm=Some((t_num /(m_num *2.)*100.)as f32);
                return Ok(())
            }else{
                return Err(());
            }
        }
        else{
            self.pwm=None;
            return Err(())
        }
    }
}
impl VolatageInfo {
    pub fn save(&mut self,req_data:&mut RequestData,sender:&mut Sender<RequestData>){
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
                    req_data.change_value=0b0000_0000_0000_0000;

                },
                _=>{

                }
            }
            
        }
    }
    pub fn volat_on_off_set(&mut self)->Result<(),()>{
        let mut temp = self.power.clone();
        if self.value !=0.0{
            temp=true;
        }
        else{
            temp=false;
        }
        if temp !=self.power{
            self.power=temp;
            return Ok(())
        }
        else{
            self.power=temp;
            return Err(())
        }
    }
}

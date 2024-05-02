use std::{fmt::format, sync::{Arc,Mutex}};

use crossbeam_channel::Sender;
use eframe::{egui::{Ui, self, InnerResponse, RichText, Sense, TextFormat, PointerState, Widget}, epaint::{Vec2, Color32, text::{LayoutJob, TextWrapping}, FontId, Pos2, vec2}};
use egui_extras::{TableBuilder, Column};
use pefapi::{device::AppState, LineCodec, RequestData, RequestDataList};
use super::{UserUi,MenuList,PulseInfo,VolatageInfo,};
use crate::keypad_view;
pub fn setting_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    pulse_info:&mut PulseInfo, 
    vol_info:&mut VolatageInfo,
    request:&mut RequestData, 
    sender:&mut Sender<RequestData>,
    response:&Arc<Mutex<Vec<RequestDataList>>>,
    report:&Arc<Mutex<Vec<RequestDataList>>>,
    app_state:&mut Arc<Mutex<AppState>>,
    timer_sender:&mut Sender<usize>,
    k_timer_sender:&mut Sender<u8>,

)->InnerResponse<()>{
    if uui.warning_pop{
        ui.set_enabled(false);
    }
    
    ui.vertical_centered(|ui|{
        
        ui.columns(2, |columns|{
            //좌측패널 컴퍼넌트
            columns[0].vertical_centered(|ui|{
                ui.add_space(60.);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        // ui.label(RichText::new("PWM").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response = button_respone(ui, uui, &MenuList::SetVoltage, format!("{} kV",vol_info.value.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="Voltage Value Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::SetVoltage,pos,k_timer_sender);
                            }
                        });
                        
                        let value = format!("Device : {}",report.lock().unwrap()[12]);
                        job_textwarp(ui,Color32::from_rgb(184, 184, 184),50.0,value.as_str(),true);
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Frequency").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response: egui::Response = button_respone(ui, uui, &MenuList::PulseFreq, format!("{} Hz",pulse_info.freq_value.to_string()));
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="Pulse Value Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::PulseFreq,pos,k_timer_sender);
                            }
                        });
                        let value = format!("Device : {}",report.lock().unwrap()[9]);
                        job_textwarp(ui,Color32::from_rgb(184, 184, 184),50.0,value.as_str(),true);
                    });
                });
                ui.add_space(30.);
                ui.label(RichText::new("Time Value").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Pulse Time").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            ui.add_enabled_ui({
                                match  pulse_info.max_time_value {
                                    Some(_)=>{
                                        true
                                    }
                                    None=>{
                                        false
                                    }
                                }
                                
                            }, |ui|{
                                let b_response = button_respone(ui, uui, &MenuList::PulseOnTime, format!("{} us",pulse_info.on_time_value.to_string()));
                                if b_response.clicked(){
                                    uui.set_value.clear();
                                    uui.status_str="Pulse ON_TIME Setting".to_string();
                                    let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                    click_voltage(uui,MenuList::PulseOnTime,pos,k_timer_sender);
                                }
                                });
                            
                        });
                        let value = format!("Device : {}",report.lock().unwrap()[8]);
                        job_textwarp(ui,Color32::from_rgb(184, 184, 184),50.0,value.as_str(),true);
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.label(RichText::new("Running Time").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(20.);
                            let b_response = button_respone(ui, uui, &MenuList::RunningTime, format!("{} M",(*app_state.lock().unwrap()).set_time.to_string()));
                            
                            if b_response.clicked(){
                                uui.set_value.clear();
                                uui.status_str="App Run Time Setting".to_string();
                                let pos = b_response.hover_pos().unwrap_or(Pos2{x:50.,y:50.});
                                click_voltage(uui,MenuList::RunningTime,pos,k_timer_sender);
                            }
                        });
                        if (*app_state.lock().unwrap()).set_time>0{
                            let value = format!("Limit Time : {}",(*app_state.lock().unwrap()).limit_time.to_string());
                            job_textwarp(ui,Color32::from_rgb(184, 184, 184),50.0,value.as_str(),true);
                        }
                        else {
                            job_textwarp(ui,Color32::from_rgb(184, 184, 184),50.0,"Set Time : None",true);
                        }
             
                    });
                });
                ui.add_space(10.);
                ui.horizontal_wrapped(|ui|{
                    ui.add_space(20.);
                    ui.label(RichText::new("Timer ON/OFF").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                    ui.add(toggle(&mut uui.timer));
                    // ui.color_edit_button_srgb(&mut uui.color)
                    //pwm 테스트시
                    // if let Some(data)=pulse_info.pwm{
                    //     let feild = format!("{:.1}",data);
                    //     let temp11:Vec<&str>  = feild.split(".").collect();
                    //     if !temp11.is_empty(){
                            
                    //         ui.label(RichText::new(format!("{:?}",temp11[0])).strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                    //         ui.label(RichText::new(format!("{:?}",temp11[1])).strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));    
                    //     }
                    //     // ui.label(RichText::new(feild).strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                    //     // ui.label(RichText::new(feild).strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                    // }
                    // else{
                    //     ui.label(RichText::new("NO DATA").strong().size(45.0).color(Color32::from_rgb(38, 150, 255)));
                    // }
                });
                
                
            });
            //우측패널 컴퍼넌트
            columns[1].vertical_centered(|ui|{
                if !uui.keypad.popon{
                    ui.add_space(40.);
                    // ui.label(RichText::new("Monitoring").strong().size(45.0).color(Color32::from_rgb(255, 200, 36)));
                    ui.add_space(20.);
                    ui.push_id(1, |ui| {
                        TableBuilder::new(ui)

                        .cell_layout(egui::Layout::top_down(egui::Align::Center))
                        .striped(false)
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .header(60.0, |mut header| {
                            header.col(|ui| {
                                ui.add_space(10.0);
                                ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                
                            });
                            header.col(|ui| {
                                ui.add_space(10.0);
                                ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                        })
                        .body(|mut body| {
                            body.row(140.0, |mut row| {
                                row.col(|ui| {
                                    ui.add_space(10.0);
                                    // let b_response = button_respone(ui, uui, &MenuList::SetVoltage, format!("{} kV",vol_info.value.to_string()));
                                    //버튼식
                                    if ui.add_sized([120.0, 120.0], egui::ImageButton::new(check_on(pulse_info.power))).clicked(){
                                        pulse_info.power=!pulse_info.power;
                                        pulse_info.save(request,sender);
                                        if vol_info.power&&pulse_info.power&&uui.timer{
                                            (*app_state.lock().unwrap()).job_time_bool=true;
                                            // (*app_state.lock().unwrap()).job_time=0;
                                        }else {
                                            (*app_state.lock().unwrap()).job_time_bool=false;
                                            // (*app_state.lock().unwrap()).job_time=0;
                                        }
                                    };
                                    //     .paint_at(ui, rect);
                                    // if _response.clicked(){
                                    //     if uui.view_menu==ViewList::Setting{
                                    //         uui.view_menu=ViewList::Main;
                                    //     }else{
                                    //         uui.view_menu=ViewList::Setting;
                                    //     }
                                    // }
                                    //일반뷰
                                    // if report.lock().unwrap()[6]==RequestDataList::PULSE_ONOFF(1){
                                    //     ui.add_sized([120.0, 120.0], egui::Image::new(egui::include_image!("../../files/pngegg2.png")));
                                    // }
                                    // else{
                                    //     ui.add_sized([120.0, 120.0], egui::Image::new(egui::include_image!("../../files/pngegg.png")));
                                    // }
                                });
                                row.col(|ui| {
                                    ui.add_space(10.0);
                                    //버튼식
                                    if ui.add_sized([120.0, 120.0], egui::ImageButton::new(check_on(vol_info.power))).clicked(){
                                        vol_info.power=!vol_info.power;
                                        vol_info.save(request,sender);
                                        if vol_info.power&&pulse_info.power&&uui.timer{
                                            (*app_state.lock().unwrap()).job_time_bool=true;
                                            // (*app_state.lock().unwrap()).job_time=0;
                                        }else {
                                            (*app_state.lock().unwrap()).job_time_bool=false;
                                            // (*app_state.lock().unwrap()).job_time=0;
                                        }
                                    };
                                    //일반뷰
                                    // if report.lock().unwrap()[10]==RequestDataList::HV_ONOFF(1){
                                    //     ui.add_sized([120.0, 120.0], egui::Image::new(egui::include_image!("../../files/pngegg2.png")));
                                    // }
                                    // else{
                                    //     ui.add_sized([120.0, 120.0], egui::Image::new(egui::include_image!("../../files/pngegg.png")));
                                    // }
                                });
                            });
                        })
                    });
                    
                    
                    ui.add_space(15.);
                    // let (rect, _response) =ui.allocate_at_least(Vec2::new(450., 80.), Sense::click_and_drag());
                    // ui.painter_at(rect).
                    ui.push_id(2, |ui| {
                        TableBuilder::new(ui)
                        .cell_layout(egui::Layout::top_down(egui::Align::Center))
                        .striped(true)
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .header(70.0, |mut header| {
                            header.col(|ui| {
                                ui.add_space(10.);
                                
                                ui.label(RichText::new("Voltage").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            header.col(|ui| {
                                ui.add_space(10.);
                                ui.label(RichText::new(format!("{}",report.lock().unwrap()[12])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                        })
                        .body(|mut body| {
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new(format!("{}",report.lock().unwrap()[9])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                            });
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("Power\nConsumption").strong().size(30.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    ui.label(RichText::new(format!("{}",report.lock().unwrap()[14])).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                            });
                            body.row(70.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("Device\nLimit Time").strong().size(30.0).color(Color32::from_rgb(247, 104, 42)));
                                });
                                row.col(|ui| {
                                    ui.add_space(10.);
                                    
                                    if (*app_state.lock().unwrap()).limit_time>0{
                                        ui.label(RichText::new(
                                            (*app_state.lock().unwrap()).get_limit_time_fmt()
                                        ).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                    }
                                    else {
                                        ui.label(RichText::new("None").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                                    }
                                });
                            });
                            if uui.timer{
                                body.row(70.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(RichText::new("Timer").strong().size(45.0).color(Color32::from_rgb(255, 0, 0)));
                                        
                                    });
                                    row.col(|ui| {
                                        ui.horizontal(|ui|{
                                            ui.label(RichText::new((*app_state.lock().unwrap()).get_job_time_fmt()).strong().size(45.0).color(Color32::from_rgb(255, 0, 0)));
                                        });
                                        
                                        
                                    });
                                });
                            }
                            
                        });
                    });
                    
                }else {
                    ui.add_space(30.);
                    keypad_view(
                        ui, 
                        ctx, 
                        pulse_info, 
                        vol_info, 
                        &mut uui.keypad.sellist, 
                        &mut uui.set_value, 
                        &mut uui.keypad.popon, 
                        &mut uui.status_str,
                        request,
                        sender,
                        app_state,
                        &mut uui.warning_pop,
                        timer_sender,
                        k_timer_sender
                    );
                }
            });
        })
    })
}

//메뉴리스트 중복처리
fn click_voltage(uui:&mut UserUi, selmenu:MenuList, get_pos:Pos2,k_timer_sender:&mut Sender<u8>,){
    if uui.keypad.popon && uui.keypad.sellist==Some(selmenu){
        uui.keypad.popon=false;
        uui.keypad.sellist=None;
        uui.keypad.uipost=get_pos;
    }
    else if uui.keypad.popon==false||uui.keypad.sellist!=Some(selmenu){
        uui.keypad.popon=true;
        uui.keypad.sellist=Some(selmenu);
        uui.keypad.uipost=get_pos;
        k_timer_sender.send(3).unwrap();
    };
}

//버튼 클릭색상처리
fn button_respone(ui: &mut Ui, uui:&UserUi, check_sel:&MenuList,value_str:String)->egui::Response{
    let size = if value_str.len()>=7{65_f32}else{90_f32};
    let b_response: egui::Response = 
    if uui.keypad.sellist==Some(*check_sel){
   
        ui.add(egui::Button::new(
            // job
            RichText::new(value_str).strong().size(size)
        )
            .min_size(Vec2{x:420.0,y:130.0})
            .sense(Sense::click())
            .fill(Color32::from_rgb(133, 255, 115))
            .rounding(egui::Rounding{nw:40.,ne:40.,sw:40.,se:40.,})
        )
    }
    else {
   
        ui.add(egui::Button::new(
            // job
            RichText::new(value_str).strong().size(size)
        )
            .min_size(Vec2{x:420.0,y:130.0})
            .rounding(egui::Rounding{nw:40.,ne:40.,sw:40.,se:40.,})
            .sense(Sense::click())
        )
    };
    b_response
}

fn check_on(on_off:bool)->eframe::egui::Image<'static>{
    match on_off{
        false=>{egui::Image::new(egui::include_image!("../../files/pngegg.png")).rounding(360.0)},
        true=>{egui::Image::new(egui::include_image!("../../files/pngegg2.png")).rounding(360.0)} 
     }
}

fn job_textwarp(ui:&mut egui::Ui,color: Color32,size:f32,text:&str,hover:bool)->egui::Response{
    let mut job = LayoutJob::single_section(text.clone().to_string(), 
        TextFormat{
            font_id: FontId::proportional(size),
            color: color,
            ..Default::default()
        });
        job.wrap = TextWrapping {
            max_rows: 1,
            break_anywhere: false,
            overflow_character: Some('…'),
            ..Default::default()
        };
    if hover{
        ui.label(job).on_hover_ui(|ui|{
            ui.label(RichText::new(text).strong().size(75.));
        })
    }else {
        ui.label(job)
    }
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.5);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // Let's ask for a simple animation from egui.
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        let how_on = ui.ctx().animate_bool(response.id, *on);
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
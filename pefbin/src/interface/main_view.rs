use std::sync::{Arc, Mutex};

use eframe::{egui::{self, InnerResponse, RichText, Sense, Ui}, epaint::{Color32, Vec2}};
use egui_extras::{Column, TableBuilder};
use pefapi::{app_error::ErrorList, device::AppState, RequestDataList};

use super::UserUi;

pub fn main_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    response:&Arc<Mutex<Vec<RequestDataList>>>,
    report:&Arc<Mutex<Vec<RequestDataList>>>,
    err_report:&Arc<Mutex<Vec<RequestDataList>>>,
    repo_error:&Arc<Mutex<ErrorList>>,
    app_state:&mut Arc<Mutex<AppState>>
)->InnerResponse<()>{
    // let mem = response.clone();
    // let report=report.clone();
    // let app_state_mem = app_state.clone();
    uui.keypad.popon=false;
    uui.keypad.sellist=None;
    //Report에 따라 뷰
    match *repo_error.lock().unwrap(){
        ErrorList::None=>{
            device_on_view(ui,ctx,uui,response,report,repo_error,app_state)
        }
        _=>{
            device_off_view(ui,ctx,uui,response,err_report,repo_error,app_state)
        }
    }
}



fn device_on_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    response:&Arc<Mutex<Vec<RequestDataList>>>,
    report:&Arc<Mutex<Vec<RequestDataList>>>,
    repo_error:&Arc<Mutex<ErrorList>>,
    app_state:&mut Arc<Mutex<AppState>>
)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.add_space(20.);
        ui.label(RichText::new("Information").strong().size(80.0).color(Color32::from_rgb(38, 150, 255)));
        
        ui.add_space(20.);
        ui.columns(2, |columns| {
            //좌측 상태창
            columns[0].vertical_centered(|ui|{
                ui.push_id(1, |ui|{
                    TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .striped(true)
                    .sense(Sense::click_and_drag())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .body(|mut body| {
                        body.row(80.0, |mut row| {
                            row.set_selected(uui.table_sel[0]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Device S/N").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[2]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[0]=!uui.table_sel[0]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[1]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if report.lock().unwrap()[6]==RequestDataList::PULSE_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[1]=!uui.table_sel[1]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[2]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse Frequency").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[9]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[2]=!uui.table_sel[2]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[3]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse Times").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[8]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[3]=!uui.table_sel[3]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[4]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if report.lock().unwrap()[10]==RequestDataList::HV_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[4]=!uui.table_sel[4]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[5]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Voltage Value").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[12]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[5]=!uui.table_sel[5]
                            };
                        });
                    });
        
                });
            });
            //우측 상태창
            columns[1].vertical_centered(|ui|{
                ui.push_id(2, |ui|{
                    TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .striped(false)
                    .sense(Sense::click_and_drag())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .body(|mut body| {
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[6]);
                            row.col(|ui| {
                                ui.label(RichText::new("Power\nConsumption").strong().size(38.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[14]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[6]=!uui.table_sel[6]
                            };
                        });
                        
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[7]);
                            row.col(|ui| {
                                ui.label(RichText::new("Running\nSet Time").strong().size(38.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                // let value = format!("{}",mem.lock().unwrap()[14]);
                                if (*app_state.lock().unwrap()).set_time>0{
                                    ui.label(RichText::new(
                                        (*app_state.lock().unwrap()).get_set_time_fmt()
                                        // format!("{}",(*app_state_mem.lock().unwrap()).set_time.to_string())
                                    ).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                                }
                                else {
                                    ui.label(RichText::new("None").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                                }
                            });
                            if row.response().drag_released(){
                                uui.table_sel[7]=!uui.table_sel[7]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[8]);
                            row.col(|ui| {
                                ui.label(RichText::new("Running\nLimit Time").strong().size(38.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                if (*app_state.lock().unwrap()).limit_time>0{
                                    ui.label(RichText::new(
                                        (*app_state.lock().unwrap()).get_limit_time_fmt()
                                    ).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                                }
                                else {
                                    ui.label(RichText::new("None").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                                }
                            });
                            if row.response().drag_released(){
                                uui.table_sel[8]=!uui.table_sel[8]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[9]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("OPEN DOOR").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                
                                let value = if report.lock().unwrap()[13]==RequestDataList::OPEN_SENSOR_MONI(1){"OPEN"}else{"CLOSE"};
                                // let value = format!("{}",);
                                ui.label(RichText::new(value.to_owned()).strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[9]=!uui.table_sel[9]
                            };
                        });
                    });
        
                });
            });
        });
        
    })
}

pub fn device_off_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    response:&Arc<Mutex<Vec<RequestDataList>>>,
    report:&Arc<Mutex<Vec<RequestDataList>>>,
    repo_error:&Arc<Mutex<ErrorList>>,
    app_state:&mut Arc<Mutex<AppState>>
)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.add_space(20.);
        ui.label(RichText::new("Check Device").strong().size(80.0).color(Color32::RED));
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.add_space(15.);
            ui.label(RichText::new("Fail Device Connect Check This Values").strong().size(50.0).color(Color32::RED));
        });
        ui.add_space(20.);
        ui.columns(2, |columns| {
            //좌측 상태창
            columns[0].vertical_centered(|ui|{
                ui.push_id(1, |ui|{
                    TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .striped(true)
                    .sense(Sense::click_and_drag())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .body(|mut body| {
                        body.row(80.0, |mut row| {
                            row.set_selected(uui.table_sel[0]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Device S/N").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[2]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[0]=!uui.table_sel[0]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[1]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if report.lock().unwrap()[6]==RequestDataList::PULSE_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[1]=!uui.table_sel[1]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[2]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse Frequency").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[9]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[2]=!uui.table_sel[2]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[3]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Pulse Times").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[8]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[3]=!uui.table_sel[3]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[4]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("High Voltage").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if report.lock().unwrap()[10]==RequestDataList::HV_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[4]=!uui.table_sel[4]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[5]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("Voltage Value").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[12]);
                                ui.label(RichText::new(value).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[5]=!uui.table_sel[5]
                            };
                        });
                    });
        
                });
            });
            //우측 상태창
            columns[1].vertical_centered(|ui|{
                ui.push_id(2, |ui|{
                    TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .striped(false)
                    .sense(Sense::click_and_drag())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .body(|mut body| {
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[6]);
                            row.col(|ui| {
                                ui.label(RichText::new("Power\nConsumption").strong().size(38.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",report.lock().unwrap()[14]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[6]=!uui.table_sel[6]
                            };
                        });
                        body.row(80.0, |mut row| {
                            // row.index()
                            row.set_selected(uui.table_sel[9]);
                            row.col(|ui| {
                                ui.add_space(15.);
                                ui.label(RichText::new("OPEN DOOR").strong().size(45.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                
                                let value = if report.lock().unwrap()[13]==RequestDataList::OPEN_SENSOR_MONI(1){"OPEN"}else{"CLOSE"};
                                // let value = format!("{}",);
                                ui.label(RichText::new(value.to_owned()).strong().size(45.0).color(Color32::RED));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[9]=!uui.table_sel[9]
                            };
                        });
                    });
        
                });
            });
        });
    })
}
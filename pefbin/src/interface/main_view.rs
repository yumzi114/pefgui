use std::sync::{Arc, Mutex};

use eframe::{egui::{self, InnerResponse, RichText, Sense, Ui}, epaint::{Color32, Vec2}};
use egui_extras::{Column, TableBuilder};
use pefapi::RequestDataList;

use super::UserUi;

pub fn main_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    uui:&mut UserUi, 
    response:&Arc<Mutex<Vec<RequestDataList>>>
)->InnerResponse<()>{
    let mem = response.clone();
    ui.vertical_centered(|ui|{
        ui.add_space(20.);
        ui.label(RichText::new("Information").strong().size(100.0).color(Color32::from_rgb(38, 150, 255)));
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
                                ui.label(RichText::new("Device S/N").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",mem.lock().unwrap()[2]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("Pulse").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if mem.lock().unwrap()[6]==RequestDataList::PULSE_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("Pulse Frequency").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",mem.lock().unwrap()[7]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("Pulse Times").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",mem.lock().unwrap()[8]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("High Voltage").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = if mem.lock().unwrap()[6]==RequestDataList::HV_ONOFF(1){"ON"}else{"OFF"};
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("Voltage Value").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            row.col(|ui| {
                                ui.add_space(15.);
                                let value = format!("{}",mem.lock().unwrap()[8]);
                                ui.label(RichText::new(value).strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                let value = format!("{}",mem.lock().unwrap()[14]);
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
                                ui.label(RichText::new("test").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
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
                                ui.label(RichText::new("test").strong().size(50.0).color(Color32::from_rgb(247, 104, 42)));
                            });
                            if row.response().drag_released(){
                                uui.table_sel[8]=!uui.table_sel[8]
                            };
                        });
                    });
        
                });
            });
        });
        
    })
}

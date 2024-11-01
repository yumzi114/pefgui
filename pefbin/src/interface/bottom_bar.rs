use std::{sync::{Mutex, Arc}};
use super::ErrorList;
use eframe::{egui::{self, Ui, InnerResponse, RichText, Sense}, epaint::{Color32, Vec2}};
use pefapi::device::AppState;
use super::{UserUi,MenuList};
pub fn bottom_view(ui: &mut Ui,ctx: &egui::Context,mem:&Arc<Mutex<usize>>, uui:&mut UserUi, err_type:&Arc<Mutex<ErrorList>>,app_state:&mut Arc<Mutex<AppState>>,)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.horizontal_wrapped(|ui|{
            ui.label(RichText::new("").strong().size(60.0));
            ui.columns(2, |columns|{
                columns[0].horizontal_centered(|ui|{
                    let dasd = *mem.lock().unwrap();
                    let time = ".".repeat(dasd);
                    let mut status_string = String::new();
                    ui.add(egui::Label::new(RichText::new("      STATE : ").color(Color32::from_rgb(36, 101, 255)).strong().size(80.0)));
                    match *err_type.lock().unwrap() {
                        ErrorList::None=>{
                            if (*app_state.lock().unwrap()).limit_time!=0{
                                status_string=format!("Machine Running{}",time.as_str());    
                            }
                            else{
                                status_string=format!("Stand By{}",time.as_str());
                            }
                        },
                        ErrorList::CheckSumErr=>{
                            status_string="ResponseCheckSum Error".to_string();
                        },
                        ErrorList::CRCError=>{
                            status_string="CRC Error".to_string();
                        },
                        ErrorList::OverLimit=>{
                            status_string="Over Limit".to_string();
                        },
                        ErrorList::NonResponse=>{
                            status_string="NonResponse".to_string();
                        },
                        ErrorList::DeviceSNErr=>{
                            status_string="Board Check".to_string();
                        },
                        _=>{
                            status_string="Board Check".to_string();
                        }
                    }
                    if uui.keypad.popon {
                        status_string=uui.status_str.clone();
                    }
                    match uui.keypad.sellist {
                        Some(MenuList::PulseFreq)
                        // |Some(MenuList::PulseOffTime)
                        |Some(MenuList::PulseOnTime)
                        // |Some(MenuList::PulseOnTime)
                        |Some(MenuList::RunningTime)
                        |Some(MenuList::SetVoltage)=>{
                            match  &status_string[0..=4]{
                                "Limit"=>{
                                    ui.label(RichText::new(&status_string).strong().size(80.0));
                                },
                                "Frequ"=>{
                                    ui.label(RichText::new(&status_string).strong().size(50.0));
                                },
                                _=>{
                                    ui.label(RichText::new(&status_string).strong().size(80.0));
                                    if &status_string[status_string.len()-7..]=="Setting"{
                                        ui.add(egui::Spinner::new().size(50.));
                                    }
                                }
                            }
                        }
                        _=>{
                            ui.label(RichText::new(status_string).strong().size(80.0));
                        }
                    }
                    
                });
                columns[1].vertical_centered(|ui|{
                        ui.horizontal_wrapped(|ui|{
                            ui.add_space(550.);
                        });
                        ui.add_space(10.);
                        
                    });
            });
        });
        // ui.horizontal_wrapped(|ui|{
        //     ui.add_space(86.0);
        //     ui.label(RichText::new("").strong().size(60.0));
        //     ui.add_space(450.0);
        //     ui.add_space(810.0);
        //     let (temp_rect, _) =ui.allocate_at_least(Vec2::new(450., 70.), Sense::hover());
        //     egui::Image::new(egui::include_image!("../../files/gitclogo1.png"))
        //         .paint_at(ui, temp_rect);
        // });
    })
}

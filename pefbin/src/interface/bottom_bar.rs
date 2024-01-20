use std::{sync::{Mutex, Arc}, fmt::format};

use eframe::{egui::{self, Ui, InnerResponse, RichText, Layout, Sense}, epaint::{Color32, Vec2}, emath::Align};
use super::{UserUi,MenuList};
pub fn bottom_view(ui: &mut Ui,ctx: &egui::Context,mem:&Arc<Mutex<usize>>, uui:&mut UserUi)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.horizontal_wrapped(|ui|{
            ui.label(RichText::new("").strong().size(60.0));
            ui.columns(2, |columns|{
                columns[0].horizontal_centered(|ui|{
                    let dasd = *mem.lock().unwrap();
                    let time = ".".repeat(dasd);
                    let mut status_string = String::new();
                    ui.add(egui::Label::new(RichText::new("      STATE : ").color(Color32::from_rgb(36, 101, 255)).strong().size(80.0)));
                    if uui.keypad.popon {
                        status_string=uui.status_str.clone();
                    }
                    else {
                        status_string=format!("Waiting{}",time.as_str());
                    }
                    match uui.keypad.sellist {
                        Some(MenuList::PulseFreq)
                        |Some(MenuList::PulseOffTime)
                        |Some(MenuList::PulseOnTime)
                        |Some(MenuList::SetVoltage)=>{
                            ui.label(RichText::new(&status_string).strong().size(80.0));
                            if &status_string[status_string.len()-7..]=="Setting"{
                                ui.add(egui::Spinner::new().size(50.));
                            }
                        }
                        _=>{ui.label(RichText::new(status_string).strong().size(80.0));}
                    }
                    
                });
                columns[1].horizontal_centered(|ui|{
                    ui.add_space(550.);
                    let (one_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/asdasd.png"))
                        .paint_at(ui, one_rect);
                    ui.add_space(20.);
                    let (two_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/warning.png"))
                        .paint_at(ui, two_rect);
                    ui.add_space(20.);
                    let (temp_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/warning.png"))
                        .paint_at(ui, temp_rect);
                });
            });
        });
        ui.horizontal_wrapped(|ui|{
            ui.add_space(86.0);
            ui.label(RichText::new("").strong().size(60.0));
            ui.label(RichText::new("PEF R&D과제명").strong().size(60.0));
        });
    })
}

fn circle_ui (ui:&mut Ui,color:Color32,size:Vec2){
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let rect = response.rect;
    let c = rect.center();
    let r = rect.width() / 2.0 - 1.0;
    painter.circle_filled(c, r, color);
}
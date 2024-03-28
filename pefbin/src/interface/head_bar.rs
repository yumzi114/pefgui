use eframe::{egui::{self, Ui, InnerResponse, RichText, Sense}, epaint::{Color32, Vec2}};
use super::{UserUi, ViewList};
pub fn top_logo_view(ui: &mut Ui,ctx: &egui::Context, uui:&mut UserUi)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.columns(2, |columns|{
            columns[0].horizontal(|ui|{
                ui.vertical(|ui|{
                    ui.add_space(30.0);
                    ui.horizontal(|ui|{
                        ui.add_space(30.0);
                        let (rect, _response) =ui.allocate_at_least(Vec2::new(450., 120.), Sense::click_and_drag());
                        egui::Image::new(egui::include_image!("../../files/device_image/red.png"))
                            .paint_at(ui, rect);
                    });
                });
            });
            columns[1].vertical_centered(|ui|{
                ui.add_space(50.0);
                ui.horizontal_top(|ui|{
                    ui.add_space(420.0);
                    let (rect, _response) =ui.allocate_at_least(Vec2::new(450., 80.), Sense::click_and_drag());
                    egui::Image::new(egui::include_image!("../../files/chologo.png"))
                        .paint_at(ui, rect);
                    if _response.clicked(){
                        if uui.view_menu==ViewList::Setting{
                            uui.view_menu=ViewList::Main;
                        }else{
                            uui.view_menu=ViewList::Setting;
                        }
                    }
                });
            });
        })
        
    })
}
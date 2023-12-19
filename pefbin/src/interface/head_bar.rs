use eframe::{egui::{self, Ui, InnerResponse, RichText, Sense}, epaint::Vec2};

pub fn top_logo_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.columns(2, |columns|{
            columns[0].vertical_centered(|ui|{
                ui.add_space(40.0);
                ui.label(RichText::new("PEF TEST BOARD").strong().size(80.0));
            });
            columns[1].vertical_centered(|ui|{
                ui.add_space(40.0);
                ui.horizontal_top(|ui|{
                    ui.add_space(380.0);
                    // ui.label(RichText::new("PEF TEST BOAR").strong().size(50.0));
                    let (rect, _response) =ui.allocate_at_least(Vec2::new(400., 80.), Sense::hover());
                    // let (rect, _response) =ui.allocate_at_least(Vec2::new(150., 150.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/g54423.png"))
                        // .rounding(5.0)
                        .paint_at(ui, rect);
                    // egui::Image::new(egui::include_image!("../../files/pngegg1.png"))
                    //     // .rounding(5.0)
                    //     .tint(egui::Color32::LIGHT_BLUE)
                    //     .paint_at(ui, rect);
                    // egui::Image::new(egui::include_image!("../../files/pngegg.png"))
                    //     // .rounding(5.0)
                    //     .tint(egui::Color32::LIGHT_BLUE)
                    //     .paint_at(ui, rect);
                    
                   
                    // let temp = egui::ImageButton::new(egui::include_image!("../../files/g54423.png"));
                });
            });
        })
        
    })
}
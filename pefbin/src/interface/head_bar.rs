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
                    let (rect, _response) =ui.allocate_at_least(Vec2::new(400., 80.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/g54423.png"))
                        .paint_at(ui, rect);
                });
            });
        })
        
    })
}
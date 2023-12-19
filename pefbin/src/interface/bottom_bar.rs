use eframe::{egui::{self, Ui, InnerResponse, RichText, Layout, Sense}, epaint::{Color32, Vec2}, emath::Align};

pub fn bottom_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    // egui::Grid::new("some_unique_id").show(ui, |ui| {
    //     ui.vertical_centered(|ui|{
            // let faded_color = ui.visuals().window_fill();
            // let faded_color = |color: Color32| -> Color32 {
            //     use egui::Rgba;
            //     let t =  { 0.0 };
            //     egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            // };
            // ui.painter().rect_filled(
            //     ui.available_rect_before_wrap(),
            //     0.2,
            //     faded_color(Color32::LIGHT_BLUE),
            // );
            // ui.label("First row, first column");
    //     });
    //     ui.end_row();
    //     ui.label("Second row, first column");
    //     ui.end_row();
    // })
    ui.vertical_centered(|ui|{
        ui.horizontal_wrapped(|ui|{
            // ui.add_space(86.0);
            ui.label(RichText::new("").strong().size(60.0));
            // let faded_color = ui.visuals().window_fill();
            // let faded_color = |color: Color32| -> Color32 {
            //     use egui::Rgba;
            //     let t =  { 0.0 };
            //     egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            // };
            // ui.painter().rect_filled(
            //     ui.available_rect_before_wrap(),
            //     0.2,
            //     faded_color(Color32::DARK_GRAY),
            // );
            ui.columns(2, |columns|{
                columns[0].horizontal_centered(|ui|{
                    // ui.label(RichText::new("      STATE : ").strong().size(80.0));
                    ui.add(egui::Label::new(RichText::new("      STATE : ").color(Color32::from_rgb(36, 101, 255)).strong().size(80.0)));
                    ui.label(RichText::new("진행상태").strong().size(80.0));
                });
                columns[1].horizontal_centered(|ui|{
                    ui.add_space(320.);
                    let (one_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    // let (rect, _response) =ui.allocate_at_least(Vec2::new(150., 150.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/asdasd.png"))
                        // .rounding(5.0)
                        // .tint(egui::Color32::LIGHT_YELLOW)
                        .paint_at(ui, one_rect);
                    ui.add_space(20.);
                    let (two_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/warning.png"))
                        // .rounding(5.0)
                        // .tint(egui::Color32::LIGHT_BLUE)
                        .paint_at(ui, two_rect);
                    ui.add_space(20.);
                    let (temp_rect, _) =ui.allocate_at_least(Vec2::new(70., 70.), Sense::hover());
                    egui::Image::new(egui::include_image!("../../files/warning.png"))
                        // .rounding(5.0)
                        // .tint(egui::Color32::LIGHT_BLUE)
                        .paint_at(ui, temp_rect);
                });
            });
            
            
            
            // ui.text_edit_singleline(&mut self.url);
        });
        // ui.add_space(10.0);
        ui.horizontal_wrapped(|ui|{
            ui.add_space(86.0);
            ui.label(RichText::new("").strong().size(60.0));
            ui.label(RichText::new("PEF R&D과제명").strong().size(60.0));
        });
    })
}

fn circle_ui (ui:&mut Ui,color:Color32,size:Vec2){
    // let size = Vec2::splat(40.0);
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let rect = response.rect;
    let c = rect.center();
    let r = rect.width() / 2.0 - 1.0;
    // let color = Color32::from_gray(128);
    // let color = Color32::from_rgb(26, 255, 0);
    painter.circle_filled(c, r, color);
}
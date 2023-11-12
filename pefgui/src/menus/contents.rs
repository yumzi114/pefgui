
// use std::{collections::btree_map::Values, ops::Range};

// use eframe::{egui::{self,Ui, InnerResponse, RichText, Slider, collapsing_header::CollapsingState, Response, Sense}, epaint::{Color32, Stroke, vec2, Vec2}};
// use egui_extras::{StripBuilder, Size, Column, TableBuilder};
// use egui_plot::{Plot, Line};
// use super::{SetMenu,MenuStatus};



// pub fn content_view(ui: &mut Ui,ctx: &egui::Context, menu:&SetMenu)->InnerResponse<()>{
//     match menu {
//         SetMenu::EXECUTE=>{
//             egui::CentralPanel::default().show(ctx, |ui| {
//                 ui.label("EXECUTE");
//                 ui.columns(2, |columns|{
//                     columns[0].vertical_centered(|ui|{
                        
//                     });
//                     columns[1].vertical_centered(|ui|{
//                         let plot = Plot::new("id_source")
//                         // .show_x(false)
//                         // .show_y(false)
//                         // .clamp_grid(true)
//                         // .show_grid(false)
//                         .show_background(false);
//                         plot.show(ui, |plot_ui|{
                            
//                         });
//                     });
//                 });
                
                
//             })
            
//         },
//         SetMenu::HISTORY=>{
//             egui::CentralPanel::default().show(ctx, |ui| {
//                 let plot = Plot::new("id_source");
//                 plot.show(ui, |plot_ui|{
                    
//                 });
//             })
//         },
//     }
// }

// fn fontcolorbutton (&self, mode:bool)->egui::Button{
//     if !mode{
//         egui::Button::new(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN)).min_size(Vec2 { x: 30., y: 75. })    
//     }else {
//         egui::Button::new(RichText::new("OFF").strong().size(50.0).color(Color32::LIGHT_RED)).min_size(Vec2 { x: 30., y: 75. })
//     }
// }

use std::sync::{Arc, Mutex,atomic::{AtomicBool, Ordering}};
use eframe::{egui::{self,InnerResponse, Ui, RichText}, epaint::{Color32, Vec2}};
use egui_extras::{StripBuilder,Size, TableBuilder, Column};
mod drug;
mod contents;
mod execmenu;
mod plot_view;
use drug::DrugList;
use egui_plot::Plot;
use execmenu::ExecuteSettings;
// #[derive(Clone)]
pub struct MenuStatus {
    setmenu:SetMenu,
    drug:DrugList,
    pub exec:ExecuteSettings,
    // pub run:Arc<AtomicBool>,
    // pub run_time:Arc<Mutex<u64>>
}
#[derive(Default,PartialEq)]
pub enum SetMenu {
    #[default]
    EXECUTE,
    HISTORY
}
impl ::std::default::Default for MenuStatus {
    fn default() -> Self { 
        Self { 
            setmenu:Default::default(), 
            drug:Default::default(),
            exec:Default::default(),
            // run:Arc::new(AtomicBool::new(false)),
            // run_time:Arc::new(Mutex::new(0)),
    } }
}
// impl ::std::default::Default for MenuStatus {
//     fn default() -> Self { Self { 
//         ..Default::default()
//     } }
// }
impl MenuStatus {
    pub fn top_menu_bar(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        egui::containers::panel::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            ui.columns(4, |columns|{
                columns[0].vertical_centered_justified(|ui|{
                    ui.add_space(40.0);
                    if ui.add_sized([0., 80.], self.fontcolorbutton("MENU",SetMenu::EXECUTE)).clicked(){
                        self.setmenu=SetMenu::EXECUTE;
                    };
                    ui.add_space(10.0);
                });
                columns[1].vertical_centered_justified(|ui|{
                    ui.add_space(40.0);
                    if ui.add_sized([0., 80.], self.fontcolorbutton("HISTORY",SetMenu::HISTORY)).clicked(){
                        self.setmenu=SetMenu::HISTORY;
                    };
                    // if ui.button(RichText::new("HISTORY").strong().size(40.0)).clicked(){
                    //     self.setmenu=SetMenu::HISTORY;
                    // };
                    ui.add_space(10.0);
                });
                columns[2].vertical_centered_justified(|ui|{
                    ui.add_space(40.0);
                    ui.add_space(10.0);
                });
                columns[3].vertical_centered_justified(|ui|{
                    ui.add_space(10.0);
                    ui.label(RichText::new("약품상태").strong().size(20.0));
                    ui.separator();
                    ui.horizontal_centered(|ui|{
                        self.drug.drug_table_view(ui, ctx);
                    });
                    ui.add_space(10.0);
                });
            });
        })
    }
    
    pub fn content_view(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        // content_view(ui,ctx,&self.setmenu)
        match self.setmenu {
            SetMenu::EXECUTE=>{
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.columns(2, |columns|{
                        columns[0].vertical_centered(|ui|{
                            ui.vertical_centered_justified(|ui|{
                                ui.add_space(30.0);
                                ui.push_id(1, |ui| {
                                    TableBuilder::new(ui)
                                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                                    .striped(true)
                                    .column(Column::remainder())
                                    .column(Column::remainder())
                                    .header(60.0, |mut header| {
                                        header.col(|ui| {
                                            ui.add_space(10.0);
                                            ui.label(RichText::new("OPTION").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                                        });
                                        header.col(|ui| {
                                            ui.add_space(10.0);
                                            ui.label(RichText::new("ON/OFF").strong().size(50.0).color(Color32::from_rgb(38, 150, 255)));
                                        });
                                    })
                                    .body(|mut body| {
                                        body.row(70.0, |mut row| {
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_option_lb(ui,ctx,self.exec.water,"원 수");
                                                // ui.label(RichText::new("원 수").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                            });
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_lb(ui,ctx,self.exec.water);
                                                // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                            });
                                        });
                                        body.row(70.0, |mut row| {
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_option_lb(ui,ctx,self.exec.medicine,"약 품");
                                                // ui.label(RichText::new("약 품").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                            });
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_lb(ui,ctx,self.exec.medicine);
                                                // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                            });
                                        });
                                        body.row(70.0, |mut row| {
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_option_lb(ui,ctx,self.exec.carbonated,"탄 산");
                                                // ui.label(RichText::new("탄 산").strong().size(50.0산).color(Color32::LIGHT_GREEN));
                                            });
                                            row.col(|ui| {
                                                ui.add_space(10.0);
                                                self.onoff_lb(ui,ctx,self.exec.carbonated);
                                                // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                            });
                                        });
                                        if self.exec.carbonated{
                                            body.row(70.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.add_space(10.0);
                                                    self.onoff_option_lb(ui,ctx,self.exec.carbonated,"탄산 수치");
                                                    // ui.label(RichText::new("탄 산").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                });
                                                match self.exec.carboption.auto {
                                                    true=>{
                                                        row.col(|ui| {
                                                            ui.add_space(10.0);
                                                            if self.exec.power.load(Ordering::Relaxed){
                                                                ui.label(RichText::new("AUTO").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                            }else {
                                                                ui.label(RichText::new("POWER OFF").strong().size(50.0).color(Color32::DARK_GRAY));
                                                            }
                                                        });
                                                    },
                                                    false=>{
                                                        row.col(|ui| {
                                                            ui.add_space(10.0);
                                                            if self.exec.power.load(Ordering::Relaxed){
                                                                ui.label(RichText::new(self.exec.carboption.data.to_string()).strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                            }else {
                                                                ui.label(RichText::new("POWER OFF").strong().size(50.0).color(Color32::DARK_GRAY));
                                                            }
                                                        });
                                                    }
                                                }
                                                
                                            });
                                        }
                                        if self.exec.power.load(Ordering::Relaxed){
                                            body.row(70.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.add_space(10.0);
                                                    self.onoff_option_lb(ui,ctx,self.exec.carbonated,"POWER TIME");
                                                    // ui.label(RichText::new("탄 산").strong().size(50.0산).color(Color32::LIGHT_GREEN));
                                                });
                                                row.col(|ui| {
                                                    ui.add_space(10.0);
                                                    let messege = format!("{} sec",self.exec.power_time.lock().unwrap().to_string());
                                                    ui.label(RichText::new(messege).strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                    // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                });
                                            });
                                        }
                                        if self.exec.run.load(Ordering::Relaxed){
                                            body.row(70.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.add_space(10.0);
                                                    self.onoff_option_lb(ui,ctx,self.exec.carbonated,"RUN TIME");
                                                    // ui.label(RichText::new("탄 산").strong().size(50.0산).color(Color32::LIGHT_GREEN));
                                                });
                                                row.col(|ui| {
                                                    ui.add_space(10.0);
                                                    let messege = format!("{} sec",self.exec.run_time.lock().unwrap().to_string());
                                                    ui.label(RichText::new(messege).strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                    // ui.label(RichText::new("0").strong().size(50.0).color(Color32::LIGHT_GREEN));
                                                });
                                            });
                                        }
                                    });
                                });
                            });
                            
                            
                        });
                        columns[1].vertical_centered(|ui|{
                            let plot = Plot::new("id_source")
                            // .show_x(false)
                            // .show_y(false)
                            // .clamp_grid(true)
                            // .show_grid(false)
                            .show_background(false);
                            plot.show(ui, |plot_ui|{
                                
                            });
                        });
                    });
                    
                    
                });
                egui::TopBottomPanel::bottom("runpanel").show_separator_line(false).show(ctx, |ui| {
                    // ui.columns(num_columns, add_contents)
                    let ext = self.exec.run.clone();
                    ui.horizontal_wrapped(|ui|{
                        ui.add_space(50.0);
                        if ui.add_enabled(self.exec.power.load(Ordering::Relaxed), self.run_button()).clicked(){
                            
                            let temp = ext.load(Ordering::Relaxed);
                            // ext.get_mut()=!ext.load(Ordering::Relaxed);
                            ext.store(!temp, Ordering::Relaxed);
                            *self.exec.run_time.lock().unwrap()=0;
                            
                            // self.run=!self.run
                        };
                        // if ext.load(Ordering::Relaxed)&&self.exec.power{
                        //     let mem = self.run_time.clone();
                        //     // let time = mem.load(Ordering::Relaxed);
                        //     // let ssss = mem.lock().to_string();
                        //     ui.add_sized([0., 40.],egui::Label::new(RichText::new("탄 산").strong().size(50.0)));
                        //     ui.add_sized([0., 40.],egui::Label::new(RichText::new(&mem.lock().unwrap().to_string()[..]).strong().size(50.0)));
                        // }
                        // let sping = egui::widgets::Spinner::new();
                        
                        // let ss = egui::widgets::Spinner::new().size(60.0);
                        // ui.add_sized([0., 40.],);
                        // ui.add(egui::widgets::Spinner::new().size(60.0));
                        ui.add_space(20.0);
                        match self.exec.run.load(Ordering::Relaxed) {
                            true =>{
                                ui.add_sized([0., 40.],egui::Label::new(RichText::new("STATUS : ").size(70.0)));
                                ui.add_sized([0., 40.],egui::Label::new(RichText::new("MACHINE RUNING.. ").color(Color32::from_rgb(252, 32, 3)).size(70.0)));
                                ui.add(egui::widgets::Spinner::new().size(60.0));
                            },
                            false=>{
                                ui.add_sized([0., 40.],egui::Label::new(RichText::new("STATUS : ").size(70.0)));
                                ui.add_sized([0., 40.],egui::Label::new(RichText::new("WATING ").size(70.0)));
                            }
                        }

                        
                    });
                    
                })
                
            },
            SetMenu::HISTORY=>{
                egui::CentralPanel::default().show(ctx, |ui| {
                    let plot = Plot::new("id_source");
                    plot.show(ui, |plot_ui|{
                        
                    });
                })
            },
        }
    }
    pub fn bottom_menu_bar(&mut self,ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
        match self.setmenu {
            SetMenu::EXECUTE=>{
                egui::containers::panel::TopBottomPanel::bottom("bottom_menu").show_separator_line(false).show(ctx, |ui| {
                    // ui.label("EXECUTE BOTTOM");
                    ui.add_space(50.);
                    self.exec.exec_menu(ui, ctx);
                })
            },
            SetMenu::HISTORY=>{
                egui::containers::panel::TopBottomPanel::bottom("bottom_menu").show_separator_line(false).show(ctx, |ui| {
                    // ui.label("HISTORY BOTTOM");
                })
            },
        }
    }
    fn fontcolorbutton (&self, text:&str, mode:SetMenu)->egui::Button{
        if self.setmenu==mode{
            egui::Button::new(RichText::new(text).strong().size(30.0).color(Color32::LIGHT_GREEN))    
        }else {
            egui::Button::new(RichText::new(text).strong().size(30.0))
        }
    }
    fn run_button (&mut self)->egui::Button{
        if !self.exec.run.load(Ordering::Relaxed) && self.exec.power.load(Ordering::Relaxed){
            egui::Button::new(RichText::new("EXECUTE").strong().size(50.0).color(Color32::LIGHT_GREEN)).min_size(Vec2 { x: 250., y: 130. })    
        }else{
            egui::Button::new(RichText::new("STOP").strong().size(50.0).color(Color32::LIGHT_RED)).min_size(Vec2 { x: 250., y: 130. })
        }
    }
    pub fn onoff_lb(&self,ui: &mut Ui,ctx: &egui::Context, check:bool){
        if self.exec.power.load(Ordering::Relaxed){
            if check {
                ui.label(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN));
            }else {
                ui.label(RichText::new("OFF").strong().size(50.0).color(Color32::LIGHT_YELLOW));
            }
        }else {
            ui.label(RichText::new("POWER OFF").strong().size(50.0).color(Color32::DARK_GRAY));
        }
        
    }
    pub fn onoff_option_lb(&self,ui: &mut Ui,ctx: &egui::Context, check:bool,name:&str){
        if self.exec.power.load(Ordering::Relaxed) {
            ui.label(RichText::new(name).strong().size(50.0).color(Color32::LIGHT_GREEN));
            // ui.label(RichText::new("ON").strong().size(50.0).color(Color32::LIGHT_GREEN));
        }else {
            ui.label(RichText::new("POWER OFF").strong().size(50.0).color(Color32::DARK_GRAY));
        }
    }
}

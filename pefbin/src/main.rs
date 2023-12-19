#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::{self, Sense, Ui, RichText}, Theme, epaint::{Vec2, Color32}};
mod interface;
use interface::{UserUi,MenuList,keypad::keypad_view};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        default_theme:Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "PEF Board App",
        options,
        Box::new(|cc| {
            let mut app = PEFApp::new(cc);
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<PEFApp>::new(app)
        }),
    )
}

#[derive(Default)]
struct PEFApp {
    mainui:UserUi,
}

impl PEFApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        setup_custom_fonts(&cc.egui_ctx);
        
        Self::default()
    }
    pub fn new_windows(&mut self, ctx: &egui::Context){
        let title = match self.mainui.keypad.sellist {
            Some(MenuList::PulseFreq)=>"Set Pulse Freq Data",
            Some(MenuList::PulseTime)=>"Set Pulse Stop time",
            Some(MenuList::SetVoltage)=>"Set Votage Data",
            _=>"UNKNOW"
        };
        let temp = egui::Window::new(RichText::new(title).color(egui::Color32::LIGHT_YELLOW).strong().size(30.0)).title_bar(true).collapsible(false).resizable(false).default_size(Vec2::new(450., 450.)).id("pad".into()).open(&mut self.mainui.keypad.popon).vscroll(false);
            temp.show(ctx, |ui| {
                keypad_view(ui, ctx);
                
                // ui.code_editor(&mut self.open_win_code);
                // ui.add(egui::Label::new(RichText::new(title).color(Color32::from_rgb(36, 101, 255)).strong().size(80.0)));
            });
    }
    fn pop_window(&mut self, ui: &mut Ui,ctx: &egui::Context){
        self.new_windows(ctx);
    }   
}
// impl Default for PEFApp {
//     fn default() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
// }

impl eframe::App for PEFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.pop_window(ui, ctx);
            // let (rect, _response) =ui.allocate_at_least(Vec2::new(350., 100.), Sense::hover());
            // egui::Image::new(egui::include_image!("../files/g5442.png"))
            //     // .rounding(5.0)
            //     .tint(egui::Color32::LIGHT_BLUE)
            //     .paint_at(ui, rect);
            // ui.label("teasdasdxt");
            self.mainui.head_view(ui, ctx);
            self.mainui.content_view(ui, ctx);
            self.mainui.bottom_view(ui, ctx);
        });
    }
}

// fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
//     let img = include_bytes!("g5442.png");
//     let image = image::load_from_memory(img)?;
//     let size = [image.width() as _, image.height() as _];
//     let image_buffer = image.to_rgba8();
//     let pixels = image_buffer.as_flat_samples();
//     Ok(ColorImage::from_rgba_unmultiplied(
//         size,
//         pixels.as_slice(),
//     ))
// }

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../files/Pilseung_Gothic.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

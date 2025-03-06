#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use egui::{Context, TextEdit, Ui, Widget, WidgetText, Response, Pos2, Rect, Sense, Color32, FontId, TextStyle, FontFamily};
use eframe::{egui::{self, TextBuffer}, glow::SHADER_COMPILER};
use std::rc::Rc;
use std::cell::RefCell;

mod pages {
    pub mod login;
    pub mod workspace;
    pub mod page;
}
mod widgets {
    pub mod password_input;
}
use pages::{
    login::{self, LoginEventArgs, LoginPage},
    workspace::{self, WorkspacePage}
};
use pages::page::{
    Page, Pages
};


fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut my_app = Rc::new(RefCell::new(Box::<MyApp>::default()));
            let my_app_clone = Rc::clone(&my_app);
            my_app.borrow_mut().login_success = Box::new(|args| {
                my_app_clone.borrow_mut().page = Pages::Workspace;
            });
            let mut meh = Rc::clone(&my_app).borrow_mut();
            // let meh2 = meh as dyn 
            Ok(*meh)
        }),
    )
}


struct MyApp<'a> {
    login_page: LoginPage<'a>,
    workspace_page: WorkspacePage,
    page: Pages,

    login_success: Box<dyn FnMut(LoginEventArgs) + 'a>
}
impl<'a> MyApp<'a> {
    fn new() -> Self {
        let login_page = LoginPage::new();
        let workspace_page = WorkspacePage::new();
        Self {
            login_page : login_page,
            workspace_page : workspace_page,
            page : Pages::Login,
            login_success: Box::new(|_: LoginEventArgs| {
                print!("Okaaaaay")
            })

        }
    }
    // fn init(&'a mut self) {
        
    //     let on_login_success = move |args: LoginEventArgs| {
    //         (self.on_login_success)(args);
    //     };

    //     &mut self.login_page.on_login = Box::new(on_login_success);
        
    // }
    fn on_login_success(&self, args: LoginEventArgs) {
        if args.success {
            print!("Heeyyy");
        }
    }
}


impl<'a> Default for MyApp<'a> {
    fn default() -> Self {
        let login_page = LoginPage::new();
        let workspace_page = WorkspacePage::new();
        // let login_page_on_login = |args| { };
        Self {
            page : Pages::Login,
            login_page: login_page,
            workspace_page : workspace_page,
            login_success: Box::new(|_: LoginEventArgs| {})
        }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let active_page: &mut Box<dyn Page> = match &mut self.page {
        match &mut self.page {
            Pages::Login => self.login_page.update(ctx, _frame),
            Pages::Workspace => self.workspace_page.update(ctx, _frame),
            // Pages::Login => &mut self.login_page,
            // Pages::Workspace => &mut self.workspace_page
        };
        //active_page.update(ctx, _frame)

    }
}

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 let name_label = ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut self.name)
//                     .labelled_by(name_label.id);
//                 self.pwd.ui(ui)
//             });
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("Increment").clicked() {
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));

//             ui.image(egui::include_image!(
//                 "../assets/wtf.png"
//             ));
//         });
//     }
// }



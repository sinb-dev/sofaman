pub trait Page: eframe::App + 'static {
}

pub enum Pages {
    Login,
    Workspace
}
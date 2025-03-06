use std::any::Any;
pub trait Page: eframe::App + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub enum Pages {
    Login,
    Workspace
}
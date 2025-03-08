use std::rc::Rc;
use std::cell::RefCell;
use crate::AppState;
pub trait Page {
    fn from_app_state(app_state: Rc<RefCell<AppState>>) -> Self;
}

pub enum Pages {
    Login,
    Workspace,
    Accounts
}
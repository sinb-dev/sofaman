use crate::pages::page::Pages;
pub struct AppState {
    is_logged_in: bool,
    page: Pages
}
impl AppState {
    pub fn new() -> Self {
        Self {
            is_logged_in : false,
            page : Pages::Login
        }
    }
    pub fn logged_in(&mut self) {
        self.is_logged_in = true;
        self.page = Pages::Workspace;
    }
    pub fn _is_logged_in(&self) -> bool {
        self.is_logged_in
    }
    pub fn active_page(&self) -> Pages {
        match self.page {
            Pages::Login => Pages::Login,
            Pages::Workspace => Pages::Workspace,
        }
    }

}
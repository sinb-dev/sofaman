use crate::{accounting::{data::AccountService, models::Account}, data::sqlite::SqliteAccountQuery};

pub struct RequestParameters {
    pub filter: Option<String>,
    pub limit: usize,
    pub offset: usize
}
impl RequestParameters {
    pub fn new() -> Self {
       Self { filter: None, limit: 30, offset: 0 }
    }
}

pub trait Repository<Q> {
    fn query(&self) -> Q;
}

pub trait ServiceQuery<M> {
    fn filter(self, value: &str) -> Box<Self>;
    fn limit(self, limit: usize) -> Box<Self>;
    fn offset(self, offset: usize) -> Box<Self>;
    fn fetch(self) -> Vec<M>;
    fn with_id(self, id: usize) -> Option<M>;
}
pub struct ServiceQuery2 {
    pub request: RequestParameters
}
impl ServiceQuery2 {
    fn filter(mut self, value: &str) -> Self {
        self.request.filter = Some(String::from(value));
        self
    }
    fn limit(mut self, limit: usize) -> Self {
        self.request.limit = limit;
        self
    }
    fn offset(mut self, offset: usize) -> Self {
        self.request.offset = offset;
        self
    }
}
pub trait ServiceQuery2Trait {
    fn get_base(&self) -> &ServiceQuery2;
}
pub trait Service {

}

pub trait ServiceManager {
    fn accounts_service(&self) -> Box<dyn AccountService<dyn ServiceQuery2Trait>>;
}
pub struct DataContext {
    pub service_manager: Box<dyn ServiceManager>
}
// impl DataContext {
//     pub fn new(manager: Box<dyn ServiceManager>) -> Self {
//         Self {
//             service_manager: manager as Box<dyn ServiceManager>
//         }
//     }
// }
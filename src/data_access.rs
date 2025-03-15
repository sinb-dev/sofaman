use crate::{accounting::{data::AccountService, models::Account}};


pub trait ServiceLayer {
    fn accounting(&self) -> &Box<dyn AccountService>;
}
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
pub struct DataContext {
    pub services: Box<dyn ServiceLayer>
}

// pub trait Repository<Q> {
//     fn query(&self) -> Q;
// }

// pub trait ServiceQuery<M> {
//     fn filter(self, value: &str) -> Box<Self>;
//     fn limit(self, limit: usize) -> Box<Self>;
//     fn offset(self, offset: usize) -> Box<Self>;
//     fn fetch(self) -> Vec<M>;
//     fn with_id(self, id: usize) -> Option<M>;
// }

pub struct ServiceQuery {
    pub filter: Option<String>,
    pub limit: usize,
    pub offset: usize
}
impl ServiceQuery {
    pub fn new() -> Self {
        ServiceQuery {
            filter: None,
            limit: 30,
            offset: 0
        }
    }
    pub fn filter(mut self, value: &str) -> Self {
        self.filter = Some(String::from(value));
        self
    }
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}

// pub struct ServiceQuery {
//     pub request: RequestParameters
// }
// impl ServiceQuery {
//     pub fn new() -> Self {
//         ServiceQuery {
//             request: RequestParameters::new()
//         }
//     }
//     pub fn filter(mut self, value: &str) -> Self {
//         self.request.filter = Some(String::from(value));
//         self
//     }
//     pub fn limit(mut self, limit: usize) -> Self {
//         self.request.limit = limit;
//         self
//     }
//     pub fn offset(mut self, offset: usize) -> Self {
//         self.request.offset = offset;
//         self
//     }
// }


// impl DataContext {
//     pub fn new(manager: Box<dyn ServiceManager>) -> Self {
//         Self {
//             service_manager: manager as Box<dyn ServiceManager>
//         }
//     }
// }
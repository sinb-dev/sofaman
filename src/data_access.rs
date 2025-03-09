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

pub trait ServiceQuery<M> {
    fn filter(self, value: &str) -> Self;
    fn limit(self, limit: usize) -> Self;
    fn offset(self, offset: usize) -> Self;
    fn fetch(self) -> Vec<M>;
    fn with_id(self, id: usize) -> Option<M>;
}
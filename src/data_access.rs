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
pub trait ServiceRequest {
    fn filter(&mut self, value: &str) -> &mut Self;
    fn limit(&mut self, limit: usize) -> &mut Self;
    fn offset(&mut self, offset: usize) -> &mut Self;
}
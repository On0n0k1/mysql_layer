use crate::requests::{
    delete::Delete,
    get::Get,
    post::Post,
    put::Put,
    notimplemented::NotImplemented,
};

pub enum RequestType{
    DELETE,
    GET,
    POST,
    PUT,
    NOTIMPLEMENTED,
}

pub trait RequestTypeResult{
    fn get_request_type() -> RequestType;
    // This will return the http status code and the body
    // fn response<T>(action: &dyn Fn() -> ActionResult<T>) -> (u8, Option<String>);
    fn get_response(&self) -> (u16, Option<String>);
}

// Holds one of each possible requestType. Each attribute will switch to it's given behavior. If attribute is None, that means it's not implemented.
pub struct Requests<T> where T: RequestTypeResult{
    // There's no 'self' in the parameters of these functions. Which means that they are stateless.
    pub delete: Option<fn() -> T>,
    pub get: Option<fn() -> T>,
    pub post: Option<fn() -> T>,
    pub put: Option<fn() -> T>,
    pub not_implemented: fn() -> T,
}


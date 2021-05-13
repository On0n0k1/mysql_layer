use crate::requests::{
    delete::Delete,
    get::Get,
    post::Post,
    put::Put,
    notimplemented::NotImplemented,
    response::{ResponseType, Response},
};

// pub trait RequestResult{
//     // This will return the http status code and the body
//     // fn response<T>(action: &dyn Fn() -> ActionResult<T>) -> (u8, Option<String>);
//     fn get_response(&self) -> (u16, Option<String>);
// }


// pub enum RequestType{
//     DELETE,
//     GET,
//     POST,
//     PUT,
//     NOTIMPLEMENTED,
// }

// impl RequestType{
//     fn new(request: &str) -> Self{
//         match &request.to_lowercase()[..]{
//             "delete" => {RequestType::DELETE},
//             "get" => {RequestType::GET},
//             "post" => {RequestType::POST},
//             "put" => {RequestType::PUT},
//             _ => {RequestType::NOTIMPLEMENTED}

//         }
//     }
// }


// Holds one of each possible requestType. Each attribute will switch to it's given behavior. If attribute is None, that means it's not implemented.
pub struct Requests{
    // There's no 'self' in the parameters of these functions. Which means that they are stateless.
    pub delete: Option<fn() -> Response>,
    pub get: Option<fn() -> Response>,
    pub post: Option<fn() -> Response>,
    pub put: Option<fn() -> Response>,
}

impl Requests{
    fn do_request(&self, request: &str) -> (u16, String){
        let request = request.to_lowercase();
        // Does behavior only when pattern and condition are correct. Else return not implemented.
        match &request[..]{
            "delete" => {
                if let Some(func) = self.delete {
                    return func().get()
                }
            },
            "get" => {
                if let Some(func) = self.get {
                    return func().get()
                }
            },
            "post" => {
                if let Some(func) = self.post {
                    return func().get()
                }
            },
            "put" => {
                if let Some(func) = self.put {
                    return func().get()
                }
            },
            _ => {}
        }
        Response::new(ResponseType::NotImplemented501, "").get()
    }
}


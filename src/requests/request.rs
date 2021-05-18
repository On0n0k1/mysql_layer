use crate::requests::{
    // delete::Delete,
    // get::Get,
    // post::Post,
    // put::Put,
    // notimplemented::NotImplemented,
    response::{ResponseType, Response, ResponseExt},
};

use serde::{Deserialize, Serialize};

use crate::lambda::{
    message_trait::Message,
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
// pub struct Requests{
//     // There's no 'self' in the parameters of these functions. Which means that they are stateless.
//     pub delete: Option<fn() -> Response>,
//     pub get: Option<fn() -> Response>,
//     pub post: Option<fn() -> Response>,
//     pub put: Option<fn() -> Response>,
// }

// impl Requests{
//     fn do_request(&self, request: &str) -> (u16, String){
//         let request = request.to_lowercase();
//         // Does behavior only when pattern and condition are correct. Else return not implemented.
//         match &request[..]{
//             "delete" => {
//                 if let Some(func) = self.delete {
//                     return func().get()
//                 }
//             },
//             "get" => {
//                 if let Some(func) = self.get {
//                     return func().get()
//                 }
//             },
//             "post" => {
//                 if let Some(func) = self.post {
//                     return func().get()
//                 }
//             },
//             "put" => {
//                 if let Some(func) = self.put {
//                     return func().get()
//                 }
//             },
//             _ => {}
//         }
//         Response::new(ResponseType::NotImplemented501, "").get()
//     }
// }

pub trait Request{
    // Each T in these functions is a different type of struct. It will be implemented by each lambda.
    // fn post<F, T>() -> F where 
    //     F: Fn(String) -> T,
    //     for <'de> T: ResponseExt<'de, T>;
    //     // for <'de> T: Message<'de>;
    // // fn get<F, T>() -> F where 
    // //     for <'de> F: Fn(&'de str) -> Response<'de, T>,
    // //     for <'de> T: Message<'de>;
    // // fn delete<F, T>() -> F where 
    // //     for <'de> F: Fn(&'de str) -> Response<'de, T>,
    // //     for <'de> T: Message<'de>;
    // // fn put<F, T>() -> F where 
    // //     for <'de> F: Fn(&'de str) -> Response<'de, T:Message<'de>>,
    //     // for <'de> T: Message<'de>;

    // fn get<F, T>() -> F where 
    //     F: Fn(String) -> T,
    //     for <'de> T: ResponseExt<'de, T>;
    // fn delete<F, T>() -> F where 
    //     F: Fn(String) -> T,
    //     for <'de> T: ResponseExt<'de, T>;
    // fn put<F, T>() -> F where 
    //     F: Fn(String) -> T,
    //     for <'de> T: ResponseExt<'de, T>;

    fn post(message: Message) -> Response;
    fn get(message: Message) -> Response;
    fn delete(message: Message) -> Response;
    fn put(message: Message) -> Response;

    // fn get<T, R>(request: &'de str) -> Response<'de, T> where T: Message<'de>;
    // fn delete<'de, T>(request: &'de str) -> Response<'de, T> where T: Message<'de>;
    // fn put<'de, T>(request: &'de str) -> Response<'de, T> where T: Message<'de>;

    // fn do_request<'de, T>(&self, request_type: &'de str, request_body: &'de str) -> (u16, Option<String>) where T: Message<'de>{
    //     let request_type = request_type.to_lowercase();
    //     // Does behavior only when pattern and condition are correct. Else return not implemented.
    //     match &request_type[..]{
    //         "delete" => {
    //             return Self::delete::<T>(request_body).get();
    //         },
    //         "get" => {
    //             return Self::get::<T>(request_body).get();
    //         },
    //         "post" => {
    //             return Self::post::<T>(request_body).get();
    //         },
    //         "put" => {
    //             return Self::put::<T>(request_body).get();
    //         },
    //         _ => {}
    //     }
    //     Response::<'de, T>::new(ResponseType::NotImplemented501, None).get()
    // }

    // fn do_request<'de, F, T>(&self, request_type: &'de str, request_body: &'de str) -> (u16, Option<String>) where 
    //     F: Fn(String) -> Response<'de, T>,
    //     T: Message<'de>,
    // {
    //     let request_type = request_type.to_lowercase();
    //     // Does behavior only when pattern and condition are correct. Else return not implemented.
    //     match &request_type[..]{
    //         "delete" => {
    //             return Self::delete::<F, Response<'de, T>>()(request_body).get();
    //         },
    //         "get" => {
    //             return Self::get::<T>(request_body).get();
    //         },
    //         "post" => {
    //             return Self::post::<T>(request_body).get();
    //         },
    //         "put" => {
    //             return Self::put::<T>(request_body).get();
    //         },
    //         _ => {}
    //     }
    //     Response::<'de, T>::new(ResponseType::NotImplemented501, None).get()
    // }
    fn do_request(&self, request_type: &str, request_body: &str) -> (u16, Option<String>) 
    {
        let request_type = request_type.to_lowercase();
        let message = Message::new_json(request_body);
        // Does behavior only when pattern and condition are correct. Else return not implemented.
        match &request_type[..]{
            "delete" => {
                return Self::delete(message).get();
            },
            "get" => {
                return Self::get(message).get();
            },
            "post" => {
                return Self::post(message).get();
            },
            "put" => {
                return Self::put(message).get();
            },
            _ => {}
        }
        Response::new(ResponseType::NotImplemented501, None).get()
    }
}

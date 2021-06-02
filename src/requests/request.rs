use crate::requests::{
    response::{ResponseType, Response, ResponseExt},
    gateway::Full,
};

// use serde::{Serialize, Deserialize};

use crate::lambda::{
    message::Message,
};



// #[derive(Serialize, Deserialize)]
// struct RequestMessage{
//     method: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     body: Option<String>,
// }



// impl RequestMessage{
//     fn new(aws_event: String) -> Result<RequestMessage, String> {
//         println!("Aws event is: {}", &aws_event[..]);

//         let full = Full::new(aws_event)?;

//         let method = full.event.requestContext.http.method.clone();
//         let body = full.event.body.clone();

//         Ok(RequestMessage{
//             method,
//             body,
//         })
//     }
// }


pub trait Action{
    fn post(message: Option<Message>) -> Response;
    fn get(message: Option<Message>) -> Response;
    fn delete(message: Option<Message>) -> Response;
    fn put(message: Option<Message>) -> Response;


    // This can be called when testing with rust
    fn do_request(&self, request_type: &str, request_body: Option<String>) -> (u32, Option<String>) 
    {
        let request_type = request_type.to_lowercase();
        // let message = Message::new_json(request_body);
        let message = match request_body {
            None => None,
            Some(value) => { Some(Message::new_json(&value[..])) },
        };
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

    // this will be called when running aws events
    fn parse_and_do(&self, aws_event: String) -> (u32, Option<String>) {
        println!("Aws event is: {}", &aws_event[..]);

        let full = match Full::new(aws_event) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return Response::new(ResponseType::BadRequest400, None).get();
            }
        };

        let request_type = full.event.requestContext.http.method.clone();
        let request_body = full.event.body.clone();

        Self::do_request(&self, &request_type[..], request_body)

        // Ok(RequestMessage{
        //     method,
        //     body,
        // })
    }
}


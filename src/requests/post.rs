///
/// OK(200): Does some processing but no resource was created. Body contains the result of the operation.
/// CREATED(201): Resource created. Must include Location Header URI for the created element(not yet implemented). Response body must contain a representation of the resource.
/// NOCONTENT(204): Does some processing, no resource created, no response body. (I don't know what is this for)
/// BADREQUEST(400): Invalid data for the request. Body may contains extra information about the error.
/// 

use crate::requests::request::{
    RequestType,
    RequestTypeResult,
};

pub enum Post{
    ACCEPTED,
    OK(String),
    CREATED(String),
    NOCONTENT,
    BADREQUEST(Option<String>),
}

impl RequestTypeResult for Post{

    fn get_request_type() -> RequestType {
        RequestType::POST
    }

    fn get_response(&self)-> (u16, Option<String>){
        match self {
            Post::ACCEPTED => {(202, None)},
            Post::OK(body) => {(200, Some(body.clone()))},
            Post::CREATED(body) => {(201, Some(body.clone()))},
            Post::NOCONTENT => {(204, None)},
            Post::BADREQUEST(body) => {(400, body.clone())},
        }
    }
}

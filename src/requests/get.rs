///
/// OK(200): Successful get.
/// ACCEPTED(202): Request accepted, but still being processed. (I'll have to implement async for this later)
/// NOTFOUND(404): Resource doesn't exist.
/// NOTACCEPTABLE(406): An accept header contains list of media types that client will accept. This error is returned when server doesn't support any of these (not implemented).
/// UNSUPPORTED(415): A Content-Type header specifies the format of the message. This error is returned if the type is unsupported (not implemented). 
/// 

use crate::requests::request::{
    RequestType,
    RequestTypeResult,
};

pub enum Get {
    ACCEPTED,
    OK(String),
    NOTFOUND,
}

impl RequestTypeResult for Get{
    fn get_request_type() -> RequestType{
        RequestType::GET
    }

    fn get_response(&self) -> (u16, Option<String>){
        match self{
            Get::ACCEPTED => {(202, None)},
            Get::OK(body) => {(200, Some(body.clone()))},
            Get::NOTFOUND => {(404, None)},
        }
    }
}


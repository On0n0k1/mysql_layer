///
/// All not implemented requests go to here.
/// 501 No body.
/// 

use crate::requests::request::{
    RequestType,
    RequestTypeResult,
};

pub enum NotImplemented{
    NOTIMPLEMENTED
}


impl RequestTypeResult for NotImplemented{
    fn get_request_type() -> RequestType{
        RequestType::NOTIMPLEMENTED
    }

    fn get_response(&self) -> (u16, Option<String>){
        match self{
            NotImplemented::NOTIMPLEMENTED => {(501, None)},
        }
    }
}
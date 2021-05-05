///
/// 
/// ACCEPTED(202): Request accepted, but still being processed. (I'll have to implement async for this later)
/// SUCCESSFUL(204): Request successful and the there's no data remaining. Body will be None.
/// NOTFOUND(404): Resource doesn't exist.
/// 

use crate::requests::request::{
    RequestType,
    RequestTypeResult,
};

pub enum Delete {
    ACCEPTED,
    SUCCESSFUL,
    NOTFOUND,
}


impl RequestTypeResult for Delete{

    fn get_request_type() -> RequestType{
        RequestType::DELETE
    }

    // pub fn response<T>(action: &dyn Fn() -> ActionResult<T>) -> (u8, Option<String>){}
    fn get_response(&self) -> (u16, Option<String>){
        match self{
            Delete::ACCEPTED => {
                // Remember to include into header "Location" with the endpoint to get the status
                (202, None)
            },
            Delete::SUCCESSFUL => {(204, None)},
            Delete::NOTFOUND => {(404, None)},
        }
    }

}


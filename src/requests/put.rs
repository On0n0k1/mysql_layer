///
/// 
/// CREATED(201): Resource created. Must include Location Header URI for the created element(not yet implemented). Response body must contain a representation of the resource.
/// OK(200): Resource updated. Contains body.
/// NOCONTENT(204): Resource updated. No body.
/// CONFLICT(409): Unable to update resource.
/// 

use crate::requests::request::{
    RequestType,
    RequestTypeResult,
};


pub enum Put{
    ACCEPTED,
    OK(String),
    CREATED(String),
    NOCONTENT,
    CONFLICT(Option<String>),
}

impl RequestTypeResult for Put{
    fn get_request_type() -> RequestType {
        RequestType::PUT
    }

    fn get_response(&self) -> (u16, Option<String>) {
        match self {
            Put::ACCEPTED => {(202, None)},
            Put::OK(body) => {(200, Some(body.clone()))},
            Put::CREATED(body) => {(201, Some(body.clone()))},
            Put::NOCONTENT => {(204, None)},
            Put::CONFLICT(body) => {(409, body.clone())},
        }
    }
}
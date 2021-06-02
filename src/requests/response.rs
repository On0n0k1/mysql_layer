///
/// Delete
/// ACCEPTED(202): Request accepted, but still being processed. (I'll have to implement async for this later)
/// SUCCESSFUL(204): Request successful and the there's no data remaining. Body will be None.
/// NOTFOUND(404): Resource doesn't exist.
/// 
/// Get
/// OK(200): Successful get.
/// ACCEPTED(202): Request accepted, but still being processed. (I'll have to implement async for this later)
/// NOTFOUND(404): Resource doesn't exist.
/// NOTACCEPTABLE(406): An accept header contains list of media types that client will accept. This error is returned when server doesn't support any of these (not implemented).
/// UNSUPPORTED(415): A Content-Type header specifies the format of the message. This error is returned if the type is unsupported (not implemented). 
/// 
/// Post
/// OK(200): Does some processing but no resource was created. Body contains the result of the operation.
/// CREATED(201): Resource created. Must include Location Header URI for the created element(not yet implemented). Response body must contain a representation of the resource.
/// NOCONTENT(204): Does some processing, no resource created, no response body. (I don't know what is this for)
/// BADREQUEST(400): Invalid data for the request. Body may contains extra information about the error.
/// 
///
/// Put
/// CREATED(201): Resource created. Must include Location Header URI for the created element(not yet implemented). Response body must contain a representation of the resource.
/// OK(200): Resource updated. Contains body.
/// NOCONTENT(204): Resource updated. No body.
/// CONFLICT(409): Unable to update resource.
/// 

// use std::marker::PhantomData;
use std::fmt;

// use serde::{Deserialize, Serialize};
use crate::lambda::{
    message::Message,
    // lambda::Lambda,
};


// source for below: https://www.softwaretestinghelp.com/rest-api-response-codes/
pub enum ResponseType{
    // These are temporary Responses
    Continue100,
    SwitchingProtocols101,
    Processing102,
    
    // The client accepts the Request, being processed successfully at the server.
    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    IMused226,

    //Most of the codes related to this series are for URL Redirection.
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    CheckOther303,
    NotModified304,
    UseProxy305,
    SwitchProxy306,
    TemporaryRedirect307,
    PermanentRedirect308,

    // These are specific to client-side error.
    BadRequest400,
    Unauthorised401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthenticationRequired407,
    RequestTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    URITooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,

    // These are specific to the server-side error.
    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HTTPVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthenticationRequired511,
}

// Get the response code associated with the value;.
impl ResponseType{
    fn get(&self) -> u32 {
        match self{
            ResponseType::Continue100 => {100},
            ResponseType::SwitchingProtocols101 => {101},
            ResponseType::Processing102 => {102},
            
            ResponseType::Ok200 => {200},
            ResponseType::Created201 => {201},
            ResponseType::Accepted202 => {202},
            ResponseType::NonAuthoritativeInformation203 => {203},
            ResponseType::NoContent204 => {204},
            ResponseType::ResetContent205 => {205},
            ResponseType::PartialContent206 => {206},
            ResponseType::MultiStatus207 => {207},
            ResponseType::AlreadyReported208 => {208},
            ResponseType::IMused226 => {226},
            
            ResponseType::MultipleChoices300 => {300},
            ResponseType::MovedPermanently301 => {301},
            ResponseType::Found302 => {302},
            ResponseType::CheckOther303 => {303},
            ResponseType::NotModified304 => {304},
            ResponseType::UseProxy305 => {305},
            ResponseType::SwitchProxy306 => {306},
            ResponseType::TemporaryRedirect307 => {307},
            ResponseType::PermanentRedirect308 => {308},

            ResponseType::BadRequest400 => {400},
            ResponseType::Unauthorised401 => {401},
            ResponseType::PaymentRequired402 => {402},
            ResponseType::Forbidden403 => {403},
            ResponseType::NotFound404 => {404},
            ResponseType::MethodNotAllowed405 => {405},
            ResponseType::NotAcceptable406 => {406},
            ResponseType::ProxyAuthenticationRequired407 => {407},
            ResponseType::RequestTimeout408 => {408},
            ResponseType::Conflict409 => {409},
            ResponseType::Gone410 => {410},
            ResponseType::LengthRequired411 => {411},
            ResponseType::PreconditionFailed412 => {412},
            ResponseType::PayloadTooLarge413 => {413},
            ResponseType::URITooLong414 => {414},
            ResponseType::UnsupportedMediaType415 => {415},
            ResponseType::RangeNotSatisfiable416 => {416},
            ResponseType::ExpectationFailed417 => {417},
            ResponseType::ImATeapot418 => {418},
            ResponseType::MisdirectedRequest421 => {421},
            ResponseType::UnprocessableEntity422 => {422},
            ResponseType::Locked423 => {423},
            ResponseType::FailedDependency424 => {424},
            ResponseType::UpgradeRequired426 => {426},
            ResponseType::PreconditionRequired428 => {428},
            ResponseType::TooManyRequests429 => {429},
            ResponseType::RequestHeaderFieldsTooLarge431 => {429},
            ResponseType::UnavailableForLegalReasons451 => {451},

            ResponseType::InternalServerError500 => {500},
            ResponseType::NotImplemented501 => {501},
            ResponseType::BadGateway502 => {502},
            ResponseType::ServiceUnavailable503 => {503},
            ResponseType::GatewayTimeout504 => {504},
            ResponseType::HTTPVersionNotSupported505 => {505},
            ResponseType::VariantAlsoNegotiates506 => {506},
            ResponseType::InsufficientStorage507 => {507},
            ResponseType::LoopDetected508 => {508}, 
            ResponseType::NotExtended510 => {510},
            ResponseType::NetworkAuthenticationRequired511 => {511},
        }
    }
}

pub trait ResponseExt{
    fn get(&self) -> (u32, Option<String>);
}


// #[derive(Clone, Serialize, Deserialize)]
pub struct Response{
    response_code: u32,
    body: Option<Message>,
}


// Essa trait que eu criei usa as caracteristicas de Serialize e Deserialize para
// disponibilizar funções mais simples de conversão de json.
// impl<'de, T> Message<'de> for Response<T>{}

// impl<'de, T> Response<'de, T> where T: Message<'de>{
//     pub fn new(response_type: ResponseType, body: Option<T>) -> Self where T: Message<'de>{
//         Response{
//             response_code: response_type.get(),
//             body,
//             phantom: PhantomData,
//         }
//     }

//     // pub fn get(&self) -> (u16, Option<String>) where T: Message<'de>{
//     //     let code = self.response_code;
//     //     let body = self.body.clone();
//     //     let body: Option<String> = match body {
//     //         None => None,
//     //         Some(value) => Some(value.into_json()),
//     //     };
//     //     // let body = self.body.into_json();
//     //     (code, body)
//     // }
// }

impl Response{
    pub fn new(response_type: ResponseType, body: Option<Message>) -> Self{
        Response{
            response_code: response_type.get(),
            body,
        }
    }
}


impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let code = self.response_code;
        // let body = self.body.into_json();
        let (code, body) = self.get();
        match body{
            None => write!(f, "{{\n  \"statusCode\": {},\n}}\n", code),
            Some(value) => write!(f, "{{\n  \"statusCode\": {},\n  \"body\": {}\n}}\n", code, value),
        }

        // write!(f, "{{\n   code: {},\n   body: {}\n}}\n", code, body)
    }
}

// impl<'de, T> ResponseExt<'de, T> for Response<'de, T> where T: Message<'de>{
//     fn get(&self) -> (u16, Option<String>) where T: Message<'de>{
//         let code = self.response_code;
//         let body = self.body.clone();
//         let body: Option<String> = match body {
//             None => None,
//             Some(value) => Some(value.into_json()),
//         };
//         // let body = self.body.into_json();
//         (code, body)
//     }
// }

impl ResponseExt for Response{
    fn get(&self) -> (u32, Option<String>) {
        let code = self.response_code;
        let body = self.body.clone();
        let body: Option<String> = match body {
            None => None,
            Some(value) => Some(value.get_json()),
        };
        // let body = self.body.into_json();
        (code, body)
    }
}


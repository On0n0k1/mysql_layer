use serde::{Deserialize, Serialize};

use mysql::*;

use crate::{
    database::{
        dao::DAO,
    },
    lambda::{
        funcionario::{
            funcionario::Funcionario,
        },
        message::Message,
    },
    requests::response::{
        Response,
        ResponseType,
    }
};


#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRequest{
    ById{id: u32,},
    ByValue(Funcionario,),
}


pub fn request_delete(message: Message) -> Response{
    // let mut message = Message::new_json(&request[..]);
    let delete: Option<DeleteRequest> = message.get_value::<DeleteRequest>();
    let delete = match delete{
        None => { return Response::new(ResponseType::BadRequest400, None)},
        Some(value) => {value},
    };

    let result = match delete{
        DeleteRequest::ById{id} => {
            Funcionario::dao_remove_id(id, None)
        },
        DeleteRequest::ByValue(element) => {
            // remove element dao function not implemented yet
            Funcionario::dao_remove_id(Funcionario::get_id(&element), None)
            // Funcionario::dao_remove_element(element, None)
        },
    };

    match result{
        Err(err) => {
            println!("Internal Error Ocurred when Processing Delete operation. \nerr: {}\nmessage: {}\n\n", err, message.get_json());
            return Response::new(ResponseType::InternalServerError500, None);
        },
        Ok(element) => {
            match element {
                None => {return Response::new(ResponseType::NotFound404, None);},
                Some(_) => {return Response::new(ResponseType::NoContent204, None);}
            }
        }
    }
}

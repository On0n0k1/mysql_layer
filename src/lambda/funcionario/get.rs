use serde::{Deserialize, Serialize};

use mysql::*;

use crate::{
    database::{
        dao::DAO,
    },
    lambda::{
        funcionario::{
            funcionario::Funcionario,
            list::{
                request_list,
                ListBody,
            },
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
pub enum ListGetRequest{
    Get{id: u32,},
    List{
        start: u32, 
        end: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u32>,
    },
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGetBody{
    List(ListBody),
    Get(GetBody),
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GetBody{
    pub funcionario: Option<Funcionario>,
}


pub fn request_get(message: Message) -> Response{
    let list_get: Option<ListGetRequest> = message.get_value::<ListGetRequest>();
    let list_get = match list_get{
        None => { return Response::new(ResponseType::BadRequest400, None)},
        Some(value) => {value},
    };

    let response = match list_get{
        ListGetRequest::List{start, end, limit} => { request_list(Some((start, end)), limit)}
        ListGetRequest::Get{id} => {
            let func  = Funcionario::dao_get(id).unwrap();
            match func{
                None => { Response::new( ResponseType::NotFound404, None)},
                Some(value) => {
                    let body = ListGetBody::Get(GetBody{ funcionario: Some(value) });
                    let conversion = Message::new_value::<ListGetBody>(body);

                    match conversion {
                        Err(err) => {
                            println!("Failed to convert get request: Err: {}\n", err);
                            Response::new(ResponseType::BadRequest400, None)
                        },
                        Ok(value) => { Response::new(ResponseType::Ok200, Some(value))}
                    }
                },
            }
        },
    };
    response
}

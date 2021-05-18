use serde::{Deserialize, Serialize};

use mysql::*;

use crate::{
    database::{
        dao::DAO,
        // db::DB,
    },
    lambda::{
        funcionario::{
            funcionario::Funcionario,
            list::{
                list,
                ListBody,
            },
        },
        message_trait::Message,
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

// impl<'de> Message<'de> for ListGetRequest{}


#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGetBody{
    List(ListBody),
    Get(GetBody),
}

// impl<'de> Message<'de> for ListGetBody{}


#[derive(Clone, Serialize, Deserialize)]
pub struct GetBody{
    pub funcionario: Option<Funcionario>,
}


// impl<'de> Message<'de> for GetBody{}


pub fn get(request: &str) -> Response{
    let mut message = Message::new_json(&request[..]);
    let list_get: Option<ListGetRequest> = message.get_value::<ListGetRequest>();
    let list_get = match list_get{
        None => { return Response::new(ResponseType::BadRequest400, None)},
        Some(value) => {value},
    };

    match list_get{
        ListGetRequest::List{start, end, limit} => { return list(Some((start, end)), limit)}
        ListGetRequest::Get{id} => {
            let func  = Funcionario::get(id).unwrap();
            match func{
                None => {
                    return Response::new(
                        ResponseType::NotFound404,
                        None,
                    );
                },
                _ => {},
            }
            let body = ListGetBody::Get(GetBody{funcionario: func,},);
            let conversion = message.store_json::<ListGetBody>(body);

            let response = match conversion {
                Err(err) => {
                    println!("Failed to convert get request: Err: {}\n", err);
                    Response::new(
                        ResponseType::BadRequest400,
                        None,
                    )
                },
                Ok(_) => {
                    Response::new(
                        ResponseType::Ok200,
                        Some(message),
                    )
                }
            };
            // if let conversion = Err(err) {
                
            // };
            // let response = Response::new(
            //     ResponseType::Ok200,
            //     Some(body),
            // );

            return response;
        },
    }
}

use serde::{Deserialize, Serialize};

use mysql::*;

use crate::{
    database::{
        dao::DAO,
        db::DB,
    },
    lambda::{
        funcionario::{
            funcionario::Funcionario,
            get::{
                ListGetBody,
                ListGetRequest,
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
pub struct ListBody{
    pub funcionario: Vec<Funcionario>,
}


// impl<'de> Message<'de> for ListBody{}

pub fn list(start_end: Option<(u32, u32)>, limit: Option<u32>) -> Response{
    // let where_z = match start_end{
    //     None => None,
    //     Some(value) => Some(format!("((id < {}) AND (id > {}))", value.0, value.1)),
    // };

    let vec = match start_end{
        None => Funcionario::list(None, limit).unwrap(),
        Some(value) => {
            let where_z = format!("((id >= {}) AND (id < {}))", value.0, value.1);
            Funcionario::list(Some(&where_z[..]), limit).unwrap()
        }
    };

    if vec.len() == 0 {
        return Response::new(
            ResponseType::NotFound404,
            None,
        );
    }
    
    // let vec = Funcionario::list(where_z,limit).unwrap();
    let body = ListGetBody::List(ListBody{funcionario: vec,});

    let body = Message::new_value::<ListGetBody>(body);
    
    let response = match body {
        Ok(message) => { 
            Response::new(
                ResponseType::Ok200,
                Some(message),
            )
        },
        Err(err) => {
            println!("Failed to convert get request: Err: {}\n", err);
            Response::new(
                ResponseType::InternalServerError500,
                None,
            )
        },
    };
    
    // let response = Response::new(
    //     ResponseType::Ok200,
    //     Some(body),
    // );

    // let (code, func) = response.get();

    // println!("{}\n   code: {}\n   body: {}\n{}", "{", code, func, "}");
    response
}
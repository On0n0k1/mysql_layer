use serde::{Deserialize, Serialize};
// use serde_json::Result;

// use mysql::*;
// use mysql::prelude::{
//     Queryable,
//     FromRow,
//     FromValue,
// };

use crate::lambda::{
    message::Message,
    funcionario::{
        get::request_get,
        post::request_post,
        delete::request_delete,
    },
};

use crate::database::db::DB;
use crate::database::dao::{
    DAO,
};

use crate::requests::{
    request::Request,
    response::{
        ResponseType,
        Response,
    },
};


// Define o tipo de objeto funcionario. As propriedades herdadas pelo derive fazem o seguinte:
// Clone permite manualmente criar copias do objeto.
// Serialize e Deserialize permitem "conversão" e "desconversão" de json.
#[derive(Clone, Serialize, Deserialize)]
pub struct Funcionario{
    // defaults to 0 if value not included in message
    #[serde(default)]
    pub id: u32,
    pub idade: u32,
    pub nome: String,
    pub cargo: String,
}


impl Funcionario{
    pub fn new(
        id: u32,
        idade: u32,
        nome: &str,
        cargo: &str,
    ) -> Funcionario
    {
        Funcionario {
            id,
            idade,
            nome: String::from(nome),
            cargo: String::from(cargo),
        }
    }  
}


impl DAO<Funcionario> for Funcionario{
    type Item = (u32, u32, String, String);

    // db_name = None makes use default name from .env
    fn get_db_name() -> Option<String> {
        let db = DB::new();
        Some(db.link.get_db_name())
    }

    fn get_table_name() -> String {
        String::from("funcionarios")
    }

    fn get_columns() -> Vec<String> {
        vec![
            String::from("id"),
            String::from("idade"),
            String::from("nome"),
            String::from("cargo"),
        ]
    }

    fn get_columns_values(element: &Funcionario) -> Vec<(String, String)> {
        let id = element.id.clone();
        let idade = element.idade.clone();
        let nome = element.nome.clone();
        let cargo = element.cargo.clone();
        
        vec![
            (String::from("id"), format!("{}", id)),
            (String::from("idade"), format!("{}", idade)),
            (String::from("nome"), format!("\'{}\'", nome)),
            (String::from("cargo"), format!("\'{}\'", cargo)),
        ]
    }

    fn get_id(element: &Funcionario) -> u32 {
        element.id.clone()
    }

    fn set_id(element: &mut Funcionario, id: u32) {
        element.id = id;
    }
    
    fn get_constructor() -> Box<dyn FnMut(Self::Item)-> Self> 
    {
        Box::new(
            |(id, idade, nome, cargo)| -> Funcionario {
                Funcionario {
                    id,
                    idade,
                    nome,
                    cargo,
                }
            }
        )
    }
}

impl Request for Funcionario{
    fn post(message: Message) -> Response{
        return request_post(message)
    }

    fn get(message: Message) -> Response{
        return request_get(message)
    }

    fn delete(message: Message) -> Response {
        // Response::new(ResponseType::NotImplemented501, None)
        return request_delete(message)
    }

    fn put(_message: Message) -> Response {
        Response::new(ResponseType::NotImplemented501, None)
    }
}

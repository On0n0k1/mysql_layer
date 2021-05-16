
use serde::{Deserialize, Serialize};
use serde_json::Result;

use mysql::*;
use mysql::prelude::{
    Queryable,
    FromRow,
    FromValue,
};

use crate::lambda::{
    message_trait::Message,
};

use crate::database::db::DB;
use crate::database::dao::{
    // DaoTrait,
    DAO,
};

use crate::requests::response::{
    ResponseType,
    Response,
};

// Define o tipo de objeto funcionario. As propriedades herdadas pelo derive fazem o seguinte:
// Clone permite manualmente criar copias do objeto.
// Serialize e Deserialize permitem "conversão" e "desconversão" de json.
#[derive(Clone, Serialize, Deserialize)]
pub struct Funcionario{
    pub id: u32,
    pub idade: u32,
    pub nome: String,
    pub cargo: String,
}

impl Funcionario{
    pub fn new(
        id: u32,
        idade: u32,
        nome: String,
        cargo: String,
    ) -> Funcionario
    {
        Funcionario {
            id,
            idade,
            nome,
            cargo,
        }
    }

    // fn get() -> Get {
    //     let func = Funcionario{
    //         id: 0,
    //         idade: 20,
    //         nome: String::from("nome"),
    //         cargo: String::from("cargoo")
    //     };
    //     Get::OK(func.into_json())
    // }
    pub fn into_response<'de>() -> Response<'de, Funcionario> {
        let func = Funcionario{
            id: 0,
            idade: 20,
            nome: String::from("nome"),
            cargo: String::from("cargoo")
        };

        Response::new(ResponseType::Ok200, func)
    }

    
}

// Essa trait que eu criei usa as caracteristicas de Serialize e Deserialize para
// disponibilizar funções mais simples de conversão de json.
impl<'de> Message<'de> for Funcionario{}

// Essa trait lambda recebe o parametro Requests e distribui o tipo de request para a função associada.
// impl Lambda for Funcionario{
//     fn get_implemented_requests() -> Requests {
//         Requests {
//             delete: None,
//             get: Some(Self::get),
//             post: None,
//             put: None,
//         }
//     }
// }

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
        let cargo = element.nome.clone();
        
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

    // Column order from get_columns_values must be the same as the constructor
    // fn get_constructor<F, T>() -> F where 
    //     T: FromRow,
    //     F: FnMut(T) -> Funcionario,
    // {
        
    //     move |(id, idade, nome, cargo)| {
    //         let id: u32 = id;
    //         let idade: u32 = idade;
    //         let nome: String = String::from(nome);
    //         let cargo: String = String::from(cargo);
    //         let func = Funcionario{
    //             id,
    //             idade,
    //             nome,
    //             cargo,
    //         };
    //         func
    //     }

    // }

    // fn get_constructor() -> Box<dyn FnMut(T) -> Self>;
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




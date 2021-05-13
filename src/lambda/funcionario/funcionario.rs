
use serde::{Deserialize, Serialize};
use serde_json::Result;


use crate::lambda::{
    message_trait::Message,
    // lambda::Lambda,
};
// use crate::requests::request::{
//     Requests,
// };

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

impl Funcionario{
    // fn get() -> Get {
    //     let func = Funcionario{
    //         id: 0,
    //         idade: 20,
    //         nome: String::from("nome"),
    //         cargo: String::from("cargoo")
    //     };
    //     Get::OK(func.into_json())
    // }
    pub fn get<'de>() -> Response<'de, Funcionario> {
        let func = Funcionario{
            id: 0,
            idade: 20,
            nome: String::from("nome"),
            cargo: String::from("cargoo")
        };

        Response::new(ResponseType::Ok200, func)
    }
}




// use lib::lambda::message_trait::Message;

// use lib::database::db::DB;
// use lib::requests::response::{
//     ResponseType,
//     Response,
// };

// use lib::database::dao::{
//     DAO,
// };


use serde::{Deserialize, Serialize};

use mysql::*;

use lib::{
    database::{
        dao::DAO,
        db::DB,
    },
    lambda::{
        funcionario::funcionario::Funcionario,
        message_trait::Message,
    },
    requests::response::{
        Response,
        ResponseType,
    }
};


// This will be what message will be turned into the body json
#[derive(Clone, Serialize, Deserialize)]
pub struct Body{
    pub funcionario: Vec<Funcionario>,
}

impl<'de> Message<'de> for Body{}

fn main() {
    let mut db = DB::new();

    // let vec = db.initiate_transaction(&|tx| -> Result<Vec<Funcionario>> {
    //         let vec = DB::select_x_from_y_where_z_map(
    //             tx,
    //             "*",
    //             "funcionarios",
    //             None,
    //             None,
    //             |(id, idade, nome, cargo)| {
    //                 Funcionario{
    //                     id,
    //                     idade,
    //                     nome,
    //                     cargo,
    //                 }
    //             },
    //         )?;
    //         Ok(vec)
        
    // }).unwrap();

    // let vec = DAO::<Funcionario>::get(None).unwrap();
    // let vec = Funcionario::new(0, 10, String::from("Nome"), String::from("cargo"));
    let vec = Funcionario::list(None).unwrap();

    // let vec = DAO::<Funcionario>::list(None);

    let body = Body{
        funcionario: vec,
    };

    let response = Response::new(
        ResponseType::Ok200,
        body,
    );

    let (code, func) = response.get();

    println!("{}\n   code: {}\n   body: {}\n{}", "{", code, func, "}");
    

}
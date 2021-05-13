use lib::lambda::funcionario::funcionario::Funcionario;
use lib::lambda::message_trait::Message;

use lib::db::table::Table;
use lib::db::db::DB;
use lib::requests::response::{
    ResponseType,
    Response,
};


use serde::{Deserialize, Serialize};


use mysql::*;


// This will be what message will be turned into the body json
#[derive(Clone, Serialize, Deserialize)]
pub struct Body{
    pub funcionario: Vec<Funcionario>,
}

impl<'de> Message<'de> for Body{}

fn main() {
    // let vec = Vec::new(["name", "cargo", "something"]);
    // let vec = Table::new_create()

    let mut db = DB::new();

    let vec = db.initiate_transaction(&|tx| -> Result<Vec<Funcionario>> {
            let vec =DB::select_x_from_y_where_z_map(
                tx,
                "*",
                "funcionarios",
                "",
                |(id, idade, nome, cargo)| {
                    Funcionario{
                        id,
                        idade,
                        nome,
                        cargo,
                    }
                },
                None,
            )?;
            Ok(vec)
        
    }).unwrap();

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
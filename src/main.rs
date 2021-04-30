mod db;

use db::{
    db::DB,
    entry::Funcionario,
};

fn main() {
    let mut new_db = DB::new();

    new_db.insert_funcionario(Funcionario::new(10, String::from("Joseph Joestar"), String::from("Rich person"))).unwrap();
    
    // let mut vec_func: Vec<Funcionario> = Vec::with_capacity(8);

    for i in 1..10 {
        let func = new_db.consultar_funcionario(i).unwrap();
        match func {
            Some(func) => {
                println!("id: {}\nidade: {}\nnome: {}\ncargo: {}\n", func.id, func.idade, func.nome, func.cargo);
            },
            None => {
                println!("Got None at {}", i);
            }
        }
        
        // vec_func.push(new_db.consultar_funcionario(i).unwrap()[0].clone());
    }

    // for func in vec_func { 
    //     println!("id: {}\nidade: {}\nnome: {}\ncargo: {}\n", func.id, func.idade, func.nome, func.cargo);
    // }
    
    println!("Hello, world!");
}

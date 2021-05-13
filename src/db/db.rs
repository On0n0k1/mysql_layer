use mysql::*;
// use mysql::prelude::*;
use mysql::prelude::{
    Queryable,
    FromRow,
};

// import funcionario type and link type
use crate::db::{
    // entry::Funcionario,
    link::DBInfo,
};


pub struct DB{
    link: DBInfo,
}

impl DB{
    pub fn new() -> Self {
        let link = DBInfo::new().unwrap();

        DB{
            link,
        }
    }


    // Start a transaction, execute the function that requires the transaction. Then commit or rollback the transaction depending on the result. 
    // T is generic, Means that it will return anything that is within the Ok() result.
    pub fn initiate_transaction<T>(&mut self, clos: &dyn Fn(&mut Transaction<'_>) -> Result<T>) -> Result<T>{
        let mut conn = self.link.connect_as_user()?;
        
        let mut tx = conn.start_transaction(TxOpts::default())?;
        match clos(&mut tx) {
            Ok(result) => {
                tx.commit()?;
                return Ok(result);
            },
            Err(err) => {
                tx.rollback()?;
                return Err(err);
            },
        }
    }


    // CREATE TABLE funcionarios(
    //     id int AUTO_INCREMENT UNIQUE NOT NULL,
    //     idade INT NOT NULL,
    //     nome VARCHAR(100) NOT NULL,
    //     cargo VARCHAR(50) NOT NULL,
    //     PRIMARY KEY(id)
    // );
    pub fn create_table_x_y(tx: &mut Transaction, x: String, y: String) -> Result<()> {
        tx.query_drop(
            format!("CREATE TABLE {} ({})", x, y),
        )
    }

    // comando drop table com os parametros opcionais. Caso true, eles são incluídos.
    pub fn drop_table_x(
        tx: &mut Transaction, 
        x: String,
        temporary: bool,
        if_exists: bool,
    ) -> Result<()> {

        let temporary: String = String::from(match temporary{
            true => {" TEMPORARY"},
            false => {""},
        });

        let if_exists: String = String::from(match if_exists{
            true => {" IF EXISTS"},
            false => {""},
        });

        tx.query_drop(
            format!("DROP{} TABLE{} {}", temporary, x, if_exists)
        )
    }

    pub fn select_x_from_y_where_z_map<F, T, U> (
        tx: &mut Transaction, 
        x: &str, 
        y: &str, 
        z: &str, 
        constructor: F,
        limit: Option<u8>,
    ) -> Result<Vec<U>> where 
        T: FromRow,
        F: FnMut(T) -> U,
    {
        
        let mut where_z = format!("WHERE {}", z);
        if let z = "" {
            where_z = String::from("");
        }

        let limit: String = match limit{
            None => {String::from(" LIMIT 100")},
            Some(limit) => {format!(" LIMIT {}", limit)},
        };

        let result = tx.query_map(
            format!("SELECT {} FROM {} {} {}", x, y, where_z, limit),
            constructor,
        );
        
        result
    }

    pub fn insert_into_x_y_values_z(tx: &mut Transaction, x: String, y: String, z: String) -> Result<()>{
        tx.query_drop(
            format!("INSERT INTO {} ({}) VALUES ({})", x, y, z),
        )
    }

    pub fn delete_from_x_where_y(tx: &mut Transaction, x: &str, y: &str) -> Result<()> {
        let mut where_y = format!("WHERE {}", y);

        if let y = "" {
            where_y = String::from("");
        }
        
        tx.query_drop(
            format!("DELETE FROM {} {}", x, where_y),
        )
    }


    // // This block was written too many times. So I turned it into a function to reduce bugs. Uses transaction connection to get an element with given id.
    // fn get_by_id(tx: &mut Transaction, idx: u32) -> Result<Option<Funcionario>>{
    //     // This will return a vector with a single element. Or an empty vector.
    //     let mut result = tx.query_map(
    //         format!("SELECT id, idade, nome, cargo FROM funcionarios WHERE (id = {})", idx),
    //         |(id, idade, nome, cargo)| {
    //             // println!("id: {}\nidade: {}\nnome: {}\ncargo: {}\n", id, idade, nome, cargo);
    //             Funcionario{
    //                 id,
    //                 idade,
    //                 nome,
    //                 cargo,
    //             }
    //         }
    //     )?;

    //     // Get the only Some(element) from the vector or becomes None.
    //     let result = result.pop();
    //     Ok(result)
    // }

    // // Inicia conexão como usuario e insere o struct funcionario no banco.
    // pub fn insert_funcionario(&mut self, funcionario: Funcionario) -> Result<()> {
    //     // Function (closure) for inserting funcionario wrapped within a transaction request.
    //     self.initiate_transaction(
    //         &|tx:&mut Transaction<'_>| -> Result<()> {
    //             // Formata essa string com os parametros de funcionario.
    //             let query_line = format!("INSERT INTO funcionarios (id, idade, nome, cargo) VALUES ({}, {}, \"{}\", \"{}\")", 
    //                 funcionario.id, 
    //                 funcionario.idade, 
    //                 funcionario.nome, 
    //                 funcionario.cargo,
    //             );
    //             // Usa a string acima como um comando de mysql.
    //             tx.query_drop(query_line)?;

    //             // Não houve erros até essa linha. Então retorna Ok.
    //             Ok(())
    //         }

    //     )?;

    //     Ok(())
    // }

    // pub fn consultar_funcionario(&mut self, idx: u32) -> Result<Option<Funcionario>>{
    //     let result = self.initiate_transaction(
    //         &|tx: &mut Transaction<'_>| -> Result<Option<Funcionario>> {
    //             Self::get_by_id(tx, idx)
    //         }
    //     );
    //     result

    // }

    // pub fn consultar_funcionarios(&mut self, id_start: u32, id_end: u32) -> Result<Vec<Funcionario>>{

    //     let result = self.initiate_transaction(

    //         &|tx: &mut Transaction<'_>| -> Result<Vec<Funcionario>> {

    //             let result = tx.query_map(
    //                 format!("SELECT id, idade, nome, cargo FROM funcionarios WHERE ((id >= {}) AND (id < {})) LIMIT 100", id_start, id_end),
    //                 |(id, idade, nome, cargo)| {
    //                     // println!("id: {}\nidade: {}\nnome: {}\ncargo: {}\n", id, idade, nome, cargo);
    //                     Funcionario{
    //                         id,
    //                         idade,
    //                         nome,
    //                         cargo,
    //                     }
    //                 }
    //             )?;

    //             Ok(result)
    //         }

    //     )?;

    //     Ok(result)
        
    // }

    // pub fn deletar_funcionario(&mut self, idx: u32) -> Result <Option<Funcionario>>{

    //     let result = self.initiate_transaction(
    //         &|tx: &mut Transaction<'_>| -> Result<Option<Funcionario>> {
    //             let result = Self::get_by_id(tx, idx)?;

    //             match result {
    //                 None => {Ok(None)},
    //                 Some(value) => {
    //                     tx.query_drop(
    //                         format!("DELETE FROM funcionarios WHERE (id = {})", idx)
    //                     )?;

    //                     Ok(Some(value))
    //                 }
    //             }
    //         }
    //     )?;

    //     Ok(result)
    // }

    
    // pub fn deletar_todos_funcionarios(&mut self) -> Result<()> {
    //     self.initiate_transaction(
    //         &|tx: &mut Transaction<'_>| -> Result<()> {
    //             tx.query_drop(format!("DELETE FROM funcionarios"))
    //         }
    //     )?;
    //     Ok(())
    // }

    // pub fn update_funcionario_from_id_by_nome(&mut self, idx: u32, nome: String) -> Result<Option<Funcionario>> {
    //     let result = self.initiate_transaction(

    //         &|tx: &mut Transaction<'_>| -> Result<Option<Funcionario>> {
    //             let result = Self::get_by_id(tx, idx)?;

    //             match result {
    //                 None => {Ok(None)},
    //                 Some(value) => {
    //                     let query_line = format!("UPDATE funcionarios SET nome = \"{}\" WHERE (id = {})", nome, idx);
    //                     // println!("\n\nTrying this query:\n\n{}\n\n", query_line);
    //                     tx.query_drop(query_line)?;

    //                     Ok(Some(value))
    //                 }
    //             }
    //         }

    //     )?;

    //     Ok(result)
    // }
}


impl Clone for DB{
    fn clone(&self) -> Self{
        Self::new()
    }
}
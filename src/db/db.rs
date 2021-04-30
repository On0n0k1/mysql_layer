use mysql::*;
// use mysql::prelude::*;
use mysql::prelude::Queryable;

// import funcionario type and link type
use crate::db::{
    entry::Funcionario,
    link::DBInfo,
};


pub struct DB{
    link: DBInfo,
}

impl DB{
    pub fn new() -> Self {
        let link = DBInfo::new();

        DB{
            link,
        }
    }

    // Start a transaction, execute the function that requires the transaction. Then commit or rollback the transaction depending on the result. T is generic, Means that it will return anything that is withing the Ok() result.
    fn initiate_transaction<T>(&mut self, clos: &dyn Fn(&mut Transaction<'_>) -> Result<T>) -> Result<T>{
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

    // Inicia conexão como usuario e insere o struct funcionario no banco.
    pub fn insert_funcionario(&mut self, funcionario: Funcionario) -> Result<()> {

        // Function (closure) for inserting funcionario wrapped within a transaction request.
        self.initiate_transaction(

            &|tx:&mut Transaction<'_>| -> Result<()> {
                tx.query_drop(
                    format!("INSERT INTO funcionarios (id, idade, nome, cargo) VALUES ({}, {}, '{}', '{}')", 
                        funcionario.id, 
                        funcionario.idade, 
                        funcionario.nome, 
                        funcionario.cargo
                    )
                )?;

                Ok(())
            }

        )?;

        Ok(())
    }

    pub fn consultar_funcionario(&mut self, idx: u32) -> Result<Option<Funcionario>>{

        let mut result = self.initiate_transaction(

            &|tx: &mut Transaction<'_>| -> Result<Vec<Funcionario>> {

                let result = tx.query_map(
                    format!("SELECT id, idade, nome, cargo FROM funcionarios WHERE (id = {})", idx),
                    |(id, idade, nome, cargo)| {
                        // println!("id: {}\nidade: {}\nnome: {}\ncargo: {}\n", id, idade, nome, cargo);
                        Funcionario{
                            id,
                            idade,
                            nome,
                            cargo,
                        }
                    }
                )?;

                Ok(result)
            }

        )?;

        Ok(result.pop())
    }

    pub fn deletar_funcionario(&mut self, idx: u32){

    }

    pub fn atualizar_funcionario(&mut self, idx: u32, funcionario: Funcionario) {

    }
}


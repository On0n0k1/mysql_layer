use std::{
    fmt,
    result,
    env,
};

use dotenv::from_path;


use crate::db::db::DB;
use mysql::*;
use mysql::Value;
// use mysql::prelude::*;
use mysql::prelude::{
    Queryable,
    FromRow,
};

use crate::db::column::Column;


pub struct Table{
    name: String,
    column: Vec<Column>,
    // db: DB,
}



impl Table{
    pub fn new_create(name: String, column: Vec<Column>) -> Result<Self> {
        // let db = DB::new();
        let mut new_table = Table{
            name,
            column,
            // db,
        };

        // Tenta criar a tabela no banco de dados. Se houver erro, retorna o erro dropando o objeto.
        new_table.create_table_table_column()?;

        Ok(new_table)
    }

    pub fn new_connect(name: &str, column: Vec<Column>) -> Result<Self> {
        let new_table = Table{
            name: String::from(name),
            column,
            // db,
        };

        // Tenta criar a tabela no banco de dados. Se houver erro, retorna o erro dropando o objeto.
        // new_table.create_table_table_column()?;

        Ok(new_table)
    }


    pub fn create_table_table_column(&mut self) -> Result<()> {
        // let x = self.name.clone();
        // let columns = Column::join_full_name(&self.column);
        
        // A tabela pode não ter nenhuma coluna. Nessa situação, ficaria como "CREATE TABLE ${NAME} ();"
        // let mut y: String = String::from("");

        // // Escreve a descrição de cada coluna em uma unica linha, e.g: "    id int AUTO_INCREMENT UNIQUE NOT NULL,"
        // if self.column.len() > 0 {
        //     y = format!("{}", self.column[0]);
        //     for i in 1..self.column.len() {
        //         y.push_str(format!(",\n    {}", i))
        //     }
        //     y.push('\n');
        // }

        // let clos = |tx: &mut Transaction<'_>| -> Result<()> {
        //     DB::create_table_x_y(tx, String::from(x), String::from(columns))

        // };

        let mut db = DB::new();
        // create a copy of itself to use in closure
        // let new_self = self.clone();

        // Executa o query do banco de dados para criar a tabela, se houve erro, retorna o erro.
        db.initiate_transaction(
            &|tx: &mut Transaction<'_>| -> Result<()> {
                // let new_self = &new_self;
                let columns = Column::join_full_name(&self.column);
                DB::create_table_x_y(tx, self.name.clone(), columns)
            }
        )?;

        // Como alcançou essa linha, não houve erros. Retorna Ok.
        Ok(())
    }


    pub fn drop_table_table(
        &mut self, 
        temporary: bool,
        if_exists: bool,
    ) -> Result<()> {

        let mut db = DB::new();
        // // create a copy of itself to use in closure
        // let new_self = self.clone();
        db.initiate_transaction(
            &|tx: &mut Transaction<'_>| -> Result<()> {
                DB::drop_table_x(tx, self.name.clone(), temporary, if_exists)
            }
        )
    }


    pub fn select_x_from_table_where_y_map<F, T, U> (&self, tx: &mut Transaction, x: &Vec<Column>, y: &str, constructor: F) -> Result<Vec<U>> where 
        T: FromRow,
        F: FnMut(T) -> U,
    {
        DB::select_x_from_y_where_z_map(tx, &Column::join_single_name(&x)[..], &self.name.clone()[..], y, constructor, None)
    }

    pub fn select_all_from_table<F, T, U> (&mut self, constructor: F) -> Result<Vec<U>> where 
    T: FromRow,
    F: FnMut(T) -> U + Copy,
    {
        let mut db = DB::new();

        let vec = db.initiate_transaction(&|tx: &mut Transaction<'_>| -> Result<Vec<U>> {
            DB::select_x_from_y_where_z_map(tx, "*", &self.name.clone()[..], "", constructor, None)
        })?;

        Ok(vec)
    }


    pub fn insert_into_table_x_values_y(&self, tx: &mut Transaction, x: &Vec<Column>, y: String) -> Result<()> {
        DB::insert_into_x_y_values_z(tx, self.name.clone(), Column::join_single_name(x), y)
    }


    pub fn delete_from_table_where_x(&self, tx: &mut Transaction, x: &str) -> Result<()> {
        DB::delete_from_x_where_y(tx, &self.name.clone()[..], x)
    }

}


impl Clone for Table{
    fn clone(&self) -> Self{
        Table{
            name: self.name.clone(),
            column: self.column.clone(),
            // db: self.db.clone(),
        }
    }
}
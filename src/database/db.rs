use mysql::*;
use mysql::prelude::{
    Queryable,
    FromRow,
};

use crate::database::{
    link::DBInfo,
};


pub struct DB{
    pub link: DBInfo,
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
    pub fn initiate_transaction<T>(
            &mut self, 
            clos: &dyn Fn(&mut Transaction<'_>) -> Result<T>,
        ) -> Result<T>
    {
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


    pub fn create_table_x_y(
            tx: &mut Transaction, 
            x: String, 
            y: String,
        ) -> Result<()> 
    {
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
        ) -> Result<()> 
    {

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
            z: Option<&str>, 
            limit: Option<u32>,
            constructor: F,
        ) -> Result<Vec<U>> where 
            T: FromRow,
            F: FnMut(T) -> U,
    {
        let where_z = match z {
            None => {String::from("")},
            Some(optional) => {format!("WHERE {}", optional)},
        };

        let limit: String = match limit{
            // limit = None means unlimited
            None => {String::from("")},
            Some(limit) => {format!(" LIMIT {}", limit)},
        };

        let result = tx.query_map(
            format!("SELECT {} FROM {} {} {}", x, y, where_z, limit),
            constructor,
        );
        
        result
    }


    pub fn insert_into_x_y_values_z(
            tx: &mut Transaction, 
            x: &str, 
            y: &str, 
            z: &str
        ) -> Result<()>
    {
        tx.query_drop(
            format!("INSERT INTO {} ({}) VALUES ({})", x, y, z),
        )
    }

    pub fn update_x_set_y_where_z(
        tx: &mut Transaction,
        x: &str,
        y: &str,
        z: Option<&str>,
    )   -> Result<()>
    {
        let where_z = match z {
            None => {String::from("")},
            Some(optional) => {format!("WHERE {}", optional)},
        };

        tx.query_drop(
            format!("UPDATE {} SET {} {}", x, y, where_z),
        )
    }

    pub fn delete_from_x_where_y(
            tx: &mut Transaction, 
            x: &str, 
            y: Option<&str>,
        ) -> Result<()> 
    {
        let where_y: String = match y {
            Some(optional) => {
                format!("WHERE {}", optional)
            },
            None => String::from("")
        };
        
        tx.query_drop(
            format!("DELETE FROM {} {}", x, where_y),
        )
    }
}


impl Clone for DB{
    fn clone(&self) -> Self{
        Self::new()
    }
}
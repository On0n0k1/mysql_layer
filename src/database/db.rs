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
        let link = DBInfo::new(true).unwrap();

        DB{
            link,
        }
    }

    pub fn initial_setup (&mut self) -> std::result::Result<(), String>
    {
        let mut conn = match self.link.connect_as_root(){
            Ok(conn) => conn,
            Err(err) => {
                let result = format!("Error connecting as root.");
                println!("{}. err: {}", result, err);
                return Err(result);
            },
        };
        
        let mut tx = match conn.start_transaction(TxOpts::default()) {
            Ok(tx) => tx,
            Err(err) => {
                let result = format!("Error initiating transaction.");
                println!("{}. err: {}", result, err);
                return Err(result);
            },
        };

        let clos = &|tx: &mut Transaction<'_>| -> Result<()>{
            let query = "SET SESSION sql_mode=''";
            println!("Calling: {}", query);
            tx.query_drop(query)?;
            
            let db_name = self.link.db_name.clone();
            // let db_address_in = self.link.address_in.clone();
            let user_name = self.link.user_name.clone();
            let user_pw = self.link.user_pw.clone();

            let query = format!("CREATE DATABASE {} CHARACTER SET utf8", db_name);
            println!("Calling: {}", query);
            tx.query_drop(query)?;

            // This is insecure authentication, but I'm in a hurry. Need to change this for production.
            let query = format!("CREATE USER '{}'@'%' IDENTIFIED WITH mysql_native_password BY '{}'",
            // let query = format!("CREATE USER '{}'@'{}' IDENTIFIED BY '{}'",
                user_name,
                // db_address_in,
                user_pw,
            );

            println!("Calling: {}", query);
            tx.query_drop(query)?;

            // let query = format!("SET old_passwords = 2");
            // println!("Calling: {}", query);
            // tx.query_drop(query)?;
            
            // let query = format!("SET PASSWORD FOR '{}'@'{}' = PASSWORD('{}')",
            //     user_name,
            //     db_address,
            //     user_pw,
            // );
            // println!("Calling: {}", query);
            // tx.query_drop(query)?;


            let query = format!("USE {}", db_name);
            println!("Calling: {}", query);
            tx.query_drop(query)?;

            let query = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                "CREATE TABLE funcionarios(",
                "    id int AUTO_INCREMENT UNIQUE NOT NULL,",
                "    idade INT NOT NULL,",
                "    nome VARCHAR(100) NOT NULL,",
                "    cargo VARCHAR(50) NOT NULL,",
                "    PRIMARY KEY(id)",
                ")",
            );

            println!("Calling: {}", query);
            tx.query_drop(query)?;

            // GRANT ALL PRIVILEGES ON * . * TO 'newuser'@'localhost';
            let query = format!("GRANT ALL PRIVILEGES ON {} . {} TO '{}'@'%'",
                db_name,
                "funcionarios",
                user_name,
                // db_address_in,
            );
            println!("Calling: {}", query);
            tx.query_drop(query)?;

            let query = "FLUSH PRIVILEGES";
            println!("Calling: {}", query);
            tx.query_drop(query)?;
            
            Ok(())
        };

        match clos(&mut tx) {
            Ok(_) => {
                match tx.commit(){
                    Err(err) => {
                        let result = format!("Error commiting transaction.");
                        println!("{}. err: {}", result, err);
                        return Err(result);
                    },
                    Ok(_) => {return Ok(())}
                }
            },
            Err(err_t) => {
                match tx.rollback(){
                    Err(err) => {
                        let result = format!("Error in transaction and error rolling back.");
                        println!("{}. err: {}.\n err_t: {}\n", result, err, err_t);
                        return Err(result);
                    },
                    Ok(_) => {
                        let result = format!("Error in transaction. RollBack successful.");
                        println!("{} err_t: {}\n", result, err_t);
                        return Err(result);
                    }
                };
            },
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

        let query = "SET SESSION sql_mode=''";
        println!("Calling: {}", query);
        tx.query_drop(query)?;
        
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

        let query = format!("SELECT {} FROM {} {} {}", x, y, where_z, limit);
        println!("Calling Query: {}", query);
        let result = tx.query_map(
            query,
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
        let query = format!("INSERT INTO {} ({}) VALUES ({})", x, y, z);
        println!("Calling Query: {}", query);
        tx.query_drop(
            query,
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

        let query = format!("UPDATE {} SET {} {}", x, y, where_z);
        println!("Calling query {}", format!("UPDATE {} SET {} {}", x, y, where_z));
        tx.query_drop(query)
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
        let query = format!("DELETE FROM {} {}", x, where_y);
        println!("{}", query);
        tx.query_drop(
            query,
        )
    }
}


impl Clone for DB{
    fn clone(&self) -> Self{
        Self::new()
    }
}
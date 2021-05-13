use std::env;
use std::result;

use dotenv::from_path;

use mysql::*;
use mysql::prelude::*;


// Holds data for connecting with the database. Each attribute is private.
pub struct DBInfo{
    db_root_pw: String,
    user_name: String,
    user_pw: String,
    db_name: String, 
    address: String,
}

impl DBInfo{
    pub fn new() -> result::Result<DBInfo, dotenv::Error> {
        Self::get_env_vars()?;

        // This block sets the environment variable names
        let (
            db_root_pw,
            user_name,
            user_pw,
            db_name, 
            address,
        ) = (
                String::from("db_root_pw"),
                String::from("db_local_user"),
                String::from("db_local_pw"),
                String::from("db_local_db_name"),
                String::from("db_address"),
        );
        
        // This block uses the names to get the environment variable values.
        // Using them to return the DBInfo struct. The ? means that if an error occurs, return that error.
        Ok(
            DBInfo{
                db_root_pw: dotenv::var(db_root_pw)?,
                user_name: dotenv::var(user_name)?,
                user_pw: dotenv::var(user_pw)?,
                db_name: dotenv::var(db_name)?,
                address: dotenv::var(address)?,
            }
        )
    }

    // Coleta as variaveis de ambiente no diretorio .env
    fn get_env_vars() -> result::Result<(), dotenv::Error>{
        let my_path = env::current_dir().unwrap();
        let my_path = my_path.join("env/.env");
        // println!("Accessing {}", my_path.to_str().unwrap());
        dotenv::from_path(my_path.as_path())
    }

    // connect to the database
    fn connect(&mut self, user_name: String, user_pw: String, db_name: String) -> Result<PooledConn>{
        let url = format!("mysql://{}:{}@{}/{}", user_name, user_pw, self.address, db_name);
        let pool = Pool::new(&url)?;
        let conn = pool.get_conn()?;
        Ok(conn)
    }

    // Connect to the database as the root user
    pub fn connect_as_root(&mut self) -> Result<PooledConn> {
        let conn = self.connect(
            String::from("root"), 
            self.db_root_pw.clone(), 
            String::from(""))?;
        Ok(conn)
    }

    // Connect to a specific database as the given user
    pub fn connect_as_user(&mut self) -> Result<PooledConn> {
        let conn = self.connect(
            self.user_name.clone(), 
            self.user_pw.clone(), 
            self.db_name.clone())?;
        Ok(conn)
    }    
}
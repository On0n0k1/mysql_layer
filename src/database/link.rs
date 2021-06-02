// use sha2::{Sha256, Digest};

use std::env;
use std::result;

// use dotenv::from_path;

use mysql::*;
// use mysql::prelude::*;


// Holds data for connecting with the database. Each attribute is private.
pub struct DBInfo{
    pub db_root_user: String,
    pub db_root_pw: String,
    pub user_name: String,
    pub user_pw: String,
    pub db_name: String, 
    pub address: String,
    pub address_in: String,
    pub caching_sha2_password_is_on: bool,
}

impl DBInfo{
    
    pub fn get_db_root_user(&self) -> String {
        let db_root_user = self.db_root_user.clone();
        db_root_user
    }

    pub fn get_db_root_pw(&self) -> String {
        let db_root_pw = self.db_root_pw.clone();
        db_root_pw
    }

    pub fn get_user_name(&self) -> String {
        let user_name = self.user_name.clone();
        user_name
    }

    pub fn get_user_pw(&self) -> String {
        let user_pw = self.user_pw.clone();
        user_pw
    }

    pub fn get_db_name(&self) -> String {
        let db_name = self.db_name.clone();
        db_name
    }

    pub fn get_address(&self) -> String {
        let address = self.address.clone();
        address
    }

    fn check_error<T>(name: &str, result: dotenv::Result<T>) -> std::result::Result<T, String> {
        return match result{
            Ok(value) => Ok(value),
            Err(err) => {
                let error_message = format!("Error collecting {} from env variables.", name);
                println!("{}. err: {}.\n\n", error_message, err);
                Err(error_message)
            }
        }
    }

    pub fn new(caching_sha2_password_is_on: bool) -> result::Result<DBInfo, String> {
        Self::check_error("env_vars", Self::get_env_vars())?;

        // This block sets the environment variable names
        let (
            db_root_user,
            db_root_pw,
            user_name,
            user_pw,
            db_name, 
            address,
            address_in,
        ) = (
                String::from("db_root_user"),
                String::from("db_root_pw"),
                String::from("db_local_user"),
                String::from("db_local_pw"),
                String::from("db_local_db_name"),
                String::from("db_address"),
                String::from("db_address_in"),
        );

        
        // This block uses the names to get the environment variable values.
        // Using them to return the DBInfo struct. The ? means that if an error occurs, return that error.
        let db_root_user = Self::check_error("db_root_user", dotenv::var(db_root_user))?;
        let db_root_pw = Self::check_error("db_root_pw", dotenv::var(db_root_pw))?;
        // println!("Using root password: {}", db_root_pw);
        
        // let db_root_pw = match dotenv::var(db_root_pw){
        //     Ok(value) => value,
        //     Err(err) => {
        //         let error_message = format!("Error collecting db_root_pw from env variables.");
        //         println!("{}. err: {}.\n\n", error_message, err);
        //         return Err(error_message);
        //     }
        // };


        // create a Sha256 object

        // let mut hasher = Sha256::new();
        // hasher.update(&db_root_pw[..]);

        // let db_root_pw = hasher.finalize();
        // let db_root_pw = format!("{:x}", db_root_pw);
        
        // let db_root_pw = db_root_pw.as_slice();
        // let db_root_pw = std::str::from_utf8(db_root_pw);
        // let db_root_pw = match db_root_pw{
        //     Ok(value) => format!("{:x}", value),
        //     Err(err) => {
        //         let error_message = String::from("Error hashing password");
        //         println!("{}, err: {} \n\n", error_message, err);
        //         return Err(error_message);
        //     }
        // };

        let db_root_pw = format!("{}", db_root_pw);
        let (user_name, user_pw, db_name, address, address_in) = (
            Self::check_error("user_name", dotenv::var(user_name))?,
            Self::check_error("user_pw", dotenv::var(user_pw))?,
            Self::check_error("db_name", dotenv::var(db_name))?,
            Self::check_error("address", dotenv::var(address))?,
            Self::check_error("address_in", dotenv::var(address_in))?,
        );

        Ok(
            DBInfo{
                db_root_user,
                db_root_pw,
                user_name,
                user_pw,
                db_name,
                address,
                address_in,
                caching_sha2_password_is_on,
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
        println!("connecting to {}", url);
        let pool = Pool::new(&url)?;
        let conn = pool.get_conn()?;
        Ok(conn)
    }

    // Connect to the database as the root user
    pub fn connect_as_root(&mut self) -> Result<PooledConn> {
        let conn = self.connect(
            self.db_root_user.clone(), 
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
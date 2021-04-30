use mysql::*;
use mysql::prelude::*;


// this is just a temporary struct for testing
pub struct DBInfo{
    db_root_pw: String,
    user_name: String,
    user_pw: String,
    db_name: String, 
    address: String,
}

impl DBInfo{
    pub fn new() -> Self {
        let (
            db_root_pw,
            user_name,
            user_pw,
            db_name, 
            address,
        ) = (
                String::from("something"),
                String::from("someone"),
                String::from("somethingelse"),
                String::from("bd_teste"),
                String::from("localhost:8666"),
        );

        DBInfo{
            db_root_pw,
            user_name,
            user_pw,
            db_name,
            address,
        }
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
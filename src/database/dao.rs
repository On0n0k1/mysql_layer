use mysql::*;
use mysql::prelude::{
    // Queryable,
    FromRow,
};

use crate::database::db::DB;

// pub trait DaoTrait where
//     Self: Clone,
// {
//     type Item: FromRow;
//     // db_name = None makes use default name from .env
//     fn get_db_name() -> Option<String>;
//     fn get_table_name() -> String;
//     fn get_columns() -> Vec<String>;
//     fn get_columns_values(&self) -> Vec<(String, String)>;
//     fn get_id(&self) -> u32;
//     fn set_id(&mut self, id: u32);
//     // Column order from get_columns_values must be the same as the constructor
//     fn get_constructor() -> Box<dyn FnMut(Self::Item) -> Self>;
//     // fn constructor<T>(row: T) -> Self where T: FromRow;
// }


/// Turn a vector of tuples (columns, values) into two Strings for select queries.
fn format_columns_values_select(columns_values: Vec<(String, String)>) -> std::result::Result<(String, String), String> {
    if columns_values.len() == 0 {
        return Err(format!("Columns_values is empty"));
    }

    let (mut columns, mut values) = columns_values[0].clone();

    for i in 1..columns_values.len() {
        let (column, value) = columns_values[i].clone();
        columns = format!("{}, {}", columns, column);
        values = format!("{}, {}", values, value);
    }

    // columns = format!("({})", columns);
    // values = format!("({})", values);

    Ok((columns, values))
}


/// Turn a vector of tuples (columns, values) into two Strings for update queries.
fn format_columns_values_update(columns_values: Vec<(String, String)>) -> std::result::Result<String, String> {
    if columns_values.len() == 0 {
        return Err(format!("Columns_values is empty"));
    }
    let (column, value) = columns_values[0].clone();
    let mut set_queries = format!("{} = {}", column, value);

    for i in 1..columns_values.len() {
        let (column, value) = columns_values[i].clone();
        set_queries = format!("{}, {} = {}", set_queries, column, value);
    }
    // set_queries = format!("({})", set_queries);

    Ok(set_queries)
}


/// Turn a vector of columns into a single string for query operations.
fn format_columns(columns: Vec<String>) -> std::result::Result<String, String> {
    if columns.len() == 0 {
        return Err(format!("Columns is empty"));
    }

    let mut formatted = columns[0].clone();

    for i in 1..columns.len() {
        formatted = format!("{}, {}", formatted, columns[i]);
    }
    // formatted = format!("({})", formatted);

    Ok(formatted)
}

pub trait DAO<D>
{
    type Item: FromRow;
    // db_name = None makes use default name from .env
    fn get_db_name() -> Option<String>;
    fn get_table_name() -> String;
    fn get_columns() -> Vec<String>;
    fn get_columns_values(element: &D) -> Vec<(String, String)>;
    fn get_id(element: &D) -> u32;
    fn set_id(element: &mut D, id: u32);
    // Column order from get_columns_values must be the same as the constructor
    fn get_constructor() -> Box<dyn FnMut(Self::Item) -> D>;
    // fn constructor<T>(row: T) -> Self where T: FromRow;

    // Hint: self refers to the object itself. Self refers to the type of the object.
    /// Insert a copy of the element into the database.
    fn add(mut element: D) -> std::result::Result<(), String> 
    {
        Self::set_id(&mut element, 0);

        // let check_id = element.get_id();
        let check_id = Self::get_id(&element);
        if check_id != 0 {
            return Err(format!("Couldn't set id to 0. id is still {}", check_id));
        }

        let (columns, values) = format_columns_values_select(Self::get_columns_values(&element))?;
        let (columns, values) = (&columns[..], &values[..]);

        let table_name = &Self::get_table_name()[..];

        // initiate_transaction is a wrapper around a closure.
        // It starts a transaction with mysql, do a series of operations,
        // If an error occurs, call rollback. Else, call commit.
        let result = DB::new().initiate_transaction(
            &|tx| -> Result<()> {
                DB::insert_into_x_y_values_z(
                    tx,
                    table_name,
                    columns,
                    values,
                )?;
                Ok(())
            },
        );

        match result {
            Err(err) => {Err(format!("Internal database Error for add call: {}", err))},
            Ok(_) => {Ok(())},
        }
    }

    /// Get an element with given ID from the database.
    fn get (id: u32,) -> std::result::Result<Option<D>, String>
    {
        // let columns = &format_columns(Self::get_columns())?[..];
        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];
        // let constructor: F = D::get_constructor();

        // Start transaction
        let value: Result<Option<D>> = DB::new().initiate_transaction(
            &|tx| -> Result<Option<D>>{

                // Attempt to select the elements, building the objects with get_constructor
                let mut vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    Some(&(format!("(id = {})", id)[..])), 
                    Some(1), 
                    Self::get_constructor(),
                )?;

                Ok(vec.pop())
            },
        );

        match value {
            Err(err) => {Err(format!("Internal database Error for get call: {}", err))},
            Ok(val) => {Ok(val)},
        }
    }


    /// Gets all elements (up to limit) from the database, 
    /// parsing all the results, if successful. If limit is None, get everything.
    fn list(where_z: Option<&str>, limit: Option<u32>) -> std::result::Result<Vec<D>, String> 
    {

        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];

        // For workload reasons, None will always limit itself to 1000. A higher value can still be taken from the message.
        let limit = match limit{
            None => Some(1000),
            Some(value) => Some(value),
        };
        
        // let constructor: F = D::get_constructor();

        // start transaction
        let vec = DB::new().initiate_transaction(
            &|tx| -> Result<Vec<D>>{

                // Does the same as get, but keep the entire vector,
                // instead of just picking a single element from it.
                let vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    where_z, 
                    limit, 
                    Self::get_constructor(),
                )?;

                Ok(vec)
            },
        );

        match vec {
            Err(err) => {Err(format!("Internal database Error for list call: {}", err))},
            Ok(val) => {Ok(val)},
        }
    }

    /// Update an element from the database with the same ID as element.
    fn update(element: D) -> std::result::Result<(), String> {

        let table_name = &Self::get_table_name()[..];
        // let element_id = element.get_id();
        let element_id = Self::get_id(&element);
        
        let y = &format_columns_values_update(
            Self::get_columns_values(&element)
        )?[..];

        // start transaction
        let result = DB::new().initiate_transaction(
            &|tx| -> Result<()> {
                
                // The ? at the end means that it returns an error, if it happens
                DB::update_x_set_y_where_z(
                    tx, 
                    table_name,
                    y, 
                    Some(&(format!("(id = {})", element_id))[..]),
                )?;

                // Reached here with no errors, then Ok.
                Ok(())
            },
        );

        match result {
            Err(err) => {Err(format!("Internal database Error for list call: {}", err))},
            Ok(_) => {Ok(())},
        }
    }


    fn remove_id (
        id: u32,
        limit: Option<u32>,
    ) -> std::result::Result<Option<D>, String>
    {
        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];
        // let constructor: F = D::get_constructor();

        // start transaction
        let value: Result<Option<D>> = DB::new().initiate_transaction(
            &|tx| -> Result<Option<D>>{

                // Attempt to select an element from the database
                let mut vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    Some(&(format!("(id = {})", id))[..]), 
                    limit, 
                    Self::get_constructor(),
                )?;

                let value = vec.pop();
                // Check if value was found, if value doesn't exist, return Ok(None)
                match value {
                    None => {return Ok(None);},
                    Some(value) => {
                        // If value was found, attempt to delete it. 
                        // Return Ok(Some(value)), Or Err(error_message), if it succeeded or not.
                        match DB::delete_from_x_where_y(
                            tx, 
                            table_name, 
                            Some(&(format!("(id = {})", id))[..]),
                        ){
                            Err(err) => {
                                println!("Found element but unable to remove. Err = {}.", err);
                                return Err(err);
                            },
                            Ok(_) => {return Ok(Some(value))}
                        }
                    }
                }
            },
        );

        match value {
            Err(err) => {Err(format!("Internal database Error for get call: {}", err))},
            Ok(val) => {Ok(val)},
        }
    }


    // Does the same as remove_id, but arg is the object.
    fn remove_element(
        element: D,
        limit: Option<u32>,
    ) -> std::result::Result<Option<D>, String>
    {
        let id = Self::get_id(&element);
        // let id = element.get_id();
        Self::remove_id(id, limit)
    }  
}


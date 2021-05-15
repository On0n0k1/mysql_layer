use mysql::*;
use mysql::prelude::{
    Queryable,
    FromRow,
};

use crate::database::db::DB;

pub trait DaoTrait where
    Self: Clone,
{
    type Item: FromRow;
    // db_name = None makes use default name from .env
    fn get_db_name() -> Option<String>;
    fn get_table_name() -> String;
    fn get_columns() -> Vec<String>;
    fn get_columns_values(&self) -> Vec<(String, String)>;
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    // Column order from get_columns_values must be the same as the constructor
    fn get_constructor() -> Box<dyn FnMut(Self::Item) -> Self>;
    // fn constructor<T>(row: T) -> Self where T: FromRow;
}


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

    columns = format!("({})", columns);
    values = format!("({})", values);

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
    set_queries = format!("({})", set_queries);

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
    formatted = format!("({})", formatted);

    Ok(formatted)
}

pub trait DAO where Self: DaoTrait,
{

    fn add(&self) -> std::result::Result<(), String> 
    {
        let element = self.clone();
        element.set_id(0);

        let check_id = element.get_id();
        if check_id != 0 {
            return Err(format!("Couldn't set id to 0. id is still {}", check_id));
        }

        let (columns, values) = format_columns_values_select(element.get_columns_values())?;
        let (columns, values) = (&columns[..], &values[..]);

        let table_name = &Self::get_table_name()[..];

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

    fn get (
        id: u32,
        limit: Option<u32>,
    ) -> std::result::Result<Option<Self>, String>
    {

        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];
        // let constructor: F = D::get_constructor();

        let value: Result<Option<Self>> = DB::new().initiate_transaction(
            &|tx| -> Result<Option<Self>>{

                let vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    Some(format!("(id = {})", id)), 
                    limit, 
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


    fn list (limit: Option<u32>) -> std::result::Result<Vec<Self>, String> 
    {

        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];
        // let constructor: F = D::get_constructor();

        let vec = DB::new().initiate_transaction(
            &|tx| -> Result<Vec<Self>>{

                let vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    None, 
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

        // Ok(vec)
    }


    fn update(element: Self) -> std::result::Result<(), String> {

        let table_name = &Self::get_table_name()[..];
        let element_id = element.get_id();
        
        let y = &format_columns_values_update(
            element.get_columns_values()
        )?[..];

        let result = DB::new().initiate_transaction(
            &|tx| -> Result<()> {
                
                DB::update_x_set_y_where_z(
                    tx, 
                    table_name,
                    y, 
                    Some(format!("(id = {})", element_id)),
                )?;

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
    ) -> std::result::Result<Option<Self>, String>
    {
        let columns = &format_columns(Self::get_columns())?[..];
        let table_name = &Self::get_table_name()[..];
        // let constructor: F = D::get_constructor();

        let value: Result<Option<Self>> = DB::new().initiate_transaction(
            &|tx| -> Result<Option<Self>>{

                let vec = DB::select_x_from_y_where_z_map(
                    tx, 
                    columns, 
                    table_name, 
                    Some(format!("(id = {})", id)), 
                    limit, 
                    Self::get_constructor(),
                )?;

                let value = vec.pop();
                match value {
                    None => {return Ok(None);},
                    Some(value) => {
                        match DB::delete_from_x_where_y(
                            tx, 
                            table_name, 
                            Some(format!("(id = {})", id)),
                        ){
                            Err(err) => {
                                return Err(format!("Found element but unable to remove. Err = {}.", err));
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


    fn remove_element(
        &self,
        limit: Option<u32>,
    ) -> std::result::Result<Option<Self>, String>
    {
        let id = self.get_id();
        Self::remove_id(id, limit)
    }  
}


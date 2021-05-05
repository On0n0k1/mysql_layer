use std::fmt;

use crate::db::db::DB;
use mysql::*;
use mysql::Value;
// use mysql::prelude::*;
use mysql::prelude::{
    Queryable,
    FromRow,
};

pub struct Column{
    pub name: String,
    pub value_type: Value,
    pub extras: String,
}
// pub enum Value {
//     NULL,
//     Bytes(Vec<u8>),
//     Int(i64),
//     UInt(u64),
//     Float(f32),
//     Double(f64),
//     Date(u16, u8, u8, u8, u8, u8, u32),
//     Time(bool, u32, u8, u8, u8, u32),
// }

impl Column {
    fn value_string(&self) -> String{
        String::from(
            match self.value_type {
                Value::NULL => "NULL",
                Value::Bytes(_) => "Bytes",
                Value::Int(_) => "Int",
                Value::UInt(_) => "UInt",
                Value::Float(_) => "Float",
                Value::Double(_) => "Double",
                Value::Date(_, _, _, _, _, _, _) => "Date",
                Value::Time(_, _, _, _, _, _) => "Time",
            }
        )
    }

    // Used for create table statements, format a string with all required 
    // attributes for each arg.
    pub fn join_full_name(values: &Vec<Column>) -> String {
        // Se não tiver colunas, resultado é "()"
        // Se tiver uma coluna, resultado é "(${coluna})"
        // Se tiver mais de uma coluna, resultado é:
        //(
        //    ${coluna1},
        //    ${coluna2},
        //    ...
        //    ${colunaN}
        //)

        let mut result = String::from("");
        if values.len() > 0 {
            if values.len() > 1 {
                result.push_str("\n    ");
            }
            result.push_str(&(format!("{}", values[0]))[..]);

            for i in 1..values.len() {
                result.push_str(&(format!(",\n    {}", values[i]))[..]);
            }
            result.push_str("\n");
        }

        result
    }

    // Used for select statements, format a string with only the column 
    // names for each arg.
    pub fn join_single_name(values: &Vec<Column>) -> String {
        // Se não tiver colunas, resultado é "()"
        // Se tiver uma coluna, resultado é "(${name})"
        // Se tiver mais de uma coluna, resultado é:
        //(
        //    ${name1},
        //    ${name2},
        //    ...
        //    ${nameN}
        //)

        let mut result = String::from("");
        if values.len() > 0 {
            if values.len() > 1 {
                result.push_str("\n    ");
            }
            result.push_str(&(format!("{}", values[0].name.clone()))[..]);

            for i in 1..values.len() {
                result.push_str(&(format!(",\n    {}", values[i].name.clone()))[..]);
            }
            result.push_str("\n");
        }

        result
    }
}


// Essa trait permite printar ou formatar o struct como string.
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = format!("{} {} {}", self.name.clone(), self.value_string(), self.extras.clone());
        write!(f,"{}", text)
    }
}

impl Clone for Column {
    fn clone(&self) -> Self {
        Column{
            name: self.name.clone(),
            value_type: self.value_type.clone(),
            extras: self.extras.clone(),
        }
    }
}

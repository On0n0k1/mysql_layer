// Module for handling commandline arguments

use std::{
    env,
    str::FromStr,
};

// Essa library permite serializar e deserializar um struct em uma linha de codigo.
use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
// Essa library usa a library acima pra converter o struct para json e o inverso.
use serde_json::Result as Serde_Result;


// Get all the arguments in the cli and returns an iterator with the values it managed to succeed converting to the given type.
pub fn get_arguments<T: FromStr>() -> Vec<T>{
    let args: Vec<String> = env::args().collect();
    let mut return_vec: Vec<T> = Vec::new();

    for arg in args.iter(){
        match arg.parse::<T>(){
            Ok(value) => {
                return_vec.push(value);
            },
            Err(_) => {
                // println!("Couldn't convert {} into the type {}", arg, T);
            },
        }
    }


    return_vec

}

// // convert a single string reference into the given type. Return Some(value) if successful, None if not.
// fn try_json<T> (arg: &str) -> Option<T> where T: DeserializeOwned{
// // fn try_json<'de, T> (arg: String) -> Option<T> where T: Deserialize<'de>{
//     // let arg_ref = &arg[..]; 
//     match serde_json::from_str(arg){
//         Ok(thing) => {return thing},
//         Err(_err) => {return None}
//     };
// }

pub fn get_arguments_as_json<'de, T> (args: &'de Vec<String>) -> Option<T> where T: Deserialize<'de> {
    // match serde_json::from_str()
    for arg in args{
        let _result: Serde_Result<T> = match serde_json::from_str(&arg[..]){
            Serde_Result::Ok(thingy) => {
                return Some(thingy)
            },
            Serde_Result::Err(_err) => continue,
        };
    }
    None
}

// // Get the first valid json argument and return the object of the given type.
// pub fn get_arguments_as_json<'de, T>(replacement: &'de mut Option<T>) where T: Deserialize<'de> + Clone{
//     let args: Vec<String> = env::args().collect();
//     // let mut return_vec: Vec<T> = Vec::new();

//     for arg in args{
//         let arg_ref = &arg[..];
//         let attempt: Serde_Result<T> = serde_json::from_str(arg_ref);

//         match attempt {
//             Serde_Result::Ok(thing) => {
//                 let return_value: T = thing.clone();
//                 *replacement = Some(return_value);
//                 // return Some(return_value);
//             },
//             Serde_Result::Err(_) => continue
//         }



//         // if let Some(value) = try_json(arg_ref) {
//         //     return Some(value)
//         // };

//     }
//     *replacement = None;
// }



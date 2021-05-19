// Essa library permite serializar e deserializar um struct em uma linha de codigo.
use serde::{
    // Deserialize, 
    Serialize, 
    de::DeserializeOwned
};
// Essa library usa a library acima pra converter o struct para json e o inverso.
use serde_json::Result as SerdeResult;

pub struct Message{
    json: String,
}

impl Message{

    pub fn new_json(json: &str) -> Self{
        Message{json: String::from(json)}
    }

    pub fn new_value<T>(value: T) -> Result<Self, String> where T: Serialize {
        let json = serde_json::to_string_pretty(&value);
        match json{
            SerdeResult::Ok(value) => {return Ok(Message{json: value}); },
            SerdeResult::Err(err) => {return Err(format!("Error converting json. Err: {}\n\n", err));},
        }
    }

    pub fn store_json<T>(&mut self, value: T) -> Result<String, String> where T: Serialize {
        let json = serde_json::to_string_pretty(&value);
        match json{
            SerdeResult::Ok(value) => {
                self.json = value.clone();
                return Ok(value);
            },
            SerdeResult::Err(err) => {return Err(format!("Error storing json. Err: {}\n\n", err));}
        };
    }

    pub fn get_json(&self) -> String {
        self.json.clone()
    }
    
    pub fn get_value<T>(&self) -> Option<T> where T: Clone + Serialize + DeserializeOwned {
        let value = &(self.json[..]);
        let value: SerdeResult<T> = serde_json::from_str(value);
        match value {
            SerdeResult::Ok(return_value) => {return Some(return_value);},
            SerdeResult::Err(_) => {return None},
        };
    }


    


    // fn new_from_json<'de, T>(json: &'de str) -> Result<T, 
}

impl Clone for Message{
    fn clone(&self) -> Self {
        Message{
            json: self.json.clone(),
        }
    }
}

// pub trait MessageExt<'de>: 'de + Clone + Serialize + Deserialize<'de> {
//     fn new_from_json(json: &'de str) -> Option<Self> {
//         let funcionario: SerdeResult<Self> = serde_json::from_str(&json[..]);
//         match funcionario {
//             SerdeResult::Ok(value) => {
//                 return Some(value);
//             },
//             SerdeResult::Err(_) => {},
//         };
//         None
//     }

//     fn into_json(&self) -> String {
//         let json = (serde_json::to_string(self)).unwrap();
//         json
//     }
// }


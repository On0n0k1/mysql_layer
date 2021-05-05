// Essa library permite serializar e deserializar um struct em uma linha de codigo.
use serde::{Deserialize,Serialize};
// Essa library usa a library acima pra converter o struct para json e o inverso.
use serde_json::Result as Serde_Result;


pub trait Entry<'de>: 'de + Clone + Serialize + Deserialize<'de> {
    fn new_from_json(json: &'de str) -> Option<Self> {
        let funcionario: Serde_Result<Self> = serde_json::from_str(&json[..]);
        match funcionario {
            Serde_Result::Ok(value) => {
                return Some(value);
            },
            Serde_Result::Err(_) => {},
        }
        None
    }

    fn into_json(&self) -> String {
        let json = (serde_json::to_string(self)).unwrap();
        json
    }
}



use serde::{Deserialize, Serialize};
use serde_json::Result;


use crate::lambda::message_trait::Message;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Funcionario{
    pub id: u32,
    pub idade: u32,
    pub nome: String,
    pub cargo: String,
}



// Essa trait permite criar copias do mesmo struct manualmente.
impl Clone for Funcionario{
    fn clone(&self) -> Self {
        let id = self.id.clone();
        let idade = self.idade.clone();
        let nome = self.nome.clone();
        let cargo = self.cargo.clone();

        Funcionario {
            id,
            idade,
            nome,
            cargo
        }
    }
}

impl<'de> Message<'de> for Funcionario{}



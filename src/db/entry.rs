#[derive(Debug, PartialEq, Eq)]
pub struct Funcionario{
    pub id: usize,
    pub idade: u32,
    pub nome: String,
    pub cargo: String,
}

impl Funcionario{
    pub fn new(idade: u32, nome: String, cargo: String) -> Self {
        Funcionario{
            id: 0,
            idade,
            nome,
            cargo,
        }
    }
}


impl Clone for Funcionario{
    fn clone(&self) -> Self {
        let id = self.id.clone();
        let idade = self.idade.clone();
        let nome = self.nome.clone();
        let cargo = self.nome.clone();

        Funcionario {
            id,
            idade,
            nome,
            cargo
        }
    }
}

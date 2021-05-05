use lib::lambda::funcionario::funcionario::Funcionario;
use lib::lambda::message_trait::Message;

fn main() {
    let mut func = Funcionario{
        id: 0,
        idade: 10,
        nome: String::from("testing"),
        cargo: String::from("testing_cargo"),
    };

    println!("{}", func.into_json())
}
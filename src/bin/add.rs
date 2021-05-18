use lib::{
    add,
    lambda::{
        funcionario::{
            funcionario::Funcionario,
        },
        message_trait::Message,
    },
};


fn main() {
    let request = Funcionario::new(665,
        555,
        "Quetzalcoatl",
        "Unknown",
    );

    let message = Message::new_value::<Funcionario>(request).unwrap();
    // let request = format!("{{\"id\":1}}");
    println!("Sending: \n\n{}\n\n", message.get_json());
    let response = add(&message.get_json()[..]);
    println!("{}", response);
}
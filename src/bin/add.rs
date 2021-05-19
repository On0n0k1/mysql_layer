use lib::{
    request_post,
    lambda::{
        funcionario::{
            funcionario::Funcionario,
        },
        message::Message,
    },
};


fn main() {
    let request = Funcionario::new(
        55,
        20,
        "Jotaro Kujo",
        "Stand User",
    );

    let message = Message::new_value::<Funcionario>(request).unwrap();
    // let request = format!("{{\"id\":1}}");
    println!("Sending: \n\n{}\n\n", message.get_json());
    let response = request_post(message);
    println!("{}", response);
}

use lib::{
    // request_post,
    // request_delete,
    request_put,
    lambda::{
        funcionario::{
            // funcionario::Funcionario,
            funcionario::Funcionario,
        },
        message::Message,
    },
};


fn main() {
    let request = Funcionario::new(
        3,
        200,
        "Editedd Name",
        "Edited Role",
    );

    // let message = Message::new_value::<Funcionario>(request).unwrap();
    // let request = DeleteRequest::ById{id: 1};

    let message = Message::new_value::<Funcionario>(request).unwrap();

    // let request = format!("{{\"id\":1}}");
    println!("Sending PUT: \n\n{}\n\n", message.get_json());
    let response = request_put(Some(message));
    println!("{}", response);
}

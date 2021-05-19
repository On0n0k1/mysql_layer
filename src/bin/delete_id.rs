use lib::{
    // request_post,
    request_delete,
    lambda::{
        funcionario::{
            // funcionario::Funcionario,
            delete::{
                DeleteRequest,
            },
        },
        message::Message,
    },
};


fn main() {
    // let request = Funcionario::new(
    //     55,
    //     20,
    //     "Jotaro Kujo",
    //     "Stand User",
    // );

    // let message = Message::new_value::<Funcionario>(request).unwrap();
    let request = DeleteRequest::ById{id: 1};
    let message = Message::new_value::<DeleteRequest>(request).unwrap();

    // let request = format!("{{\"id\":1}}");
    println!("Sending: \n\n{}\n\n", message.get_json());
    let response = request_delete(message);
    println!("{}", response);
}
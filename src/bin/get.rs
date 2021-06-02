use lib::{
    request_get,
    lambda::{
        funcionario::get::ListGetRequest,
        message::Message,
    },
};


fn main() {
    let request = ListGetRequest::Get{id:1};
    let message = Message::new_value::<ListGetRequest>(request).unwrap();
    // let request = format!("{{\"id\":1}}");
    println!("Sending GET: \n\n{}\n\n", message.get_json());
    let response = request_get(Some(message));
    println!("{}", response);
}
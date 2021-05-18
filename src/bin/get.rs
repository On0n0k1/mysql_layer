use lib::{
    get,
    lambda::{
        funcionario::get::ListGetRequest,
        message_trait::Message,
    },
};


fn main() {
    let request = ListGetRequest::Get{id:1};
    let message = Message::new_value::<ListGetRequest>(request).unwrap();
    // let request = format!("{{\"id\":1}}");
    println!("Sending: \n\n{}\n\n", message.get_json());
    let response = get(&message.get_json()[..]);
    println!("{}", response);
}
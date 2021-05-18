use lib::{
    get,
    lambda::{
        funcionario::get::ListGetRequest,
        message_trait::Message,
    },
};

fn main(){
    let request = ListGetRequest::List{start: 0, end: 100, limit: None};
    let message = Message::new_value::<ListGetRequest>(request).unwrap();

    println!("Sending: \n\n{}\n\n", message.get_json());

    let response = get(&message.get_json()[..]);
    // let response = list(Some((0, 100)), None);
    // let response = list(None, None);
    println!("{}", response);
}
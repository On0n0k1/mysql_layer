use lib::{
    request_get,
    lambda::{
        funcionario::get::ListGetRequest,
        message::Message,
    },
};

fn main(){
    let request = ListGetRequest::List{start: 0, end: 100, limit: None};
    let message = Message::new_value::<ListGetRequest>(request).unwrap();

    println!("Sending GET : \n\n{}\n\n", message.get_json());

    let response = request_get(Some(message));
    // let response = list(Some((0, 100)), None);
    // let response = list(None, None);
    println!("{}", response);
}
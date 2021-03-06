use crate::{
    database::{
        dao::DAO,
    },
    lambda::{
        funcionario::funcionario::Funcionario,
        message::Message,
    },
    requests::response::{
        Response,
        ResponseType,
    }
};


pub fn request_post(message: Option<Message>) -> Response{
    // let func = Funcionario::get(id).unwrap();
    // let message = Message::new_json(request);
    let message = match message {
        None => { 
            println!("Failed to parse AWS Event");
            return Response::new(ResponseType::InternalServerError500, None)
        },
        Some(value) => {value},
    };

    // let func = Funcionario::new_from_json(request);
    let func = message.get_value::<Funcionario>();
    let func = match func{
        None => { return Response::new(ResponseType::BadRequest400, None)},
        Some(value) => {value},
    };

    // I will use func later, in case an error happens. That's why I'm cloning it to the add function.
    let result = Funcionario::dao_add(func.clone());

    // let body = AddBody{
    //     funcionario: func,
    // };

    let response = match result {
        Ok(_) => {
            let response = Response::new(            
                ResponseType::Ok200,
                None,
            );
            response
        },
        Err(err) => {
            let message = Message::new_value::<Funcionario>(func);
            let message = match message {
                Ok(value) => value.get_json(),
                Err(_) => {
                    println!("Error converting funcionario into the error message in Add Function.");
                    String::from(" ((Error converting)) ")
                },
            };

            println!("Got an Error when trying to add Funcionario. \n\nvalue: {}\n\n, Error: {}\n\n", message, err);
            let response = Response::new(
                ResponseType::InternalServerError500,
                None,
            );
            response
        },
    };

    response

    // let response = Response::new(
    //     ResponseType::Ok200,
    //     Some(body),
    // );
    // response
}

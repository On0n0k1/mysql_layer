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


pub fn request_put(message: Option<Message>) -> Response{
    // let func = Funcionario::get(id).unwrap();
    // let message = Message::new_json(request);
    let message = match message {
        None => { 
            println!("Failed to parse AWS Event");
            return Response::new(ResponseType::InternalServerError500, None)
        },
        Some(value) => { value },
    };

    // let func = Funcionario::new_from_json(request);
    let func = message.get_value::<Funcionario>();
    let func = match func{
        None => { return Response::new(ResponseType::BadRequest400, None)},
        Some(value) => { value },
    };

    // I will use func later, in case an error happens. That's why I'm cloning it to the add function.
    let result = Funcionario::dao_update(func.clone());

    // let body = AddBody{
    //     funcionario: func,
    // };

    let response = match result {
        Ok(value) => {
            let response = match value{
                None =>{
                    Response::new(
                        ResponseType::Conflict409,
                        None,
                    )
                },
                Some(updated_value) => 
                {
                    let message = Message::new_value::<Funcionario>(updated_value);
                    let message = match message {
                        Err(err) => {
                            println!("Succesfully updated the value, but creating the message to return resulted in an error. err: {}", err);
                            return Response::new(            
                                ResponseType::InternalServerError500,
                                None,
                            );
                        },
                        Ok(unpacked_message) => {
                            println!("Successfully updated: {}\n", unpacked_message.get_json());
                            unpacked_message
                        },
                    };
                    Response::new(            
                        ResponseType::Ok200,
                        Some(message),
                    )
                },
            };
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

            println!("Got an Error when trying to update Funcionario. \n\nvalue: {}\n\n, Error: {}\n\n", message, err);
            let response = Response::new(
                ResponseType::InternalServerError500,
                None,
            );
            response
        },
    };

    response
}

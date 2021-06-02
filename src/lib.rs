pub mod database;
pub mod lambda;
pub mod cli;
pub mod requests;
// pub mod bin;

// pub use lambda::{
//     consultar_funcionario,
//     consultar_funcionarios,
// };

// pub use lambda::funcionario::funcionario::Funcionario;
// pub use database::dao::DAO;

pub use lambda::funcionario::{
    post::request_post,
    get::request_get,
    list::request_list,
    delete::request_delete,
    put::request_put,
};

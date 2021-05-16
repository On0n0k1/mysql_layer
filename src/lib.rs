pub mod database;
pub mod lambda;
pub mod cli;
pub mod requests;

// pub use lambda::{
//     consultar_funcionario,
//     consultar_funcionarios,
// };

// pub use lambda::funcionario::funcionario::Funcionario;
pub use database::dao::DAO;

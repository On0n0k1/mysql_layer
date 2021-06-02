use lib::database::db::DB;

fn main(){
    // if let DB::initial_setup() = Ok(_) => {

    // }
    match DB::new().initial_setup() {
        Ok(_) => { println!(" Successful initial setup. ")}
        Err(err) => { println!("{}", err );}
    }
}
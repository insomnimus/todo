use todo::command::Command;
//use std::process;

fn main() {
    if let Err(e) = Command::run() {
        panic!("error: {:?}", e);
        //process::exit(1);
    }
}

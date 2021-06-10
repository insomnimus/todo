use std::process;
use todo::command::Command;

fn main() {
    if let Err(e) = Command::run() {
        eprintln!("error: {:?}", e);
        process::exit(1);
    }
}

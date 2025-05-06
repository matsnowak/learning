use std::{env::args, fs::File};

mod bytecode;
mod lex;
mod parse;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = args().collect();

    // TODO: better args parsing and error codes
    if args.len() != 2 {
        eprintln!(" Usage: {} script", args[0]);
        return;
    }
    let file = File::open(&args[1]).expect("Script not found");
    let proto = parse::load(file);
    vm::ExeState::new().execute(proto);
}

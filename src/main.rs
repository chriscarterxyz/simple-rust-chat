use std::env;

mod server;
mod client;
mod message;

fn main() {
        
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    match mode.as_str() {
        "client" => client::main(),
        "server" => server::main(),
        &_ => todo!(),
    }

}

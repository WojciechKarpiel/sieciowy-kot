extern crate getopts;

mod errors;
mod options;

use getopts::Options;
use options::prepare_options;
use options::parse_arguments;
use options::print_help;
use options::Mode;

#[derive(Debug)]
pub struct Host(String);
#[derive(Debug)]
pub struct Port(u16);

fn main() {
    let options: Options = prepare_options();
    let arguments: Vec<String> = std::env::args().collect();
    let program_name =  arguments[0].clone();
    match parse_arguments(&options, arguments) {
        Ok(Mode::Listen(port)) => listen(port),
        Ok(Mode::Send(host, port)) => send(host, port),
        Ok(Mode::Help) => print_help(&program_name, &options),
        Err(error) => {
            println!("{}", error);
            print_help(&program_name, &options);
        }
    }
}


fn listen(Port(port): Port) {
    println!("listen {}", port);
}

fn send(Host(host): Host, Port(port): Port) {
    println!("{} {}",host,port);
}




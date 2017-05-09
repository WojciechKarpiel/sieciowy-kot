extern crate getopts;

mod errors;
mod options;

use getopts::Options;
use options::prepare_options;
use options::parse_arguments;
use options::print_help;
use options::Mode;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Error;
use std::io::Write;

#[derive(Debug)]
pub struct Host(String);
#[derive(Debug)]
pub struct Port(u16);

fn main() {
    let options: Options = prepare_options();
    let arguments: Vec<String> = std::env::args().collect();
    let program_name =  arguments[0].clone();
    match parse_arguments(&options, arguments) {
        Ok(Mode::Listen(port)) => with_error_handed(|| listen(port)),
        Ok(Mode::Send(host, port)) => with_error_handed(|| send(host, port)),
        Ok(Mode::Help) => print_help(&program_name, &options),
        Err(error) => {
            writeln!(&mut std::io::stderr(), "{}", error).unwrap();
            print_help(&program_name, &options);
        }
    }
}

fn with_error_handed<F>(procedure: F) 
    where F: FnOnce() -> Result<(), Error>
{ 
    match procedure() {
        Err(error) => writeln!(&mut std::io::stderr(), "{}", error).unwrap(),
        _ => {},
    }
}

fn listen(Port(port): Port)  -> Result<(), Error> {
    let listener = TcpListener::bind(("localhost", port))?;
    let (mut input_stream, _) = listener.accept()?;
    std::io::copy(&mut input_stream, &mut std::io::stdout())?;
    Ok(())
}

fn send(Host(host): Host, Port(port): Port) -> Result<(), Error> {
    let mut output_stream = TcpStream::connect((&host as &str, port))?;
	std::io::copy(&mut std::io::stdin(), &mut output_stream)?;
	Ok(())
}




extern crate getopts;

use getopts::Options;
use std::error::Error;
use std::fmt;

const LISTEN_FLAG_SHORT: &str = "s";
const LISTEN_FLAG_LONG: &str = "słuchajcie";
const LISTEN_FLAG_DESC: &str = "przełączcie w tryb słuchania";
const HELP_FLAG_SHORT: &str = "p";
const HELP_FLAG_LONG: &str = "pomóżcie";
const HELP_FLAG_DESC: &str = "wyświetla ten tekst";
const PORT_OPTION_SHORT: &str = "p";
const PORT_OPTION_LONG: &str = "port";
const PORT_OPTION_DESC: &str =  "słuchajcie/wysyłajcie na port";
const PORT_OPTION_HINT: &str = "1234";
const HOST_OPTION_SHORT: &str = "g";
const HOST_OPTION_LONG: &str = "gość";
const HOST_OPTION_DESC: &str = "wysyłajcie do gościa";
const HOST_OPTION_HINT: &str = "127.0.0.1";

#[derive(Debug)]
enum ProgramArgumentError {
    LackingPortError,
    LackingHostError,
}

impl fmt::Display for ProgramArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.description(), f)
    }
}

impl Error for ProgramArgumentError {
    fn description(&self) -> &str {
        match *self {
          ProgramArgumentError::LackingPortError => "Trzeba podać port!",
          ProgramArgumentError::LackingHostError => "Trzeba podać adres gościa!"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug)]
enum Mode {
    Listen(Port),
    Send(Host, Port),
    Help,
}

#[derive(Debug)]
struct Host(String);
#[derive(Debug)]
struct Port(String);

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

fn prepare_options() -> Options {
    let mut options: Options = Options::new();
    options.optflag(LISTEN_FLAG_SHORT,
                    LISTEN_FLAG_LONG,
                    LISTEN_FLAG_DESC);
    options.optflag(HELP_FLAG_SHORT,
                    HELP_FLAG_LONG,
                    HELP_FLAG_DESC);
    options.optopt(PORT_OPTION_SHORT,
                   PORT_OPTION_LONG,
                   PORT_OPTION_DESC,
                   PORT_OPTION_HINT);
    options.optopt(HOST_OPTION_SHORT,
                   HOST_OPTION_LONG,
                   HOST_OPTION_DESC,
                   HOST_OPTION_HINT);
    options                
}

fn listen(Port(port): Port) {
    println!("listen {}", port);
}

fn send(Host(host): Host, Port(port): Port) {
    println!("{} {}",host,port);
}

fn print_help(program_name: &str, options: &Options) {
    let description = format!("Sposób użycia: {} [OPCJE]", program_name);
    println!("{}", options.usage(&description));
}

fn parse_arguments( options: &Options,arguments: Vec<String>) -> Result<Mode, ProgramArgumentError> {
    let matches = match  options.parse(&arguments[1..]) {
        Ok(m) => m,
        Err(err) =>  panic!(err.to_string())
    };
    
    if matches.opt_present(HELP_FLAG_SHORT) {
        return Ok(Mode::Help)
    }

    let port: Port = match  matches.opt_str(PORT_OPTION_SHORT) {
        Some(port) =>  Port(port),
        None => return Err(ProgramArgumentError::LackingPortError)
    };

    if matches.opt_present(LISTEN_FLAG_SHORT) {
        let host: Host = match matches.opt_str(HOST_OPTION_SHORT) {
            Some(host) => Host(host),
            None => return Err(ProgramArgumentError::LackingHostError)
        };
        Ok(Mode::Send(host, port))
    } else {
        Ok(Mode::Listen(port))
    }
}
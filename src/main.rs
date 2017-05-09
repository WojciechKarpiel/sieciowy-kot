extern crate getopts;

use getopts::Options;
use std::error::Error;
use std::fmt;

const LISTEN_FLAG_SHORT: &str = "s";
const LISTEN_FLAG_LONG: &str = "słuchajcie";
const LISTEN_FLAG_DESC: &str = "przełączcie w tryb słuchania";
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
        match *self {
            ProgramArgumentError::LackingPortError => fmt::Display::fmt("Brak portu", f),
            ProgramArgumentError::LackingHostError => fmt::Display::fmt("Brak gościa", f)
        }
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

enum Mode {
    Listen(Port),
    Send(Host, Port),
}

struct Host(String);
struct Port(String);

fn main() {
    println!("Hello, world!");
    let mode = parse_args();
}

fn parse_args() -> Result<Mode, ProgramArgumentError> {
    let args: Vec<String> = std::env::args().collect();
    let mut options = Options::new();
    options.optflag(LISTEN_FLAG_SHORT,
                    LISTEN_FLAG_LONG,
                    LISTEN_FLAG_DESC);
    options.optopt(PORT_OPTION_SHORT,
                   PORT_OPTION_LONG,
                   PORT_OPTION_DESC,
                   PORT_OPTION_HINT);
    options.optopt(HOST_OPTION_SHORT,
                   HOST_OPTION_LONG,
                   HOST_OPTION_DESC,
                   HOST_OPTION_HINT);
    let matches = match  options.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) =>  panic!(err.to_string())
    };
    
    let port: Port = match  matches.opt_str(PORT_OPTION_SHORT) {
        Some(port) =>  Port(port) ,
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

fn print_help_and_exit() {

}
use getopts::Options;

use Port;
use Host;
use errors::ProgramArgumentError;

const LISTEN_FLAG_SHORT: &str = "s";
const LISTEN_FLAG_LONG: &str = "słuchajcie";
const LISTEN_FLAG_DESC: &str = "przełączcie w tryb słuchania";
const UDP_FLAG_SHORT: &str = "u";
const UDP_FLAG_LONG: &str = "udp";
const UDP_FLAG_DESC: &str = "używajcie UDP zamiast TCP";
const HELP_FLAG_SHORT: &str = "h";
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
pub enum Mode {
    Listen(Protocol, Port),
    Send(Protocol, Host, Port),
    Help,
}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
}


pub fn prepare_options() -> Options {
    let mut options: Options = Options::new();
    options.optflag(LISTEN_FLAG_SHORT,
                    LISTEN_FLAG_LONG,
                    LISTEN_FLAG_DESC);
    options.optflag(UDP_FLAG_SHORT,
                    UDP_FLAG_LONG,
                    UDP_FLAG_DESC);
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

pub fn parse_arguments( options: &Options,arguments: Vec<String>) -> Result<Mode, ProgramArgumentError> {
    let matches = match  options.parse(&arguments[1..]) {
        Ok(m) => m,
        Err(err) =>  panic!(err.to_string())
    };
    
    if matches.opt_present(HELP_FLAG_SHORT) {
        return Ok(Mode::Help)
    }

    let protocol: Protocol = if matches.opt_present(UDP_FLAG_SHORT) {
                                Protocol::UDP
                            } else {
                                Protocol::TCP
                            };

    let port: Port = match  matches.opt_str(PORT_OPTION_SHORT) {
        Some(port) =>  {
           match port.parse::<u16>() {
               Ok(port) => Port(port),
               Err(err) => return Err(ProgramArgumentError::from(err)),
           } 
        },
        None => return Err(ProgramArgumentError::LackingPortError)
    };

    if matches.opt_present(LISTEN_FLAG_SHORT) {
        Ok(Mode::Listen(protocol, port))
    } else {
        let host: Host = match matches.opt_str(HOST_OPTION_SHORT) {
            Some(host) => Host(host),
            None => return Err(ProgramArgumentError::LackingHostError)
        };
        Ok(Mode::Send(protocol, host, port))
    }
}

pub fn print_help(program_name: &str, options: &Options) {
    let description = format!("Sposób użycia: {} [OPCJE]", program_name);
    println!("{}", options.usage(&description));
}
extern crate getopts;

mod errors;
mod options;

use getopts::Options;
use options::prepare_options;
use options::parse_arguments;
use options::print_help;
use options::Mode;
use options::Protocol;
use options::ConnectionOptions;
use options::ProgramToExec;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::io::Error;
use std::io::Write;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug)]
pub struct Host(String);
#[derive(Debug)]
pub struct Port(u16);

const BUF_SIZE : usize = 100;

/*
DOZRO:
* dwukierunkowe strumienie
* wspólna obsługa uruchamiania procesów
*/

fn main() {
    let options: Options = prepare_options();
    let arguments: Vec<String> = std::env::args().collect();
    let program_name = arguments[0].clone();
    match parse_arguments(&options, arguments) {
        Ok(Mode::Listen(connection_options)) => listen(connection_options),
        Ok(Mode::Send(host, connction_options)) => send(host, connction_options),
        Ok(Mode::Help) => print_help(&program_name, &options),
        Err(error) => {
            writeln!(&mut std::io::stderr(), "{}", error).unwrap();
            print_help(&program_name, &options);
        }
    }
}

fn listen(ConnectionOptions(port, protocol, program_to_exec) : ConnectionOptions) {
    match protocol {
        Protocol::TCP => with_error_handed(|| listen_tcp(port, program_to_exec)),
        Protocol::UDP => with_error_handed(|| listen_udp(port)),
    }
}

fn send(host: Host, ConnectionOptions(port, protocol, program_to_exec) : ConnectionOptions) {
    match protocol {
        Protocol::TCP => with_error_handed(|| send_tcp(host, port, program_to_exec)),
        Protocol::UDP => with_error_handed(|| send_udp(host, port)),
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

fn listen_tcp(Port(port): Port, ProgramToExec(program_to_exec): ProgramToExec)  -> Result<(), Error> {
    let listener = TcpListener::bind(("localhost", port))?;
    let (mut tcp_stream, _) = listener.accept()?;
    match program_to_exec {
        Some(program_name) => {
            let process =  Command::new(program_name)
                .stdout(Stdio::piped())
                .spawn()?;
            std::io::copy(&mut process.stdout.unwrap(), &mut tcp_stream)?;
        }
        None => {std::io::copy(&mut tcp_stream, &mut std::io::stdout())?;},
    }
    Ok(())
}

fn send_tcp(Host(host): Host, Port(port): Port, ProgramToExec(program_to_exec): ProgramToExec) -> Result<(), Error> {
    let mut output_stream = TcpStream::connect((&host as &str, port))?;
    match program_to_exec {
        Some(program_name) =>  {
            let process =  Command::new(program_name)
                .stdout(Stdio::piped())
                .spawn()?;
            std::io::copy(&mut process.stdout.unwrap(), &mut output_stream)?;
            },
	    None => {std::io::copy(&mut std::io::stdin(), &mut output_stream)?;},
    };
	Ok(())
}

fn listen_udp(Port(port): Port) -> Result<(), Error> {
    let socket = UdpSocket::bind(("localhost", port))?;
    let mut buf = [0; BUF_SIZE];
    loop{
        let (size, _) = socket.recv_from(&mut buf)?;
        std::io::stdout().write(&buf[0 .. size])?;
    }
}

fn send_udp(Host(host): Host, Port(port): Port) -> Result<(), Error> {
    let socket = UdpSocket::bind(("localhost", 0))?;
    let mut buf = [0; BUF_SIZE];
    loop {
        let size = std::io::stdin().read(&mut buf)?;
        if size == 0 {
            break;
        }
        socket.send_to(&buf[0 .. size], (&host as &str, port))?;
    }
    Ok(())
}
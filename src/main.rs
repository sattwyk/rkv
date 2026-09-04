use std::{
    env,
    io::Read,
    net::{TcpListener, TcpStream},
    process,
};

enum Mode {
    Primary,
    Replica,
}

const PRIMARY_PORT: &str = "127.0.0.1:7000";
const REPLICA_PORT: &str = "127.0.0.1:7001";

fn handle_incoming_stream(stream: &mut TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();

    println!("{buffer}");
}

fn run(mode: Mode) {
    let port = match mode {
        Mode::Primary => PRIMARY_PORT,
        Mode::Replica => REPLICA_PORT,
    };

    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        handle_incoming_stream(&mut stream.unwrap());
    }
}

fn usage() {
    eprintln!("Usage: rkv [-p | --primary | -r | --replica]");
    eprintln!("Defaults to --primary if no argument is given.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = match args.as_slice() {
        [_] => Mode::Primary,
        [_, flag] => match flag.as_str() {
            "-p" | "--primary" => Mode::Primary,
            "-r" | "--replica" => Mode::Replica,
            other => {
                eprintln!("Error: unrecognized argument '{}'", other);
                usage();
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Error: expected at most 1 argument, got {}", args.len() - 1);
            usage();
            process::exit(1);
        }
    };

    run(mode);
}

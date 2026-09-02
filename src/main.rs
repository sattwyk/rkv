use std::env;
use std::process;

enum Mode {
    Primary,
    Replica,
}

fn run(mode: Mode) {
    match mode {
        Mode::Primary => println!("running as a primary node"),
        Mode::Replica => println!("running as a replica node"),
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

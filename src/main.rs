use std::env;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Please only use between --primary , --replica, -p or -r");
        return;
    }

    if args.len() == 1 {
        run(Mode::Primary);
        return;
    }

    let flag = &args[1];

    match flag.as_str() {
        "-p" => run(Mode::Primary),
        "-r" => run(Mode::Replica),
        "--primary" => run(Mode::Primary),
        "--replica" => run(Mode::Replica),
        _ => eprintln!("Please only use between --primary , --replica, -p or -r"),
    }
}

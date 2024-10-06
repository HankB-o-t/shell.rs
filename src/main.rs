use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::env;
use std::path::Path;

fn read(input: &mut String) {
    let _ = stdout().flush();
    stdin().read_line(input).unwrap();
}

const TEXT: [&str; 10] = ["  ██████  ██░ ██ ▓█████  ██▓     ██▓          ██▀███    ██████ ",
                          "▒██    ▒ ▓██░ ██▒▓█   ▀ ▓██▒    ▓██▒         ▓██ ▒ ██▒▒██    ▒ ",
                          "░ ▓██▄   ▒██▀▀██░▒███   ▒██░    ▒██░         ▓██ ░▄█ ▒░ ▓██▄   ",
                          "  ▒   ██▒░▓█ ░██ ▒▓█  ▄ ▒██░    ▒██░         ▒██▀▀█▄    ▒   ██▒",
                          "▒██████▒▒░▓█▒░██▓░▒████▒░██████▒░██████▒ ██▓ ░██▓ ▒██▒▒██████▒▒",
                          "▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒░░ ▒░ ░░ ▒░▓  ░░ ▒░▓  ░ ▒▓▒ ░ ▒▓ ░▒▓░▒ ▒▓▒ ▒ ░",
                          "░ ░▒  ░ ░ ▒ ░▒░ ░ ░ ░  ░░ ░ ▒  ░░ ░ ▒  ░ ░▒    ░▒ ░ ▒░░ ░▒  ░ ░",
                          "░  ░  ░   ░  ░░ ░   ░     ░ ░     ░ ░    ░     ░░   ░ ░  ░  ░  ",
                          "      ░   ░  ░  ░   ░  ░    ░  ░    ░  ░  ░     ░           ░  ",
                          "                                          ░                    "];
// 63
fn main() {
    for row in 0..10 {
        println!("{}", TEXT[row]);
    }
    loop {
        print!("                               $ ");
        let mut input = String::new();
        read(&mut input);

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        if command == "exit" {
            break;
        } 

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait();
            }
        }
    }
}

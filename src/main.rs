use interpreter::eval::eval;
use interpreter::eval::Environment;
use interpreter::parse::parse;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // File input
        let filename = &args[1];
        let source_code = fs::read_to_string(filename).unwrap();
        let tokens = parse(&source_code).unwrap();
        let mut env = &mut Environment {
            arr: [0; 20000],
            index: 0,
        };

        eval(&tokens, &mut env).unwrap();

        println!("{:?}", &env.arr[..31]);
    } else {
        println!("Welcome to the REPL! Enter Ctrl+C to exit.");
        let mut env = Environment {
            arr: [0; 20000],
            index: 0,
        };

        loop {
            let mut input = String::new();

            print!(">> ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let tokens = parse(&input).unwrap();
            eval(&tokens, &mut env).unwrap();
        }
    }
}

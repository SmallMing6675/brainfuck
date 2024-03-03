mod tests;
use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Environment {
    pub arr: [u8; 30000],
    pub index: usize,
}

#[derive(Debug)]
struct Macro {
    pub name: String,
    pub instructions: String,
}

/// Expands Extended Brainfuck files (.bfx) into a single file
fn expand(input: &str, file_path: &PathBuf) -> String {
    let mut macros = Vec::new();
    let macro_re = Regex::new(r"\\([^\\]+)\\([^\\]+)\\").unwrap();
    let comment_re = Regex::new(r"%([^%]+)%").unwrap();
    let import_re = Regex::new(r"@([^@]+)@").unwrap();

    let mut output = comment_re.replace_all(&input, "").to_string();

    let binding = PathBuf::from(file_path);
    let script_path = binding.parent();

    output = import_re
        .replace_all(&output, |caps: &regex::Captures| {
            if let Some(file_name) = caps.get(1) {
                let full_path = script_path.unwrap().join(file_name.as_str());
                if let Ok(file_content) = fs::read_to_string(&full_path) {
                    return file_content;
                }
            }
            caps[0].to_string()
        })
        .to_string();

    for capture in macro_re.captures_iter(&output) {
        let name = capture.get(1).map_or("", |m| m.as_str()).to_string();
        let instructions = expand(capture.get(2).map_or("", |m| m.as_str()), file_path);
        macros.push(Macro { name, instructions });
    }

    for macro_ in macros.iter() {
        let re = Regex::new(&format!(r"/{}/", macro_.name)).unwrap();
        output = re.replace_all(&output, &macro_.instructions).to_string();
    }

    output = macro_re.replace_all(&output, "").to_string();

    output
        .chars()
        .filter(|ch| match ch {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        })
        .collect::<String>()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <File>");
        return;
    }
    let mut env = Environment {
        arr: [0; 30000],
        index: 0,
    };

    let input = fs::read_to_string(&args[1]).expect("Invalid File Path");
    let expanded_code = expand(&input, &PathBuf::from(&args[1]));
    eval(&expanded_code, &mut env);
}

fn eval(input: &str, env: &mut Environment) -> () {
    let input_bytes: Vec<u8> = input.bytes().collect();
    let mut code_ptr: usize = 0;
    let mut input_buffer = String::new();

    while code_ptr < input.len() {
        match input_bytes[code_ptr] as char {
            '>' => env.index += 1,
            '<' => env.index -= 1,
            '+' => env.arr[env.index] = env.arr[env.index].wrapping_add(1),
            '-' => env.arr[env.index] = env.arr[env.index].wrapping_sub(1),
            '.' => println!(
                "{}",
                if env.arr[env.index] > 31 {
                    env.arr[env.index] as char
                } else {
                    char::from_u32(env.arr[env.index] as u32).unwrap()
                }
            ),
            ',' => {
                if input_buffer.is_empty() {
                    io::stdin()
                        .read_line(&mut input_buffer)
                        .expect("Failed to read input");
                }
                env.arr[env.index] = input_buffer.remove(0) as u8;
            }
            '[' => {
                if env.arr[env.index] == 0 {
                    let mut i = code_ptr + 1;
                    let mut stack = 1;
                    while stack > 0 {
                        match input_bytes[i] as char {
                            '[' => stack += 1,
                            ']' => stack -= 1,
                            _ => (),
                        }
                        i += 1;
                    }
                    code_ptr = i - 1;
                    continue;
                }
            }

            ']' => {
                if env.arr[env.index] != 0 {
                    let mut i = code_ptr - 1;
                    let mut stack = 1;
                    while stack > 0 {
                        match input_bytes[i] as char {
                            ']' => stack += 1,
                            '[' => stack -= 1,
                            _ => (),
                        }
                        i -= 1;
                    }
                    code_ptr = i + 1;
                    continue;
                }
            }
            _ => (),
        }

        print!("{}[2J", 27 as char); // Clear terminal screen
        print!("{}[H", 27 as char); // Move cursor to top-left corner

        println!();
        for i in 0..input.len() {
            if i == code_ptr {
                print!("\x1b[1;31m{}\x1b[0m", input_bytes[i] as char);
            } else {
                print!("{}", input_bytes[i] as char);
            }
        }
        println!();

        for i in 0..15 {
            if i == env.index {
                print!("\x1b[1;31m{}\x1b[0m ", env.arr[i]);
            } else {
                print!("{} ", env.arr[i]);
            }
        }

        println!();
        code_ptr += 1;

        thread::sleep(Duration::from_millis(100));
    }
}

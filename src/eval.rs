use crate::parse::Token;

#[derive(Debug)]
pub struct Environment {
    pub arr: [u8; 20000],
    pub index: usize,
}
#[derive(Debug)]
pub enum EvalError {}

pub fn eval(tokens: &[Token], env: &mut Environment) -> Result<(), EvalError> {
    for token in tokens {
        match token {
            Token::MoveLeft => {
                env.index = (env.index + env.arr.len() - 1) % env.arr.len();
            }
            Token::MoveRight => {
                env.index = (env.index + 1) % env.arr.len();
            }
            Token::Output => println!("{}", env.arr[env.index].to_string(),),
            Token::Input => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                env.arr[env.index] = input.chars().next().unwrap() as u8
            }
            Token::Increment => {
                env.arr[env.index] = env.arr[env.index].wrapping_add(1);
            }
            Token::Decrement => {
                env.arr[env.index] = env.arr[env.index].wrapping_sub(1);
            }
            Token::Block(tokens) => {
                while env.arr[env.index] != 0 {
                    eval(&tokens, env)?;
                }
            }
            Token::Empty => unreachable!(),
        }
    }
    Ok(())
}

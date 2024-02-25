#[derive(PartialEq, Debug)]
pub enum Token {
    Decrement,
    Increment,
    MoveLeft,
    MoveRight,
    Output,
    Input,
    Block(Vec<Token>),
    Empty,
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    EndBlock,
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
    parse_input(input)
}

fn parse_input(input: &str) -> Result<Vec<Token>, ParseError> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0usize;
    loop {
        let (token, i) = match_char(&chars, index)?;
        if token != Token::Empty {
            tokens.push(token);
        }
        index = i;
        if index >= chars.len() {
            break;
        }
    }
    Ok(tokens)
}

fn match_char(chars: &[char], index: usize) -> Result<(Token, usize), ParseError> {
    if index >= chars.len() {
        return Err(ParseError::EndBlock);
    }

    Ok(match chars[index] {
        '-' => (Token::Decrement, index + 1),
        '+' => (Token::Increment, index + 1),
        '<' => (Token::MoveLeft, index + 1),
        '>' => (Token::MoveRight, index + 1),
        '.' => (Token::Output, index + 1),
        ',' => (Token::Input, index + 1),
        '[' => {
            let mut tokens = Vec::new();
            let mut i = index + 1;
            while chars[i] != ']' {
                let token = match_char(chars, i);
                match token {
                    Ok((token, index)) => {
                        i = index;
                        if token != Token::Empty {
                            tokens.push(token);
                        }
                    }
                    Err(ParseError::EndBlock) => return Err(ParseError::EndBlock),
                }
            }
            (Token::Block(tokens), i + 1)
        }
        ']' => return Err(ParseError::EndBlock),
        _ => (Token::Empty, index + 1),
    })
}

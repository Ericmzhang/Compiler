use std::fs;
use std::iter;
use std::iter::from_fn;

use crate::tokens::Token;

pub fn lex(input_file: &String) -> Vec<Token> {
    // Read the file content into a String
    let file_content = fs::read_to_string(input_file)
        .expect("panic!ed to read the file");

    // The file content is now stored in `file_content`
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = file_content.chars().peekable();

    // LEXING
    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ';' => tokens.push(Token::SemiColon),
            '-' => tokens.push(Token::Negation),
            '~' => tokens.push(Token::BitwiseComplement),
            '!' => tokens.push(Token::LogicalNegation),
            '+' => tokens.push(Token::Addition),
            '*' => tokens.push(Token::Multiplication),
            '/' => tokens.push(Token::Division),
            '0'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::Number(n));
            },

            'A'..='z' => {
                let n: String = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_alphabetic())))
                    .collect::<String>();

                let n_string = n.to_string();
                match n_string.as_str() {
                    "return" => {
                        // Make sure 'return' is not followed by a number
                        if iter.peek().map_or(false, |&next| next.is_digit(10)) {
                            panic!("Invalid token: 'return' followed by a digit");
                        }
                        tokens.push(Token::Return);
                    },
                    "int" => {
                        // Make sure 'int' is not followed by a number
                        if iter.peek().map_or(false, |&next| next.is_digit(10)) {
                            panic!("Invalid token: 'int' followed by a digit");
                        }
                        tokens.push(Token::Int);
                    },
                    _ => tokens.push(Token::Identifier(n_string)),
                }
            }
            
            _ => {
                panic!("unrecognized char");
            }
        }
    }

    tokens.push(Token::EOF);
    for token in &tokens {
        println!("{:?}", token); // This will print each token in Debug format
    }
    println!();
    return tokens;
}
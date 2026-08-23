use std::any::{Any, TypeId};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Token {
    Int32Literal(String),
    Keyword(String),
    Separator(String),
    Operator(String),
    Identifier(String),
}

impl Token {
    fn id(&self) -> usize {
        match (self) {
            Token::Int32Literal(_) => 0,
            Token::Keyword(_) => 1,
            Token::Separator(_) => 2,
            Token::Operator(_) => 3,
            Token::Identifier(_) => 4,
        }
    }

    pub fn content(&self) -> &str {
        match (self) {
            Token::Int32Literal(content) => content.as_str(),
            Token::Keyword(content) => content.as_str(),
            Token::Separator(content) => content.as_str(),
            Token::Operator(content) => content.as_str(),
            Token::Identifier(content) => content.as_str(),
        }
    }

    fn create(id: usize, content: String) -> Token {
        match(id) {
            0 => Token::Int32Literal(content),
            1 => Token::Keyword(content),
            2 => Token::Separator(content),
            3 => Token::Operator(content),
            4 => Token::Identifier(content),
            _ => panic!(),
        }
    }


}

pub fn tokenize(mut src: BufReader<File>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // no global variables :( so precompile regexes here
    let regexes = vec![
        Regex::new("^\\d+").unwrap(), // int
        // TODO fix the parsing for this to allow semicolons but not consume them
        //  idk if this is possible w/out look-ahead support :( (maybe look into extern libs?)
        Regex::new("^(?:i32|let|return|if|else|while|break|continue|for|do)").unwrap(), // keyword
        Regex::new("^[{}();,]").unwrap(), // separator
        Regex::new("^(?:--|->|&&|\\|\\||==|!=|<=|>=|\\+=|-=|/=|\\*=|\\+\\+|[-~!+*/<>=?:])").unwrap(), // operator
        Regex::new("^\\w+").unwrap(), // identifier
    ];
    let line = &mut String::new();
    while (src.read_line(line).is_ok_and(|result| result != 0)) {
        if (!line.is_ascii()) {
            panic!("Files can only use ASCII characters")
        }
        line.push('\n');
        let line_str = line.as_str();
        let mut index: usize = 0;
        while (index < line.len()) {
            // skip whitespace
            while (index < line.len() && line.as_bytes()[index].is_ascii_whitespace()) {
                index += 1;
            }
            if (index >= line.len()) {
                break;
            }
            let result = next_token(line_str, &mut index, &regexes).unwrap();
            tokens.push(result);
        }

        line.clear();
    }
    return tokens;
}

fn next_token(line: &str, index: &mut usize, regexes: &Vec<Regex>) -> Option<Token> {
    let slice: &str = &line[*index..];

    for i in 0..regexes.len() {
        let result = regexes[i].find(slice);
        if (result.is_some()) {
            let result_string = result.unwrap().as_str();
            *index += result_string.len();
            return Some(Token::create(i, String::from(result_string.trim_ascii())));
        }
    }
    return None
}

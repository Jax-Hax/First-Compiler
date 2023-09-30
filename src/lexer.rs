use std::str::Chars;
#[derive(Debug)]
pub enum Token {
    Eof,
    Fn,
    Identifier(String),
    Number(f32),
    Other(char)
}
pub fn get_tok(input_file: &mut Chars) -> Token {
    let mut last_char: char = ' ';
    let mut identifier_str: String = "".to_string();
    while last_char == ' ' || last_char == '\n' || last_char == '\r' {
        match input_file.next() {
            Some(char) => last_char = char,
            None => return Token::Eof,
        }
    }
    if last_char.is_alphabetic() {
        identifier_str.push(last_char);
        match input_file.next() {
            Some(char) => last_char = char,
            None => {
                if identifier_str == "fn" {
                    return Token::Fn;
                }
                return Token::Identifier(identifier_str);
            }
        }
        while last_char.is_alphanumeric() {
            identifier_str.push(last_char);
            match input_file.next() {
                Some(char) => last_char = char,
                None => {
                    if identifier_str == "fn" {
                        return Token::Fn;
                    }
                    return Token::Identifier(identifier_str);
                }
            }
        }
        if identifier_str == "fn" {
            return Token::Fn;
        }
        return Token::Identifier(identifier_str);
    }
    if last_char.is_numeric() || last_char == '.' {
        let mut num_str: String = "".to_string();
        loop {
            num_str.push(last_char);
            last_char = input_file.next().unwrap();
            if !last_char.is_numeric() && !(last_char == '.') {
                break;
            }
        }
        return Token::Number(num_str.parse().unwrap());
    }
    if last_char == '#' {
        loop {
            let is_eof = input_file.next();
            match is_eof {
                Some(char) => {
                    if char == '\n' || char == '\r' {
                        return get_tok(input_file);
                    }
                }
                None => return Token::Eof,
            }
        }
    }
    return Token::Other(last_char)
}

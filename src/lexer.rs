use std::str::Chars;
#[derive(Debug)]
pub enum Token {
    Eof,
    Fn,
    Identifier(String),
    Number(f32),
}
pub fn get_tok(input_file: &mut Chars) -> Token {
    let mut last_char: char = ' ';
    let mut identifier_str: String = "".to_string();
    while last_char == ' ' {
        last_char = input_file.next().unwrap(); // handle the unwrap
    }
    if last_char.is_alphabetic() {
        identifier_str.push(last_char);
        last_char = input_file.next().unwrap();
        while last_char.is_alphanumeric() {
            identifier_str.push(last_char);
            last_char = input_file.next().unwrap();
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
            last_char = input_file.next().unwrap(); //handle unwrap
            if last_char.is_numeric() || last_char == '.' {
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
    Token::Eof
}

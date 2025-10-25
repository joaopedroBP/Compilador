use std::fs::File;
use std::io::Read;

pub struct Token {
    tipe: String,
    lexeme: String,
}

impl Token {
    pub fn new(tipe: &str, lexeme: &str) -> Token {
        Token {
            tipe: tipe.to_string(),
            lexeme: lexeme.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("< {} , {} >", self.tipe, self.lexeme)
    }
}

fn is_math_operator(code: &str) -> Token {
    if code.len() > 1 {
        return Token::new("Err", "???");
    }

    let code_character: char = code.chars().next().unwrap_or('$');

    match code_character {
        '+' => Token::new("Sum Operator", "+"),
        '-' => Token::new("Sub Operator", "-"),
        '*' => Token::new("Mult Operator", "*"),
        '/' => Token::new("Div Operator", "/"),
        '$' => Token::new("EOF", "$"),
        '%' => Token::new("Percentage", "%"),
        _ => Token::new("Err", "???"),
    }
}

fn is_valid_integer(code: &str) -> Token {
    let mut code_characters = code.chars();
    let mut current_character = code_characters.next().unwrap_or('$');

    if current_character == '$' {
        return Token::new("EOF", "$");
    }

    while current_character != '$' {
        if !current_character.is_ascii_digit() {
            return Token::new("Err", "???");
        }
        current_character = code_characters.next().unwrap_or('$');
    }
    Token::new("Integer", code)
}

fn is_valid_id(code: &str) -> Token {
    let mut code_characters = code.chars();
    let mut current_character = code_characters.next().unwrap_or('$');

    if current_character == '$' {
        return Token::new("EOF", "$");
    }

    if !current_character.is_alphabetic() {
        return Token::new("Err", "???");
    }

    current_character = code_characters.next().unwrap_or('$');
    while current_character != '$' {
        if !current_character.is_alphanumeric() && current_character != '_' {
            return Token::new("Err", "???");
        }

        current_character = code_characters.next().unwrap_or('$');
    }

    Token::new("ID", code)
}

fn is_valid_comment(code: &str) -> Token {
    if code.is_empty() {
        return Token::new("EOF", "$");
    }
    if code.starts_with("#") && code.ends_with("#") {
        return Token::new("Comentario", code);
    } else {
        return Token::new("Err", "???");
    }
}

fn is_token_separator(code: char) -> bool {
    matches!(code, ' ')
}

fn is_reserved_name(code: &str) -> Token {
    match code {
        "println" => Token::new("Reserved", "println"),
        "function" => Token::new("Reserved", "function"),
        "INT" => Token::new("Reserved_INT", "INT"),
        "FLOAT" => Token::new("Reserved_FLOAT", "FLOAT"),
        "CHAR" => Token::new("Reserved_CHAR", "CHAR"),
        "scanln" => Token::new("Reserved", "scanln"),
        "VOID" => Token::new("Reserved_VOID", "VOID"),
        "if" => Token::new("Reserved", "if"),
        "else" => Token::new("Reserved", "else"),
        "while" => Token::new("Reserved", "while"),
        "for" => Token::new("Reserved", "for"),
        "break" => Token::new("Reserved", "break"),
        "return" => Token::new("Reserved", "return"),
        "struct" => Token::new("Reserved", "struct"),
        "TRUE" => Token::new("Reserved_TRUE", "TRUE"),
        "FALSE" => Token::new("Reserved_FALSE", "FALSE"),
        "continue" => Token::new("Reserved", "continue"),
        "main" => Token::new("Reserved", "main"),
        _ => Token::new("Err", "???"),
    }
}

fn is_string(code: &str) -> Token {
    if code.starts_with("\"") && code.ends_with("\"") {
        return Token::new("string", code);
    } else {
        return Token::new("Err", "???");
    }
}

fn is_character(code: &str) -> Token {
    if code.starts_with("\'") && code.ends_with("\'") && code.len() == 3 {
        return Token::new("character", code);
    } else {
        return Token::new("Err", "???");
    }
}

fn is_special_character(code: &str) -> Token {
    let c = code.chars().next().unwrap_or('$');
    match c {
        '=' => Token::new("equal_sign", "="),
        '!' => Token::new("exclamation", "!"),
        '%' => Token::new("percent", "%"),
        '(' => Token::new("opening_parenthesis", "("),
        ')' => Token::new("closing_parenthesis", ")"),
        '{' => Token::new("opening_curly_brackets", "{"),
        '}' => Token::new("closing_curly_brackets", "}"),
        ';' => Token::new("end_of_opperation", ";"),
        '$' => Token::new("EOF", "$"),
        ':' => Token::new("type_assign", ":"),
        '[' => Token::new("opening_brackets", "["),
        ']' => Token::new("closing_brackets", "]"),
        '&' => Token::new("address_of", "&"),
        '\\' => Token::new("backslash", "\\"),
        '|' => Token::new("colum", "|"),
        ',' => Token::new("coma", ","),
        '.' => Token::new("dot", "."),
        '>' => Token::new("greater", ">"),
        '<' => Token::new("lesser", "<"),
        _ => Token::new("Err", "???"),
    }
}

fn is_floatin_point(code: &str) -> Token {
    let parts: Vec<&str> = code.split('.').collect();

    if parts.len() != 2 {
        return Token::new("Err", "???");
    }

    let before_dot = parts[0];
    let after_dot = parts[1];

    if before_dot.is_empty() || after_dot.is_empty() {
        return Token::new("Err", "???");
    }

    for part in parts {
        if !part.chars().all(|character| character.is_ascii_digit()) {
            return Token::new("Err", "???");
        }
    }

    return Token::new("Floating_Point", code);
}

fn error(code: &str) -> Token {
    panic!("Token not recognized {}", code);
}

fn is_valid_token(code: &str) -> Token {
    for validator in [
        is_reserved_name,
        is_floatin_point,
        is_math_operator,
        is_valid_id,
        is_valid_integer,
        is_valid_comment,
        is_character,
        is_string,
        is_special_character,
    ] {
        let evaluated_token = validator(code);
        if evaluated_token.tipe != "Err" {
            return evaluated_token;
        }
    }

    error(code)
}

pub fn get_tokens(mut code: File) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut file_to_string = String::new();
    code.read_to_string(&mut file_to_string).unwrap();

    let mut code_characters = file_to_string.chars();
    let mut accumulator = String::new();
    let mut in_comment: bool = false;
    let mut in_string: bool = false;

    while let Some(character) = code_characters.next() {
        match character {
            '#' if !in_comment && !in_string => {
                if !accumulator.is_empty() {
                    tokens.push(is_valid_token(&accumulator));
                    accumulator.clear();
                }
                in_comment = true;
                accumulator.push(character);
            }

            '#' if in_comment != in_string => {
                in_comment = false;
                accumulator.push(character);
                tokens.push(is_valid_token(&accumulator));
                accumulator.clear();
            }

            '\"' => {
                if !in_string {
                    in_string = true;
                    if !accumulator.is_empty() {
                        tokens.push(is_valid_token(&accumulator));
                        accumulator.clear();
                    }
                    accumulator.push(character);
                } else {
                    in_string = false;
                    accumulator.push(character);
                    tokens.push(is_valid_token(&accumulator));
                    accumulator.clear();
                }
            }

            character if is_token_separator(character) => {
                if in_comment || in_string {
                    accumulator.push(character);
                } else if !accumulator.is_empty() {
                    tokens.push(is_valid_token(&accumulator));
                    accumulator.clear();
                }
            }

            '\n' => {
                if !in_comment && !in_string {
                    if !accumulator.is_empty() {
                        tokens.push(is_valid_token(&accumulator));
                        accumulator.clear();
                    }
                } else {
                    accumulator.push('\n');
                }
            }

            character
                if character.is_alphanumeric()
                    || character == '_'
                    || character == '.'
                    || character == '\'' =>
            {
                accumulator.push(character);
            }

            character if is_special_character(&character.to_string()).tipe != "Err" => {
                if in_comment || in_string {
                    accumulator.push(character)
                } else {
                    if !accumulator.is_empty() {
                        tokens.push(is_valid_token(&accumulator));
                        accumulator.clear();
                    }
                    tokens.push(is_special_character(&character.to_string()))
                }
            }

            character if is_math_operator(&character.to_string()).tipe != "Err" => {
                if in_comment || in_string {
                    accumulator.push(character)
                } else {
                    if !accumulator.is_empty() {
                        tokens.push(is_valid_token(&accumulator));
                        accumulator.clear();
                    }
                    tokens.push(is_math_operator(&character.to_string()))
                }
            }
            _ => {
                if !accumulator.is_empty() {
                    tokens.push(is_valid_token(&accumulator));
                    accumulator.clear();
                }

                let t = is_valid_token(&character.to_string());

                if t.tipe == "EOF" {
                    tokens.push(t);
                    break;
                }

                tokens.push(t);
            }
        }
    }

    tokens.push(Token::new("EOF", "$"));
    if !accumulator.is_empty() {
        tokens.push(is_valid_token(&accumulator));
        accumulator.clear();
    }

    tokens
}

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
        '+' => Token::new("Math Operator", "+"),
        '-' => Token::new("Math Operator", "-"),
        '*' => Token::new("Math Operator", "*"),
        '/' => Token::new("Math Operator", "/"),
        '$' => Token::new("EOF", "$"),
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
        if !current_character.is_numeric() {
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
    let first_character = code.chars().next().unwrap_or('$');
    let last_character = code.chars().last().unwrap_or('\0');

    if first_character == '$' {
        return Token::new("EOF", "$");
    }

    if first_character != '#' {
        return Token::new("Err", "???");
    }

    if last_character != '#' {
        return Token::new("Err", "???");
    }

    Token::new("Comentario", code)
}

fn is_token_separator(code: char) -> bool {
    matches!(code, ' ')
}

fn error(code: &str) {
    panic!("Token not recognized {}", code);
}

fn is_valid_token(code: &str) -> Token {
    let mut evaluated_token: Token;

    evaluated_token = is_math_operator(code);
    if evaluated_token.tipe != "Err" {
        return evaluated_token;
    }

    evaluated_token = is_valid_id(code);
    if evaluated_token.tipe != "Err" {
        return evaluated_token;
    }

    evaluated_token = is_valid_integer(code);
    if evaluated_token.tipe != "Err" {
        return evaluated_token;
    }

    evaluated_token = is_valid_comment(code);
    if evaluated_token.tipe != "Err" {
        return evaluated_token;
    }

    if evaluated_token.tipe == "Err" {
        error(code);
    }

    return evaluated_token;
}

pub fn get_tokens(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut code_characters = code.chars();
    let mut current_character = code_characters.next().unwrap_or('$');

    let mut code_string: String = String::from("");
    let mut comment: bool = false;

    loop {
        let t: Token;

        if current_character == '#' && !comment {
            comment = true;
            code_string.push(current_character);
            current_character = code_characters.next().unwrap_or('$');
            continue;
        }

        if current_character == '#' && comment {
            comment = false;
            code_string.push(current_character);
            if code_string.len() >= 1 {
                tokens.push(is_valid_token(&code_string));
                code_string.clear();
            }
            continue;
        }

        if is_token_separator(current_character) {
            if !comment {
                if code_string.len() >= 1 {
                    tokens.push(is_valid_token(&code_string));
                    code_string.clear();
                }

                current_character = code_characters.next().unwrap_or('$');
                continue;
            } else {
                code_string.push(current_character);
                continue;
            }
        } else if current_character.is_alphanumeric() || current_character == '_' {
            code_string.push(current_character);
            current_character = code_characters.next().unwrap_or('$');

            continue;
        } else {
            let tc = current_character.to_string();
            t = is_valid_token(&tc);
        }

        if t.tipe == "EOF" {
            if code_string.len() >= 1 {
                let t = is_valid_token(&code_string);
                tokens.push(t);
            }
            break;
        }

        tokens.push(t);
        current_character = code_characters.next().unwrap_or('$');
    }

    tokens
}

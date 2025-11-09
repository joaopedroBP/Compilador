use std::fs::File;
use std::io::Read;

pub struct Token {
    pub tipe: String,
    pub lexeme: String,
    pub linha: usize,
    pub coluna: usize,
}

impl Token {
    pub fn new(tipe: &str, lexeme: &str) -> Token {
        Token {
            tipe: tipe.to_string(),
            lexeme: lexeme.to_string(),
            linha: 0,
            coluna: 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "< {} , {} > linha: {}, coluna:{}",
            self.tipe, self.lexeme, self.linha, self.coluna
        )
    }

    pub fn add_pos(token: Token, linha: usize, coluna: usize) -> Token {
        let mut aux_token: Token = Token::new("", "");
        aux_token.tipe = token.tipe;
        aux_token.lexeme = token.lexeme;
        aux_token.linha = linha;
        aux_token.coluna = coluna;
        aux_token
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

    if current_character == '-' {
        current_character = code_characters.next().unwrap_or('$');
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
        "call" => Token::new("Reserved_call", "call"),
        "println" => Token::new("Reserved_println", "println"),
        "function" => Token::new("Reserved_function", "function"),
        "INT" => Token::new("Reserved_INT", "INT"),
        "FLOAT" => Token::new("Reserved_FLOAT", "FLOAT"),
        "CHAR" => Token::new("Reserved_CHAR", "CHAR"),
        "scanln" => Token::new("Reserved_scanln", "scanln"),
        "VOID" => Token::new("Reserved_VOID", "VOID"),
        "if" => Token::new("Reserved_if", "if"),
        "else" => Token::new("Reserved_if", "else"),
        "while" => Token::new("Reserved_while", "while"),
        "BOOL" => Token::new("Reserved_BOOL", "BOOL"),
        "for" => Token::new("Reserved_for", "for"),
        "break" => Token::new("Reserved_break", "break"),
        "return" => Token::new("Reserved_return", "return"),
        "struct" => Token::new("Reserved_struct", "struct"),
        "TRUE" => Token::new("Reserved_TRUE", "TRUE"),
        "FALSE" => Token::new("Reserved_FALSE", "FALSE"),
        "continue" => Token::new("Reserved_continue", "continue"),
        "main" => Token::new("Reserved_main", "main"),
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

    let mut before_dot = parts[0];
    let after_dot = parts[1];

    if before_dot.starts_with('-') {
        before_dot = &before_dot[1..];
    }

    if before_dot.is_empty() || after_dot.is_empty() {
        return Token::new("Err", "???");
    }

    for part in [before_dot,after_dot] {
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

    let mut linha: usize = 1;
    let mut coluna: usize = 1;

    while let Some(character) = code_characters.next() {
        match character {
            '#' if !in_comment && !in_string => {
                if !accumulator.is_empty() {
                    tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                    accumulator.clear();
                }
                in_comment = true;
                accumulator.push(character);
            }

            '#' if in_comment != in_string => {
                in_comment = false;
                accumulator.push(character);
                tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                accumulator.clear();
            }

            '\"' => {
                if !in_string {
                    in_string = true;
                    if !accumulator.is_empty() {
                        tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                        accumulator.clear();
                    }
                    accumulator.push(character);
                } else {
                    in_string = false;
                    accumulator.push(character);
                    tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                    accumulator.clear();
                }
            }

            character if is_token_separator(character) => {
                if in_comment || in_string {
                    accumulator.push(character);
                } else if !accumulator.is_empty() {
                    tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                    accumulator.clear();
                }
            }

            '\n' => {
                linha += 1;
                coluna = 1;
                if !in_comment && !in_string {
                    if !accumulator.is_empty() {
                        tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
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
                        tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                        accumulator.clear();
                    }
                    tokens.push(Token::add_pos(
                        is_special_character(&character.to_string()),
                        linha,
                        coluna,
                    ))
                }
            }

            character if is_math_operator(&character.to_string()).tipe != "Err" => {
                if in_comment || in_string {
                    accumulator.push(character)
                } else {
                    if !accumulator.is_empty() {
                        tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                        accumulator.clear();
                    }
                    if character == '-' {
                        let next_char = code_characters.clone().next();

                        let mut last_tipe = "";
                        let mut last_lexeme = "";

                        if let Some(t) = tokens.last() {
                            last_tipe = &t.tipe;
                            last_lexeme = &t.lexeme;
                        }

                        if !(last_tipe == "Integer"
                            || last_tipe == "Floating_Point"
                            || last_tipe == "ID"
                            || last_lexeme == ")")
                        {
                            if let Some(proximo) = next_char {
                                if proximo.is_ascii_digit() {
                                    accumulator.push('-');
                                    continue;
                                }
                            }
                        }
                    }
                    tokens.push(Token::add_pos(
                        is_math_operator(&character.to_string()),
                        linha,
                        coluna,
                    ));
                }
            }
            _ => {
                if !accumulator.is_empty() {
                    tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
                    accumulator.clear();
                }

                let t = is_valid_token(&character.to_string());

                if t.tipe == "EOF" {
                    tokens.push(Token::add_pos(t, linha, coluna));
                    break;
                }

                tokens.push(Token::add_pos(t, linha, coluna));
            }
        }
        coluna += 1;
    }

    tokens.push(Token::add_pos(Token::new("EOF", "$"), linha, coluna));
    if !accumulator.is_empty() {
        tokens.push(Token::add_pos(is_valid_token(&accumulator), linha, coluna));
        accumulator.clear();
    }

    tokens
}

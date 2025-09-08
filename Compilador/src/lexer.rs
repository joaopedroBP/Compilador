pub struct Token {
  tipe: String,
  lexeme: String,
}

struct MathOperator {}
struct Digit {}
struct Id {}

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

impl MathOperator {
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
}

impl Digit {
  fn is_digit(code: &str) -> Token {
    if code == "$" {
      return Token::new("EOF", "$");
    }

    if code.len() > 1 {
      return Token::new("Err", "???");
    }

    let code_character: char = code.chars().next().unwrap_or('$');

    match code_character.to_digit(10) {
      Some(digito) if (0..=9).contains(&digito) => {
        Token::new("Digito", &digito.to_string())
      }
      _ => Token::new("Err", "???"),
    }
  }
}

impl Id{
  fn is_character(code: char) -> bool {

    match code {
      character if ('a'..='z').contains(&character)
        || ('A'..='Z').contains(&character) => {true}
      _ => false,
    }
  }

  fn is_valid_id(code: &str) -> Token{
    let mut code_characters = code.chars();
    let mut current_character = code_characters.next().unwrap_or('$');

    if current_character == '$' {return Token::new("EOF","$")}

    if !Self::is_character(current_character) {
      return Token::new("Err", "???")
    }
    
    current_character = code_characters.next().unwrap_or('$');
    while current_character != '$'{
      if !current_character.is_alphanumeric() && current_character != '_'{
        return Token::new("Err", "???");
      }
      
      current_character = code_characters.next().unwrap_or('$');
    }

    Token::new("ID",code)
  }
}

fn is_token_separator(code: char) -> bool {
  matches!(code, ' ' | '\n')
}

fn error(code: &str) {
  panic!("Token not recognized {}", code);
}

fn is_valid_token(code: &str) -> Token {
  let mut evaluated_token: Token;

  evaluated_token = MathOperator::is_math_operator(code);
  if evaluated_token.tipe != "Err" {return evaluated_token}

  evaluated_token = Digit::is_digit(code);
  if evaluated_token.tipe != "Err"{return evaluated_token}

  evaluated_token = Id::is_valid_id(code);
  if evaluated_token.tipe == "Err" {
    error(code);
  }

  evaluated_token
}

pub fn get_tokens(code: String) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let mut code_characters = code.chars();
  let mut current_character = code_characters.next().unwrap_or('$');

  let mut code_string: String = String::from("");
  loop {

    let t: Token;

    if is_token_separator(current_character) {

      if code_string.len() >= 1 {
        tokens.push(is_valid_token(&code_string));
        code_string.clear();
      }

      current_character = code_characters.next().unwrap_or('$');
      continue;
    }

    else if current_character.is_alphanumeric() || current_character == '_'{
      code_string.push(current_character);
      current_character = code_characters.next().unwrap_or('$');
      continue;
    }

    else{
      let tc = current_character.to_string();
      t = is_valid_token(&tc);
    }

    if t.tipe == "EOF" {
      if code_string.len() >= 1{
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

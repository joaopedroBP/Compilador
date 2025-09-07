 
pub struct Token{
  tipe:String,
  lexeme:String,
}

struct MathOperator{
}

impl Token{
  pub fn new(tipe:&str, lexeme:&str) -> Token{
    Token{
      tipe : tipe.to_string(),
      lexeme : lexeme.to_string(),
    }
  } 

  pub fn to_string(&self) -> String{
    format!("<{},{}>", self.tipe,self.lexeme)
  }
}


impl MathOperator{
  fn is_math_operator(code: char) -> Token{
  //let code_length = code.len();

  //if code_length > 1{
    //return Token::new("EOF","$")
  //}
  //let code_character : char = code.chars().next().unwrap_or('$');
  
    match code{
      '+' => Token::new("Math Operator","+"),
      '-' => Token::new("Math Operator", "-"),
      '*' => Token::new("Math Operator", "*"),
      '/' => Token::new("Math Operator", "/"),
      '$' => Token::new("EOF","$"),
      _=> Token::new("Err","$???"),
    }
  }
}

fn is_token_separator(code:char) -> bool{
  matches!(code, ' ' | '\n')
}

fn error(code:char){
  panic!("Token not recognized {}", code);
}

fn is_valid_token(code:char ) -> Token {
  let evaluated_token : Token;
  evaluated_token = MathOperator::is_math_operator(code);
  
  if evaluated_token.tipe == "Err"{
    error(code)
  }

  evaluated_token  
     
}

pub fn list_tokens(code:String) -> Vec<Token>{
  let mut tokens:Vec<Token> = Vec::new();  
  let mut code_characters = code.chars();
  let mut current_character = code_characters.next().unwrap_or('$');
  
  loop{
    if is_token_separator(current_character){
      current_character = code_characters.next().unwrap_or('$');
      continue 
    }
    let t = is_valid_token(current_character); 

    if t.tipe == "EOF" {break};

    tokens.push(t);
    current_character = code_characters.next().unwrap_or('$');
  }

  tokens
}


    

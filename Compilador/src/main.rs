mod lexer;

fn main() {
  let teste : String = String::from("1 a 2 + - -/A A A ");
  let lista : Vec<lexer::Token> =lexer::get_tokens(teste);

  for token in lista{
    println!("{}", token.to_string());
  }
  
}

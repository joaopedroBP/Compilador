mod lexer;

fn main() {
  let teste : String = String::from("++ +");
  let lista : Vec<lexer::Token> =lexer::list_tokens(teste);

  for token in lista{
    println!("{}", token.to_string());
  }
  
}

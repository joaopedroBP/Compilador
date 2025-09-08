mod lexer;

fn main() {
  let teste : String = String::from("Variavel x 1 ++ Vari_avel");
  let lista : Vec<lexer::Token> =lexer::get_tokens(teste);

  for token in lista{
    println!("{}", token.to_string());
  }
  
}

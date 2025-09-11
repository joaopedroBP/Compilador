mod lexer;
use std::io::Write;
use std::fs::File;
fn main() {
{
    let mut file = File::create("Code.txt").unwrap();
    file.write(b"Hello World").unwrap();
}

  let file = File::open("Code.txt").unwrap();
  let lista: Vec<lexer::Token> = lexer::get_tokens(file);

  for token in lista{
    println!("{}", token.to_string());
  }
  
}

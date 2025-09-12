mod lexer;
use std::fs::File;
//use std::io::Write;
fn main() {
    //    {
    //        let mut file = File::create("Code.txt").unwrap();
    //        file.write(b"Hello World\n this is another line").unwrap();
    //    }

    let file = File::open("Code.txt").unwrap();
    let lista: Vec<lexer::Token> = lexer::get_tokens(file);

    for token in lista {
        println!("{}", token.to_string());
    }
}

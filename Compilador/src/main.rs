mod arvore;
mod lexer;
mod parser;
mod translate;
use arvore::Node;
use arvore::Tree;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

//use std::io::Write;
fn main() {
    //    {
    //        let mut file = File::create("Code.txt").unwrap();
    //        file.write(b"Hello World\n this is another line").unwrap();
    //    }

    let file = File::open("Code.txt").unwrap();
    let lista: Vec<lexer::Token> = lexer::get_tokens(file);

    for token in &lista {
        println!("{}", token.to_string());
    }

    let (parser_result, token_tree) = parser::parser(lista);

    if parser_result {
        println!("codigo válido");
        token_tree.print_tree();
        translate::salvar_e_formatar(&token_tree.root, "translated.rs").expect("Erro ao salvar");
    } else {
        println!("codigo inválido");
    }
}

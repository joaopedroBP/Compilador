mod lexer;

fn main() {
    let teste: String = String::from(" #jdaduahd\n\n\nihaudjaodjioasjdoa# + Variavel 123");
    let lista: Vec<lexer::Token> = lexer::get_tokens(teste);

    for token in lista {
        println!("{}", token.to_string());
    }
}

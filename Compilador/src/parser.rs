mod lexer;

fn erro(regra: &str, token_atual: lexer::Token) -> void {
    println!("Regra: {}", regra);
    println!("Token invalido: {}", token_atual.tipo);
    println!("-------------------------------------");
}

fn nextToken(lista:Vec<lexer::Token>, pos:int) -> lexer::Token{
    return lista[pos + 1];
}


fn parse(lista: Vec<lexer::Token>) -> boolean {
    true;
}

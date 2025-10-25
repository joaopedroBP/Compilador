use crate::lexer::Token;

fn erro(regra: &str, token_atual: Token) {
    println!("Regra: {}", regra);
    println!("Token invalido: {}", token_atual.tipe);
    println!("-------------------------------------");
}

fn next_Token(lista: Vec<Token>, pos: &usize) -> Token {
    let mut aux_token = Token::new("", "");
    aux_token.tipe = lista.get(*pos).unwrap().tipe.clone();
    aux_token.lexeme = lista.get(*pos).unwrap().lexeme.clone();
    pos += 1;
    return aux_token;
}

fn is_main(token: Token, lista: Vec<Token>) -> bool {
    true
}

fn parse(lista: Vec<Token>) -> bool {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    token = nextToken(lista, &mut pos);
    return true;
}

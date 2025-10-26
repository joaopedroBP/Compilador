use crate::lexer::Token;

fn erro(regra: &str, token_atual: Token) {
    println!("Regra: {}", regra);
    println!("Token invalido: {}", token_atual.tipe);
    println!("-------------------------------------");
}

fn next_token(lista: Vec<Token>, pos: usize) -> (Token, usize) {
    let mut aux_token = Token::new("", "");
    aux_token.tipe = lista.get(pos).unwrap().tipe.clone();
    aux_token.lexeme = lista.get(pos).unwrap().lexeme.clone();
    return (aux_token, pos + 1);
}

fn is_if(lista: Vec<Token>, token: Token, pos: usize) -> bool {
    fn COND(lista: Vec<Token>, token: Token, pos: usize) {
        if (EXP_OU()) {
            return true;
        }
    }

    fn is_elseif(lista: Vec<Token>, token: Token, pos: usize) -> bool {
        if (is_if()) {
            return true;
        } else if (token.lexeme == "{") {
            (token, pos) = next_token(lista, pos);
            if (Bloco()) {
                if (token.lexeme == "}") {
                    (token, pos) = next_token(lista, pos);
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn is_else(lista: Vec<Token>, token: Token, pos: usize) -> bool {
        if (token.lexeme == "else") {
            (token, pos) = next_token(lista, pos);
            if (is_elseif()) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    if (token.lexeme == "if") {
        (token, pos) = next_token(lista, pos);
        if (token.lexeme == "(") {
            (token, pos) = next_token(lista, pos);
            if (COND()) {
                if (token.lexeme == ")") {
                    (token, pos) = next_token(lista, pos);
                    if (token.lexeme == "{") {
                        (token, pos) = next_token(lista, pos);
                        if (Bloco()) {
                            if (token.lexeme == "}") {
                                (token, pos) = next_token(lista, pos);
                                if (is_else()) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse(lista: Vec<Token>, token: Token, pos: usize) -> bool {}

fn parser(lista: Vec<Token>) -> bool {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    (token, pos) = next_token(lista, pos);
    while (token.tipe != "EOF") {}
    return true;
}

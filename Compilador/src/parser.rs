use crate::lexer::Token;

fn erro(regra: &str, token_atual: Token) {
    println!("Regra: {}", regra);
    println!("Token invalido: {}", token_atual.tipe);
    println!("-------------------------------------");
}

fn next_token(lista: &Vec<Token>, pos: usize) -> (Token, usize) {
    let mut aux_token = Token::new("", "");
    aux_token.tipe = lista.get(pos).unwrap().tipe.clone();
    aux_token.lexeme = lista.get(pos).unwrap().lexeme.clone();
    return (aux_token, pos + 1);
}

fn is_if(mut lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
    fn Bloco(lista: Vec<Token>, token: Token, pos: usize) -> bool {
        return true;
    }
    fn is_valid_comparated(token: Token) -> bool {
        match token.lexeme.as_str() {
            "Floating_point" => true,
            "Integer" => true,
            "TRUE" => true,
            "FALSE" => true,
            "ID" => true,
            _ => false,
        }
    }

    fn OP_COMP(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if (token.lexeme == ">") {
            (token, pos) = next_token(&lista, pos);
            if (token.lexeme == "=") {
                (token, pos) = next_token(&lista, pos);
                return true;
            } else {
                return true;
            }
        } else if (token.lexeme == "<") {
            (token, pos) = next_token(&lista, pos);
            if (token.lexeme == "=") {
                (token, pos) = next_token(&lista, pos);
                return true;
            } else {
                return true;
            }
        } else if (token.lexeme == "!") {
            (token, pos) = next_token(&lista, pos);
            if (token.lexeme == "=") {
                (token, pos) = next_token(&lista, pos);
                return true;
            } else {
                return false;
            }
        } else if (token.lexeme == "=") {
            (token, pos) = next_token(&lista, pos);
            if (token.lexeme == "=") {
                (token, pos) = next_token(&lista, pos);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EXP_COMP(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if is_valid_comparated(token) {
            (token, pos) = next_token(&lista, pos);
            if OP_COMP(lista, token, pos) {
                if is_valid_comparated(token) {
                    (token, pos) = next_token(&lista, pos);
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

    fn EXP_EL(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if (token.lexeme == "&") {
            (token, pos) = next_token(&lista, pos);
            if (token.lexeme == "&") {
                (token, pos) = next_token(&lista, pos);
                if EXP_COMP(lista, token, pos) {
                    if EXP_EL(lista, token, pos) {
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

        return true;
    }

    fn EXP_E(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if EXP_COMP(lista, token, pos) {
            if EXP_EL(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EXP_OUL(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if token.lexeme == "|" {
            (token, pos) = next_token(&lista, pos);
            if token.lexeme == "|" {
                (token, pos) = next_token(&lista, pos);
                if EXP_E(lista, token, pos) {
                    if EXP_OU(lista, token, pos) {
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

        return true;
    }

    fn EXP_OU(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if EXP_E(lista, token, pos) {
            if EXP_OUL(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn COND(lista: Vec<Token>, token: Token, pos: usize) -> bool {
        if EXP_OU(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn is_elseif(lista: Vec<Token>, token: Token, pos: usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if (token.lexeme == "{") {
            (token, pos) = next_token(&lista, pos);
            if Bloco(lista, token, pos) {
                if (token.lexeme == "}") {
                    (token, pos) = next_token(&lista, pos);
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

    fn is_else(lista: Vec<Token>, mut token: Token, mut pos: usize) -> bool {
        if token.lexeme == "else" {
            (token, pos) = next_token(&lista, pos);
            if is_elseif(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    if token.lexeme == "if" {
        (token, pos) = next_token(&lista, pos);
        if token.lexeme == "(" {
            (token, pos) = next_token(&lista, pos);
            if COND(lista, token, pos) {
                if token.lexeme == ")" {
                    (token, pos) = next_token(&lista, pos);
                    if token.lexeme == "{" {
                        (token, pos) = next_token(&lista, pos);
                        if Bloco(lista, token, pos) {
                            if token.lexeme == "}" {
                                (token, pos) = next_token(&lista, pos);
                                if is_else(lista, token, pos) {
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
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        };
    } else {
        return false;
    }
}

fn parse(lista: Vec<Token>, token: Token, pos: usize) -> bool {
    if is_if(lista, token, pos) {
        return true;
    } else {
        return false;
    }
}

fn parser(lista: Vec<Token>) -> bool {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    (token, pos) = next_token(&lista, pos);
    while (token.tipe != "EOF") {}
    return true;
}

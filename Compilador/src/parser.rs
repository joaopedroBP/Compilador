use crate::lexer::Token;

fn erro(regra: &str, token_atual: &mut Token) {
    println!("Regra: {}", regra);
    println!("Token invalido: {}", token_atual.tipe);
    println!("-------------------------------------");
}

fn next_token(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) {
    token.tipe = lista.get(*pos).unwrap().tipe.clone();
    token.lexeme = lista.get(*pos).unwrap().lexeme.clone();
    *pos += 1;
}

fn is_if(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn Bloco(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        return true;
    }
    fn is_valid_comparated(token: &mut Token) -> bool {
        match token.lexeme.as_str() {
            "Floating_point" => true,
            "Integer" => true,
            "TRUE" => true,
            "FALSE" => true,
            "ID" => true,
            _ => false,
        }
    }

    fn OP_COMP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == ">" {
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                next_token(lista, pos, token);
                return true;
            } else {
                return true;
            }
        } else if token.lexeme == "<" {
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                next_token(lista, pos, token);
                return true;
            } else {
                return true;
            }
        } else if token.lexeme == "!" {
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else if token.lexeme == "=" {
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EXP_COMP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_valid_comparated(token) {
            next_token(lista, pos, token);
            if OP_COMP(&lista, token, pos) {
                if is_valid_comparated(token) {
                    next_token(lista, pos, token);
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

    fn EXP_EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if (token.lexeme == "&") {
            next_token(lista, pos, token);
            if (token.lexeme == "&") {
                next_token(lista, pos, token);
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

    fn EXP_E(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if EXP_COMP(lista, token, pos) {
            if EXP_EL(&lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EXP_OUL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "|" {
            next_token(lista, pos, token);
            if token.lexeme == "|" {
                next_token(lista, pos, token);
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

    fn EXP_OU(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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

    fn COND(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if EXP_OU(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn is_elseif(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if (token.lexeme == "{") {
            next_token(lista, pos, token);
            if Bloco(&lista, token, pos) {
                if (token.lexeme == "}") {
                    next_token(lista, pos, token);
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

    fn is_else(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "else" {
            next_token(lista, pos, token);
            if is_elseif(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    if token.lexeme == "if" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if COND(&lista, token, pos) {
                if token.lexeme == ")" {
                    next_token(lista, pos, token);
                    if token.lexeme == "{" {
                        next_token(lista, pos, token);
                        if Bloco(&lista, token, pos) {
                            if token.lexeme == "}" {
                                next_token(lista, pos, token);
                                if is_else(&lista, token, pos) {
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

fn parse(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if is_if(lista, token, pos) {
        return true;
    } else {
        erro("if", token);
        return false;
    }
}

pub fn parser(lista: Vec<Token>) -> bool {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    next_token(&lista, &mut pos, &mut token);
    while token.tipe != "EOF" {
        parse(&lista, &mut pos, &mut token);
    }
    return true;
}

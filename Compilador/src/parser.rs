use crate::lexer::Token;

fn erro(regra: &str, token_atual: &mut Token) {
    println!("Regra: {}", regra);
    println!(
        "Token invalido: {}, {}",
        token_atual.tipe, token_atual.lexeme
    );
    println!("-------------------------------------");
}

fn next_token(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) {
    token.tipe = lista.get(*pos).unwrap().tipe.clone();
    token.lexeme = lista.get(*pos).unwrap().lexeme.clone();
    *pos += 1;
}

fn is_declaration(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn DEC_TYPE(token: &mut Token) -> bool {
        match token.tipe.as_str() {
            "Reserved_FLOAT" => true,
            "Reserved_INT" => true,
            "Reserved_CHAR" => true,
            "Reserved_VOID" => true,
            "Reserved_BOOL" => true,
            _ => false,
        }
    }

    fn DEC_ATB(token: &mut Token) -> bool {
        match token.tipe.as_str() {
            "Floating_Point" => true,
            "Integer" => true,
            "character" => true,
            "ID" => true,
            "Reserved_TRUE" => true,
            "Reserved_FALSE" => true,
            _ => false,
        }
    }

    if DEC_TYPE(token) {
        next_token(lista, pos, token);
        if token.lexeme == ":" {
            next_token(lista, pos, token);
            if token.tipe == "ID" {
                next_token(lista, pos, token);
                if token.lexeme == "=" {
                    next_token(lista, pos, token);
                    if DEC_ATB(token) {
                        next_token(lista, pos, token);
                        if token.lexeme == ";" {
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

fn is_if(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn Bloco(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_declaration(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }
    fn is_valid_comparated(token: &mut Token) -> bool {
        match token.tipe.as_str() {
            "Floating_Point" => true,
            "Integer" => true,
            "Reserved_TRUE" => true,
            "Reserved_FALSE" => true,
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
                    if EXP_OUL(lista, token, pos) {
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
    } else if is_declaration(lista, token, pos) {
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
    let mut result: bool = false;
    while token.tipe != "EOF" {
        result = parse(&lista, &mut token, &mut pos);
        if !result {
            break;
        }
    }
    return result;
}

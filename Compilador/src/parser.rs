use crate::lexer::Token;

fn erro(regra: &str, token_atual: &mut Token) {
    println!("{}", regra);
    println!(
        "Invalid Token: {}, {} at line:{} columm:{}",
        token_atual.tipe, token_atual.lexeme, token_atual.linha, token_atual.coluna
    );
    println!("-------------------------------------");
}

fn next_token(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) {
    token.tipe = lista.get(*pos).unwrap().tipe.clone();
    token.lexeme = lista.get(*pos).unwrap().lexeme.clone();
    *pos += 1;
}

fn return_type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "Floating_Point" {
        next_token(lista, pos, token);
        return true;
    } else if token.tipe == "Integer" {
        next_token(lista, pos, token);
        return true;
    } else if token.tipe == "character" {
        next_token(lista, pos, token);
        return true;
    } else if token.tipe == "ID" {
        next_token(lista, pos, token);
        return true;
    } else if token.tipe == "Reserved_TRUE" {
        next_token(lista, pos, token);
        return true;
    } else if token.tipe == "Reserved_FALSE" {
        next_token(lista, pos, token);
        return true;
    } else {
        return false;
    }
}

fn Return(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "Reserved_return" {
        next_token(lista, pos, token);
        if return_type(lista, token, pos) {
            if token.lexeme == ";" {
                next_token(lista, pos, token);
                return true;
            } else {
                erro("missing ';' after return", token);
                return false;
            }
        } else {
            erro("invalid return value", token);
            return false;
        }
    }

    return false;
}

fn is_atribuicao(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn SIMP_OP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "+" {
            let mut aux_pos: usize = *pos + 1;
            if lista[aux_pos].lexeme == "+" {
                next_token(lista, pos, token);
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else if token.lexeme == "-" {
            let mut aux_pos: usize = *pos + 1;
            if lista[aux_pos].lexeme == "-" {
                next_token(lista, pos, token);
                next_token(lista, pos, token);
                return true;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    fn COMP_OPTION(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_operation(lista, token, pos) {
            return true;
        } else if token.tipe == "Floating_Point" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Integer" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "character" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_TRUE" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_FALSE" {
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn COMP_OP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "+" || token.lexeme == "-" || token.lexeme == "*" || token.lexeme == "/"
        {
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                next_token(lista, pos, token);
                if COMP_OPTION(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else if token.lexeme == "=" {
            next_token(lista, pos, token);
            if COMP_OPTION(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    fn OP_ATB(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if SIMP_OP(lista, token, pos) {
            return true;
        } else if COMP_OP(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    if token.tipe == "ID" {
        next_token(lista, pos, token);
        if OP_ATB(lista, token, pos) {
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
}

fn is_operation(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn F(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "ID" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Integer" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Floating_Point" {
            next_token(lista, pos, token);
            return true;
        } else if token.lexeme == "(" {
            next_token(lista, pos, token);
            if is_operation(lista, token, pos) {
                if token.lexeme == ")" {
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

    fn TL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "*" {
            next_token(lista, pos, token);
            if F(lista, token, pos) {
                if TL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else if token.lexeme == "/" {
            next_token(lista, pos, token);
            if F(lista, token, pos) {
                if TL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    fn T(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if F(lista, token, pos) {
            if TL(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "+" {
            next_token(lista, pos, token);
            if T(lista, token, pos) {
                if EL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else if token.lexeme == "-" {
            next_token(lista, pos, token);
            if T(lista, token, pos) {
                if EL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    if T(lista, token, pos) {
        if EL(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn VAR(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) -> bool {
    fn DEC_ATB(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) -> bool {
        if is_operation(lista, token, pos) {
            return true;
        } else if token.tipe == "Floating_Point" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Integer" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "character" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_TRUE" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_FALSE" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("Missing proper declaration atribution", token);
            return false;
        }
    }

    if token.tipe == "ID" {
        next_token(lista, pos, token);
        if token.lexeme == "=" {
            next_token(lista, pos, token);
            if DEC_ATB(lista, pos, token) {
                if token.lexeme == ";" {
                    next_token(lista, pos, token);
                    return true;
                } else {
                    erro("Declaration missing end of operation sign ';'", token);
                    return false;
                }
            } else {
                return false;
            }
        } else {
            erro("Declaration missing '=' sign", token);
            return false;
        }
    } else {
        return false;
    }
}

fn FUNC(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn PARAMETER_TYPE(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_operation(lista, token, pos) {
            return true;
        } else if token.tipe == "Floating_Point" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Integer" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "character" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_TRUE" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_FALSE" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("Missing proper declaration atribution", token);
            return false;
        }
    }

    fn PARAMS(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "," {
            next_token(lista, pos, token);
            if PARAMETER(lista, token, pos) {
                return true;
            } else {
                erro("function parameter incorect", token);
                return false;
            }
        }
        return true;
    }
    fn PARAMETER(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if PARAMETER_TYPE(lista, token, pos) {
            if token.lexeme == ":" {
                next_token(lista, pos, token);
                if token.tipe == "ID" {
                    next_token(lista, pos, token);
                    if PARAMS(lista, token, pos) {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if is_declaration(lista, token, pos) {
            return true;
        } else if is_atribuicao(lista, token, pos) {
            return true;
        } else if Return(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn func_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return func_block(lista, token, pos);
        } else {
            return false;
        }
    }

    if token.tipe == "Reserved_function" {
        next_token(lista, pos, token);
        if token.tipe == "ID" {
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                next_token(lista, pos, token);
                if PARAMETER(lista, token, pos) {
                    if token.lexeme == ")" {
                        next_token(lista, pos, token);
                        if token.lexeme == "{" {
                            next_token(lista, pos, token);
                            if func_block(lista, token, pos) {
                                if token.lexeme == "}" {
                                    next_token(lista, pos, token);
                                    return true;
                                } else {
                                    erro("function declaration missng closing '}'", token);
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            erro("function declaration missing opening '{'", token);
                            return false;
                        }
                    } else {
                        erro("function declaration missing closing ')'", token);
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                erro("function declaraton missing opening '('", token);
                return false;
            }
        } else {
            erro("Function declaratin missing name", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_declaration(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn DEC_TYPE(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "Reserved_FLOAT" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_INT" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_CHAR" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_BOOL" {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Reserved_VOID" {
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn DECLARATION(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) -> bool {
        if VAR(lista, pos, token) {
            return true;
        } else if FUNC(lista, token, pos) {
            return true;
        } else {
            erro("Missing proper Declaration", token);
            return false;
        }
    }

    if DEC_TYPE(lista, token, pos) {
        if token.lexeme == ":" {
            next_token(lista, pos, token);
            if DECLARATION(lista, pos, token) {
                return true;
            } else {
                return false;
            }
        } else {
            erro("Declaration Missing ':'", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_if(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if is_declaration(lista, token, pos) {
            return true;
        } else if is_atribuicao(lista, token, pos) {
            return true;
        } else if Return(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn if_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return if_block(lista, token, pos);
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
            if if_block(&lista, token, pos) {
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
                        if if_block(&lista, token, pos) {
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
        }
    } else {
        return false;
    }
}

fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if is_if(lista, token, pos) {
        return true;
    } else if is_declaration(lista, token, pos) {
        return true;
    } else if is_atribuicao(lista, token, pos) {
        return true;
    } else if Return(lista, token, pos) {
        return true;
    } else {
        return false;
    }
}

fn bloco(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "EOF" {
        return true;
    }

    if CMD(lista, token, pos) {
        return bloco(lista, token, pos);
    }

    return false;
}

pub fn parser(lista: Vec<Token>) -> bool {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    next_token(&lista, &mut pos, &mut token);
    let result: bool = bloco(&lista, &mut token, &mut pos);

    if result {
        return true;
    } else {
        return false;
    }
}

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
    token.linha = lista.get(*pos).unwrap().linha;
    token.coluna = lista.get(*pos).unwrap().coluna;
    *pos += 1;
}

fn Continue(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.lexeme == "continue" {
        next_token(lista, pos, token);
        return true;
    } else {
        return false;
    }
}

fn Break(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.lexeme == "break" {
        next_token(lista, pos, token);
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
    if token.lexeme == ">" || token.lexeme == "<" {
        next_token(lista, pos, token);
        if token.lexeme == "=" {
            return true;
        } else {
            return true;
        }
    } else if token.lexeme == "!" || token.lexeme == "=" {
        next_token(lista, pos, token);
        if token.lexeme == "=" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("Invalid conparator used", token);
            return false;
        }
    } else {
        erro("Invalid conparator used", token);
        return false;
    }
}

fn COMPARATION(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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

fn return_type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "Floating_Point"
        || token.tipe == "Integer"
        || token.tipe == "character"
        || token.tipe == "ID"
        || token.tipe == "Reserved_TRUE"
        || token.tipe == "Reserved_FALSE"
    {
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

fn VAR_ATB(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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
        } else if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
        {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                if FUNC_CALL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return true;
            }
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

    if OP_ATB(lista, token, pos) {
        return true;
    } else {
        return false;
    }
}

fn FUNC_CALL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn argument_type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "ID"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
        {
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn args(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "," {
            next_token(lista, pos, token);
            if arguments(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }
    fn arguments(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if argument_type(lista, token, pos) {
            if args(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    if token.lexeme == "(" {
        next_token(lista, pos, token);
        if arguments(lista, token, pos) {
            if token.lexeme == ")" {
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return false;
}

fn is_atribuicao_interna(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn ATB_KIND(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if VAR_ATB(lista, token, pos) {
            return true;
        } else if FUNC_CALL(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    if token.tipe == "ID" {
        next_token(lista, pos, token);
        if ATB_KIND(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn is_atribuicao(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if is_atribuicao_interna(lista, token, pos) {
        if token.lexeme == ";" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("attribution missing end of operation ';'", token);
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

fn Main(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if is_declaration(lista, token, pos) {
            return true;
        } else if is_atribuicao(lista, token, pos) {
            return true;
        } else if Return(lista, token, pos) {
            return true;
        } else if println(lista, token, pos) {
            return true;
        } else if scanln(lista, token, pos) {
            return true;
        } else if is_while(lista, token, pos) {
            return true;
        } else if Continue(lista, token, pos) {
            return true;
        } else if Break(lista, token, pos) {
            return true;
        } else if is_for(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn main_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "}" {
            return true;
        }

        if CMD(lista, token, pos) {
            return main_block(lista, token, pos);
        }

        return false;
    }

    if token.lexeme == "main" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if token.lexeme == ")" {
                next_token(lista, pos, token);
                if token.lexeme == "{" {
                    next_token(lista, pos, token);
                    if main_block(lista, token, pos) {
                        if token.lexeme == "}" {
                            next_token(lista, pos, token);
                            return true;
                        } else {
                            erro("main function body missing closing '}'", token);
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    erro("main function body missing opening '{", token);
                    return false;
                }
            } else {
                erro("main function missing closing ')", token);
                return false;
            }
        } else {
            erro("main functiom missing opening '('", token);
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
        } else if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
        {
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                if FUNC_CALL(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return true;
            }
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
        } else {
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
        } else if println(lista, token, pos) {
            return true;
        } else if scanln(lista, token, pos) {
            return true;
        } else if is_while(lista, token, pos) {
            return true;
        } else if Continue(lista, token, pos) {
            return true;
        } else if Break(lista, token, pos) {
            return true;
        } else if is_for(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn func_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return func_block(lista, token, pos);
        } else {
            return false;
        }
    }

    if token.lexeme == "function" {
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
        if Main(lista, token, pos) {
            return true;
        } else if VAR(lista, pos, token) {
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
        } else if println(lista, token, pos) {
            return true;
        } else if scanln(lista, token, pos) {
            return true;
        } else if is_while(lista, token, pos) {
            return true;
        } else if Continue(lista, token, pos) {
            return true;
        } else if Break(lista, token, pos) {
            return true;
        } else if is_for(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn if_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return if_block(lista, token, pos);
        } else {
            return false;
        }
    }

    fn EXP_EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if (token.lexeme == "&") {
            next_token(lista, pos, token);
            if (token.lexeme == "&") {
                next_token(lista, pos, token);
                if COMPARATION(lista, token, pos) {
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
        if COMPARATION(lista, token, pos) {
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
                        if if_block(lista, token, pos) {
                            if token.lexeme == "}" {
                                next_token(lista, pos, token);
                                if is_else(&lista, token, pos) {
                                    return true;
                                } else {
                                    return false;
                                }
                            } else {
                                erro("missing cloasing brackets", token);
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

fn scanln(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "Reserved_scanln" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if token.tipe == "ID" {
                next_token(lista, pos, token);
                if token.lexeme == ")" {
                    next_token(lista, pos, token);
                    if token.lexeme == ";" {
                        next_token(lista, pos, token);
                        return true;
                    } else {
                        erro("scanln missing end of operation ';'", token);
                        return false;
                    }
                } else {
                    erro("scanln missing closing ')'", token);
                    return false;
                }
            } else {
                erro("scanln missing ID to scan", token);
                return false;
            }
        } else {
            erro("scanln missing opening '('", token);
            return false;
        }
    } else {
        return false;
    }
}

fn println(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn vars(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "," {
            next_token(lista, pos, token);
            if token.tipe == "ID" {
                next_token(lista, pos, token);
                if vars(lista, token, pos) {
                    return true;
                }
            } else {
                erro("println variable missing name", token);
                return false;
            }
        }
        return true;
    }

    if token.tipe == "Reserved_println" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if token.tipe == "string" {
                next_token(lista, pos, token);
                if vars(lista, token, pos) {
                    if token.lexeme == ")" {
                        next_token(lista, pos, token);
                        if token.lexeme == ";" {
                            next_token(lista, pos, token);
                            return true;
                        } else {
                            erro("println missing end of operation ';'", token);
                            return false;
                        }
                    } else {
                        erro("println missing closing ')'", token);
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                erro("println missing content", token);
                return false;
            }
        } else {
            erro("println missing opening '('", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_while(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if is_declaration(lista, token, pos) {
            return true;
        } else if is_atribuicao(lista, token, pos) {
            return true;
        } else if Return(lista, token, pos) {
            return true;
        } else if println(lista, token, pos) {
            return true;
        } else if scanln(lista, token, pos) {
            return true;
        } else if is_while(lista, token, pos) {
            return true;
        } else if Continue(lista, token, pos) {
            return true;
        } else if Break(lista, token, pos) {
            return true;
        } else if is_for(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn while_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return while_block(lista, token, pos);
        } else {
            return false;
        }
    }

    fn E_PARL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "&" {
            next_token(lista, pos, token);
            if token.lexeme == "&" {
                next_token(lista, pos, token);
                if COMPARATION(lista, token, pos) {
                    if E_PARL(lista, token, pos) {
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
            return true;
        }
    }

    fn E_PAR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if COMPARATION(lista, token, pos) {
            if E_PARL(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn OU_PARL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "|" {
            next_token(lista, pos, token);
            if token.lexeme == "|" {
                next_token(lista, pos, token);
                if E_PAR(lista, token, pos) {
                    if OU_PARL(lista, token, pos) {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                erro("'OR' parameter missing second '|'", token);
                return false;
            }
        } else {
            return true;
        }
    }

    fn OU_PAR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if E_PAR(lista, token, pos) {
            if OU_PARL(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn WPARAMETERS(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "Reserved_TRUE" {
            next_token(lista, pos, token);
            return true;
        } else if OU_PAR(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    if token.tipe == "Reserved_while" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if WPARAMETERS(lista, token, pos) {
                if token.lexeme == ")" {
                    next_token(lista, pos, token);
                    if token.lexeme == "{" {
                        next_token(lista, pos, token);
                        if while_block(lista, token, pos) {
                            if token.lexeme == "}" {
                                next_token(lista, pos, token);
                                return true;
                            } else {
                                erro("while missing closing '}'", token);
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        erro("while missing opening '{'", token);
                        return false;
                    }
                } else {
                    erro("while missing closing ')'", token);
                    return false;
                }
            } else {
                return false;
            }
        } else {
            erro("while missing opening '('", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_for(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_if(lista, token, pos) {
            return true;
        } else if is_declaration(lista, token, pos) {
            return true;
        } else if is_atribuicao(lista, token, pos) {
            return true;
        } else if Return(lista, token, pos) {
            return true;
        } else if println(lista, token, pos) {
            return true;
        } else if scanln(lista, token, pos) {
            return true;
        } else if is_while(lista, token, pos) {
            return true;
        } else if Continue(lista, token, pos) {
            return true;
        } else if Break(lista, token, pos) {
            return true;
        } else if is_for(lista, token, pos) {
            return true;
        } else {
            return false;
        }
    }

    fn for_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos) {
            return for_block(lista, token, pos);
        } else {
            return false;
        }
    }

    fn COMPARATOR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if is_declaration(lista, token, pos) {
            return true;
        } else if token.tipe == "ID" {
            next_token(lista, pos, token);
            if token.lexeme == ";" {
                next_token(lista, pos, token);
                return true;
            } else {
                erro("for loop ID comparator missing end of operation ';'", token);
                return false;
            }
        } else {
            return false;
        }
    }

    if token.tipe == "Reserved_for" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            next_token(lista, pos, token);
            if COMPARATOR(lista, token, pos) {
                if COMPARATION(lista, token, pos) {
                    if token.lexeme == ";" {
                        next_token(lista, pos, token);
                        if is_atribuicao_interna(lista, token, pos) {
                            if token.lexeme == ")" {
                                next_token(lista, pos, token);
                                if token.lexeme == "{" {
                                    next_token(lista, pos, token);
                                    if for_block(lista, token, pos) {
                                        if token.lexeme == "}" {
                                            next_token(lista, pos, token);
                                            return true;
                                        } else {
                                            erro("For loop body missing closing '}'", token);
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }
                                } else {
                                    erro("For loop body missing opening '}'", token);
                                    return false;
                                }
                            } else {
                                erro("For loop missing closing ')'", token);
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        erro("For loop missing ';' on condition", token);
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            erro("For loop missing openign '('", token);
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
    } else if println(lista, token, pos) {
        return true;
    } else if scanln(lista, token, pos) {
        return true;
    } else if is_while(lista, token, pos) {
        return true;
    } else if Continue(lista, token, pos) {
        return true;
    } else if Break(lista, token, pos) {
        return true;
    } else if is_for(lista, token, pos) {
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

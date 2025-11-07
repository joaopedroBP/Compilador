use crate::lexer::Token;

fn erro(regra: &str, token_atual: &mut Token) {
    println!("==================== SINTAX ERROR ======================");
    println!("Rule Violated : {}", regra);
    println!(
        "Invalid Token: < {} , {} >",
        token_atual.tipe, token_atual.lexeme
    );
    println!(
        "Location: LINE {}, COLUMM {}",
        token_atual.linha, token_atual.coluna
    );
    println!("=========================================================");
}

fn next_token(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) {
    token.tipe = lista.get(*pos).unwrap().tipe.clone();
    token.lexeme = lista.get(*pos).unwrap().lexeme.clone();
    token.linha = lista.get(*pos).unwrap().linha;
    token.coluna = lista.get(*pos).unwrap().coluna;
    *pos += 1;
}

fn Type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "Reserved_FLOAT"
        || token.tipe == "Reserved_INT"
        || token.tipe == "Reserved_CHAR"
        || token.tipe == "Reserved_BOOL"
        || token.tipe == "Reserved_VOID"
    {
        return true;
    } else {
        return false;
    }
}

fn Continue(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.lexeme == "continue" {
        next_token(lista, pos, token);
        if token.lexeme == ";" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("expected ';' after 'continue'", token);
            return false;
        }
    } else {
        return false;
    }
}

fn Break(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.lexeme == "break" {
        next_token(lista, pos, token);
        if token.lexeme == ";" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("expected ';' after 'break'", token);
            return false;
        }
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
            erro(
                "expected '=' after '!' or '=' in comparison operator",
                token,
            );
            return false;
        }
    } else {
        erro("invalid comparison operator", token);
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
                erro("expected value after comparison operator", token);
                return false;
            }
        } else {
            erro("missing or invalid comparison operator", token);
            return false;
        }
    } else {
        erro("expected value before comparison operator", token);
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
        erro("invalid return expression", token);
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
        if token.tipe == "Reserved_call" {
            if func_call_interna(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
            || token.tipe == "ID"
        {
            let aux_pos: usize = *pos;

            if lista[aux_pos].lexeme == "+"
                || lista[aux_pos].lexeme == "-"
                || lista[aux_pos].lexeme == "*"
                || lista[aux_pos].lexeme == "/"
            {
                if is_operation(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            }
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

    if OP_ATB(lista, token, pos) {
        return true;
    } else {
        return false;
    }
}

fn func_call_interna(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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

    if token.lexeme == "call" {
        next_token(lista, pos, token);
        if token.tipe == "ID" {
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                next_token(lista, pos, token);
                if arguments(lista, token, pos) {
                    if token.lexeme == ")" {
                        next_token(lista, pos, token);
                        return true;
                    } else {
                        erro("missing ')' after function arguments", token);
                        return false;
                    }
                } else {
                    erro("invalid arguments in function call", token);
                    return false;
                }
            } else {
                erro("missing '(' after function name", token);
                return false;
            }
        } else {
            erro("expected function identifier", token);
            return false;
        }
    } else {
        return false;
    }
}

fn func_call(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if func_call_interna(lista, token, pos) {
        if token.lexeme == ";" {
            next_token(lista, pos, token);
            return true;
        } else {
            erro("missing ':' after function call", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_atribuicao_interna(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    if token.tipe == "ID" {
        next_token(lista, pos, token);
        if VAR_ATB(lista, token, pos) {
            return true;
        } else {
            erro("invalid variable assignment", token);
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
                    erro("missing closing ')' in expression", token);
                    return false;
                }
            } else {
                erro("invalid expression inside parentheses", token);
                return false;
            }
        } else {
            erro("expected identifier,number or '(' in expression", token);
            return false;
        }
    }

    fn TL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "*" || token.lexeme == "/" {
            next_token(lista, pos, token);
            if F(lista, token, pos) {
                if TL(lista, token, pos) {
                    return true;
                } else {
                    erro("invalid expression", token);
                    return false;
                }
            } else {
                erro("expected operand", token);
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
                erro("invalid term continuation in expression", token);
                return false;
            }
        } else {
            erro("Invalid term in expression", token);
            return false;
        }
    }

    fn EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.lexeme == "+" || token.lexeme == "-" {
            next_token(lista, pos, token);
            if T(lista, token, pos) {
                if EL(lista, token, pos) {
                    return true;
                } else {
                    erro("invalid expression", token);
                    return false;
                }
            } else {
                erro("invalid operand", token);
                return false;
            }
        }
        return true;
    }

    if T(lista, token, pos) {
        if EL(lista, token, pos) {
            return true;
        } else {
            erro("invalid expression continuation", token);
            return false;
        }
    } else {
        return false;
    }
}

fn Main(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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
        if token.tipe == "Reserved_call" {
            if func_call_interna(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
            || token.tipe == "ID"
        {
            let aux_pos: usize = *pos;

            if lista[aux_pos].lexeme == "+"
                || lista[aux_pos].lexeme == "-"
                || lista[aux_pos].lexeme == "*"
                || lista[aux_pos].lexeme == "/"
            {
                if is_operation(lista, token, pos) {
                    return true;
                } else {
                    return false;
                }
            }
            next_token(lista, pos, token);
            return true;
        } else {
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
                    erro("Declaration missing ';' at the end", token);
                    return false;
                }
            } else {
                erro("variable being assigned invalid value", token);
                return false;
            }
        } else {
            erro("Declaration missing '='", token);
            return false;
        }
    } else {
        erro("declaration missing variable name", token);
        return false;
    }
}

fn FUNC(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
    fn PARAMETER_TYPE(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
        if token.tipe == "Reserved_FLOAT"
            || token.tipe == "Reserved_INT"
            || token.tipe == "Reserved CHAR"
            || token.tipe == "Reserved_BOOL"
        {
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
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
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
        if token.lexeme == "main" {
            if Main(lista, token, pos) {
                return true;
            } else {
                return false;
            }
        } else if token.tipe == "ID" {
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
        if token.tipe == "Reserved_FLOAT"
            || token.tipe == "Reserved_INT"
            || token.tipe == "Reserved_CHAR"
            || token.tipe == "Reserved_BOOL"
            || token.tipe == "Reserved_VOID"
        {
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn DECLARATION(lista: &Vec<Token>, pos: &mut usize, token: &mut Token) -> bool {
        if token.lexeme == "function" {
            if FUNC(lista, token, pos) {
                return true;
            }
            return false;
        } else if VAR(lista, pos, token) {
            return true;
        } else {
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
            erro("declaration missing ':' operator", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_if(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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
                                    erro("invalid else or elseif structure", token);
                                    return false;
                                }
                            } else {
                                erro("missing closing '}' after if block", token);
                                return false;
                            }
                        } else {
                            erro("invalid command insided if block", token);
                            return false;
                        }
                    } else {
                        erro("missing opening '{' after condition", token);
                        return false;
                    }
                } else {
                    erro("missing closing ')' after condition", token);
                    return false;
                }
            } else {
                erro("invalid or missing condition in if statment", token);
                return false;
            }
        } else {
            erro("missing opening '(' after 'if'", token);
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
                        erro("expecterd ';' after scnaln statement", token);
                        return false;
                    }
                } else {
                    erro("missing closing ')' in scanln", token);
                    return false;
                }
            } else {
                erro("expected identifier inside scanln()", token);
                return false;
            }
        } else {
            erro("missing opening '(' after scanln", token);
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
                            erro("expected ';' after println statement", token);
                            return false;
                        }
                    } else {
                        erro("missing closing ')' in println", token);
                        return false;
                    }
                } else {
                    erro("invalid variable list in println", token);
                    return false;
                }
            } else {
                erro("missing string or content inside println", token);
                return false;
            }
        } else {
            erro("missing openign '(' after println", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_while(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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
                                erro("missing closing '}' in while block", token);
                                return false;
                            }
                        } else {
                            erro("invalid command inside while block", token);
                            return false;
                        }
                    } else {
                        erro("missing opening '{' for while block", token);
                        return false;
                    }
                } else {
                    erro("missing closing ')' in while condition", token);
                    return false;
                }
            } else {
                erro("invalid condition in while", token);
                return false;
            }
        } else {
            erro("missing opening '(' after while", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_for(lista: &Vec<Token>, token: &mut Token, pos: &mut usize) -> bool {
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
                                            erro("for loop body missing closing '}'", token);
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }
                                } else {
                                    erro("For loop body missing opening '{'", token);
                                    return false;
                                }
                            } else {
                                erro("For loop missing closing ')'", token);
                                return false;
                            }
                        } else {
                            erro("for loop missing increment statement", token);
                            return false;
                        }
                    } else {
                        erro("For loop missing ';' after condition", token);
                        return false;
                    }
                } else {
                    erro("for loop missing condition comparison", token);
                    return false;
                }
            } else {
                erro(
                    "for loop missing initialization (declaration or ID comparator",
                    token,
                );
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
    } else if Type(lista, token, pos) {
        if is_declaration(lista, token, pos) {
            return true;
        }
        return false;
    } else if token.tipe == "ID" {
        if is_atribuicao(lista, token, pos) {
            return true;
        }
        return false;
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
    } else if token.lexeme == "call" {
        if func_call(lista, token, pos) {
            return true;
        }
        return false;
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

use crate::arvore::Node;
use crate::arvore::NodeRef;
use crate::arvore::Tree;
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

fn Type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
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

fn Continue(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let continue_call_node = Node::new("Continue_call");
    let continue_node = Node::new("continue");
    if token.lexeme == "continue" {
        Node::add_node(pai, &continue_call_node);
        Node::add_node(&continue_call_node, &continue_node);
        next_token(lista, pos, token);
        if token.lexeme == ";" {
            let end_node = Node::new(";");
            Node::add_node(&continue_call_node, &end_node);
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

fn Break(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let break_call_node = Node::new("break_call");
    let break_node = Node::new("break");
    if token.lexeme == "break" {
        Node::add_node(pai, &break_call_node);
        Node::add_node(&break_call_node, &break_node);
        next_token(lista, pos, token);
        if token.lexeme == ";" {
            let end_node = Node::new(";");
            Node::add_node(&break_call_node, &end_node);
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

fn is_valid_comparated(
    lista: &Vec<Token>,
    token: &mut Token,
    pos: &mut usize,
    pai: &NodeRef,
) -> bool {
    if token.tipe == "Reserved_call" {
        if func_call_interna(lista, token, pos, pai) {
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
            if is_operation(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        }
        let direcct_com_node = Node::new("direct_comparation");
        let node_type = Node::new(&token.tipe);
        let node_name = Node::new(&token.lexeme);
        Node::add_node(pai, &direcct_com_node);
        Node::add_node(&direcct_com_node, &node_type);
        Node::add_node(&node_type, &node_name);
        next_token(lista, pos, token);
        return true;
    } else {
        return false;
    }
}

fn OP_COMP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let et_node = Node::new("=");
    let comp_op_node = Node::new("comparation_operator");
    Node::add_node(pai, &comp_op_node);
    if token.lexeme == ">" || token.lexeme == "<" {
        let fc_node = Node::new(&token.lexeme);
        Node::add_node(&comp_op_node, &fc_node);
        next_token(lista, pos, token);
        if token.lexeme == "=" {
            next_token(lista, pos, token);
            Node::add_node(&comp_op_node, &et_node);
            return true;
        } else {
            return true;
        }
    } else if token.lexeme == "!" || token.lexeme == "=" {
        let fc_node = Node::new(&token.lexeme);
        Node::add_node(&comp_op_node, &fc_node);
        next_token(lista, pos, token);
        if token.lexeme == "=" {
            Node::add_node(&comp_op_node, &et_node);
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

fn COMPARATION(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let comparation_node = Node::new("comparation");
    Node::add_node(pai, &comparation_node);
    if is_valid_comparated(lista, token, pos, &comparation_node) {
        if OP_COMP(&lista, token, pos, &comparation_node) {
            if is_valid_comparated(lista, token, pos, &comparation_node) {
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

fn return_type(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    if token.tipe == "Floating_Point"
        || token.tipe == "Integer"
        || token.tipe == "character"
        || token.tipe == "ID"
        || token.tipe == "Reserved_TRUE"
        || token.tipe == "Reserved_FALSE"
    {
        let type_node = Node::new(&token.tipe);
        let type_content = Node::new(&token.lexeme);
        Node::add_node(pai, &type_node);
        Node::add_node(&type_node, &type_content);
        next_token(lista, pos, token);
        return true;
    } else {
        erro("invalid return expression", token);
        return false;
    }
}

fn Return(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let return_call_node = Node::new("return_call");
    let return_node = Node::new("return");
    if token.tipe == "Reserved_return" {
        Node::add_node(pai, &return_call_node);
        Node::add_node(&return_call_node, &return_node);
        next_token(lista, pos, token);
        if return_type(lista, token, pos, &return_call_node) {
            if token.lexeme == ";" {
                let end_node = Node::new(";");
                Node::add_node(&return_call_node, &end_node);
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

fn VAR_ATB(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn SIMP_OP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        let simp_op_node = Node::new("simple_atribution");
        if token.lexeme == "+" {
            let mut aux_pos: usize = *pos;
            if lista[aux_pos].lexeme == "+" {
                let plus_node = Node::new("+");
                let increment_node = Node::new("increment");
                Node::add_node(pai, &simp_op_node);
                Node::add_node(&simp_op_node, &increment_node);
                Node::add_node(&increment_node, &plus_node);
                Node::add_node(&increment_node, &plus_node);
                next_token(lista, pos, token);
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else if token.lexeme == "-" {
            let mut aux_pos: usize = *pos;
            if lista[aux_pos].lexeme == "-" {
                let minus_node = Node::new("-");
                let reduction_node = Node::new("reduction");
                Node::add_node(pai, &simp_op_node);
                Node::add_node(&simp_op_node, &reduction_node);
                Node::add_node(&reduction_node, &minus_node);
                Node::add_node(&reduction_node, &minus_node);
                next_token(lista, pos, token);
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn COMP_OPTION(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.tipe == "Reserved_call" {
            if func_call_interna(lista, token, pos, pai) {
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
                if is_operation(lista, token, pos, pai) {
                    return true;
                } else {
                    return false;
                }
            }
            let direcct_atb_node = Node::new("direct_attribution");
            let node_type = Node::new(&token.tipe);
            let node_name = Node::new(&token.lexeme);
            Node::add_node(pai, &direcct_atb_node);
            Node::add_node(&direcct_atb_node, &node_type);
            Node::add_node(&node_type, &node_name);
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn COMP_OP(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "+" || token.lexeme == "-" || token.lexeme == "*" || token.lexeme == "/"
        {
            let sign_node = Node::new(&token.lexeme);
            next_token(lista, pos, token);
            if token.lexeme == "=" {
                let equal_node = Node::new("=");
                Node::add_node(pai, &sign_node);
                Node::add_node(pai, &equal_node);
                next_token(lista, pos, token);
                if COMP_OPTION(lista, token, pos, pai) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else if token.lexeme == "=" {
            let equal_node = Node::new("=");
            Node::add_node(pai, &equal_node);
            next_token(lista, pos, token);
            if COMP_OPTION(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    fn OP_ATB(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if SIMP_OP(lista, token, pos, pai) {
            return true;
        } else if COMP_OP(lista, token, pos, pai) {
            return true;
        } else {
            return false;
        }
    }

    if OP_ATB(lista, token, pos, pai) {
        return true;
    } else {
        return false;
    }
}

fn func_call_interna(
    lista: &Vec<Token>,
    token: &mut Token,
    pos: &mut usize,
    pai: &NodeRef,
) -> bool {
    fn argument_type(
        lista: &Vec<Token>,
        token: &mut Token,
        pos: &mut usize,
        pai: &NodeRef,
    ) -> bool {
        if token.tipe == "Floating_Point"
            || token.tipe == "Integer"
            || token.tipe == "character"
            || token.tipe == "ID"
            || token.tipe == "Reserved_TRUE"
            || token.tipe == "Reserved_FALSE"
        {
            let type_node = Node::new(&token.tipe);
            let type_name_node = Node::new(&token.lexeme);
            Node::add_node(pai, &type_node);
            Node::add_node(&type_node, &type_name_node);
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn args(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "," {
            next_token(lista, pos, token);
            if token.lexeme != ")" {
                if arguments(lista, token, pos, pai) {
                    return true;
                } else {
                    return false;
                }
            } else {
                erro("functon missing an argument after ','", token);
                return false;
            }
        }
        return true;
    }
    fn arguments(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme != ")" {
            if argument_type(lista, token, pos, pai) {
                if args(lista, token, pos, pai) {
                    return true;
                } else {
                    return false;
                }
            } else {
                erro("function argument missing or invalid", token);
                return false;
            }
        }
        return true;
    }

    let i_func_c_node = Node::new("internal_function_call");
    Node::add_node(pai, &i_func_c_node);
    if token.lexeme == "call" {
        let call_node = Node::new("call");
        Node::add_node(&i_func_c_node, &call_node);
        next_token(lista, pos, token);
        if token.tipe == "ID" {
            let id_node = Node::new("ID");
            let id_name_node = Node::new(&token.lexeme);
            Node::add_node(&i_func_c_node, &id_node);
            Node::add_node(&id_node, &id_name_node);
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                let op_node = Node::new("(");
                let func_arguments_node = Node::new("function_call_arguments");
                Node::add_node(&i_func_c_node, &op_node);
                Node::add_node(&i_func_c_node, &func_arguments_node);
                next_token(lista, pos, token);
                if arguments(lista, token, pos, &func_arguments_node) {
                    if token.lexeme == ")" {
                        let cp_node = Node::new(")");
                        Node::add_node(&i_func_c_node, &cp_node);
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

fn func_call(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let func_c_node = Node::new("function_call");
    Node::add_node(pai, &func_c_node);
    if func_call_interna(lista, token, pos, &func_c_node) {
        if token.lexeme == ";" {
            let end_node = Node::new(";");
            Node::add_node(&func_c_node, &end_node);
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

fn is_atribuicao_interna(
    lista: &Vec<Token>,
    token: &mut Token,
    pos: &mut usize,
    pai: &NodeRef,
) -> bool {
    let atb_int_node = Node::new("internal_attribution_call");
    Node::add_node(pai, &atb_int_node);
    if token.tipe == "ID" {
        let id_node = Node::new("ID");
        let id_name_node = Node::new(&token.lexeme);
        Node::add_node(&atb_int_node, &id_node);
        Node::add_node(&id_node, &id_name_node);
        next_token(lista, pos, token);
        if VAR_ATB(lista, token, pos, &atb_int_node) {
            return true;
        } else {
            erro("invalid variable assignment", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_atribuicao(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let atb_node = Node::new("attribution_call");
    Node::add_node(pai, &atb_node);
    if is_atribuicao_interna(lista, token, pos, &atb_node) {
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

fn is_operation(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn F(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.tipe == "ID" {
            let id_node = Node::new("ID");
            let id_name_node = Node::new(&token.lexeme);
            Node::add_node(pai, &id_node);
            Node::add_node(&id_node, &id_name_node);
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Integer" {
            let integer_node = Node::new(&token.lexeme);
            Node::add_node(pai, &integer_node);
            next_token(lista, pos, token);
            return true;
        } else if token.tipe == "Floating_Point" {
            let float_node = Node::new(&token.lexeme);
            Node::add_node(pai, &float_node);
            next_token(lista, pos, token);
            return true;
        } else if token.lexeme == "(" {
            let op_node = Node::new("(");
            Node::add_node(pai, &op_node);
            next_token(lista, pos, token);
            if is_operation(lista, token, pos, pai) {
                if token.lexeme == ")" {
                    let cp_node = Node::new(")");
                    Node::add_node(pai, &cp_node);
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

    fn TL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "*" || token.lexeme == "/" {
            let operand_node = Node::new(&token.lexeme);
            Node::add_node(pai, &operand_node);
            next_token(lista, pos, token);
            if F(lista, token, pos, pai) {
                if TL(lista, token, pos, pai) {
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

    fn T(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if F(lista, token, pos, pai) {
            if TL(lista, token, pos, pai) {
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

    fn EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "+" || token.lexeme == "-" {
            let operand_node = Node::new(&token.lexeme);
            Node::add_node(pai, &operand_node);
            next_token(lista, pos, token);
            if T(lista, token, pos, pai) {
                if EL(lista, token, pos, pai) {
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
    let op_node = Node::new("operation");
    Node::add_node(pai, &op_node);

    if T(lista, token, pos, &op_node) {
        if EL(lista, token, pos, &op_node) {
            return true;
        } else {
            erro("invalid expression continuation", token);
            return false;
        }
    } else {
        return false;
    }
}

fn Main(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn main_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "}" {
            return true;
        }

        if CMD(lista, token, pos, pai) {
            return main_block(lista, token, pos, pai);
        }

        return false;
    }

    if token.lexeme == "main" {
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            Node::add_node(pai, &op_node);
            next_token(lista, pos, token);
            if token.lexeme == ")" {
                let cp_node = Node::new(")");
                Node::add_node(pai, &cp_node);
                next_token(lista, pos, token);
                if token.lexeme == "{" {
                    let ocb_node = Node::new("{");
                    Node::add_node(pai, &ocb_node);
                    next_token(lista, pos, token);
                    let main_block_node = Node::new("main_block");
                    Node::add_node(pai, &main_block_node);
                    if main_block(lista, token, pos, &main_block_node) {
                        if token.lexeme == "}" {
                            let fcb_node = Node::new("}");
                            Node::add_node(pai, &fcb_node);
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

fn VAR(lista: &Vec<Token>, pos: &mut usize, token: &mut Token, pai: &NodeRef) -> bool {
    fn DEC_ATB(lista: &Vec<Token>, pos: &mut usize, token: &mut Token, pai: &NodeRef) -> bool {
        if token.tipe == "Reserved_call" {
            if func_call_interna(lista, token, pos, pai) {
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
                if is_operation(lista, token, pos, pai) {
                    return true;
                } else {
                    return false;
                }
            }
            let atb_type = Node::new(&token.tipe);
            let atb_name = Node::new(&token.lexeme);
            Node::add_node(pai, &atb_type);
            Node::add_node(&atb_type, &atb_name);
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }
    let var_dec_node = Node::new("variable_declaration");
    Node::add_node(pai, &var_dec_node);

    if token.tipe == "ID" {
        let id_node = Node::new("ID");
        let var_name_node = Node::new(&token.lexeme);
        Node::add_node(&var_dec_node, &id_node);
        Node::add_node(&id_node, &var_name_node);

        next_token(lista, pos, token);
        if token.lexeme == "=" {
            let eq_node = Node::new("=");
            Node::add_node(&var_dec_node, &eq_node);
            next_token(lista, pos, token);
            if DEC_ATB(lista, pos, token, &var_dec_node) {
                if token.lexeme == ";" {
                    let end_node = Node::new(";");
                    Node::add_node(&var_dec_node, &end_node);
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
        erro("declaration variable has invalid or missing name", token);
        return false;
    }
}

fn FUNC(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn PARAMETER_TYPE(
        lista: &Vec<Token>,
        token: &mut Token,
        pos: &mut usize,
        pai: &NodeRef,
    ) -> bool {
        if token.tipe == "Reserved_FLOAT"
            || token.tipe == "Reserved_INT"
            || token.tipe == "Reserved CHAR"
            || token.tipe == "Reserved_BOOL"
        {
            let par_type = Node::new(&token.lexeme);
            Node::add_node(pai, &par_type);
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn PARAMS(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "," {
            let coma_node = Node::new(",");
            Node::add_node(pai, &coma_node);
            next_token(lista, pos, token);
            if PARAMETER(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }
    fn PARAMETER(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        let parameter_node = Node::new("parameter");
        Node::add_node(pai, &parameter_node);
        if PARAMETER_TYPE(lista, token, pos, &parameter_node) {
            if token.lexeme == ":" {
                let dp_node = Node::new(":");
                Node::add_node(&parameter_node, &dp_node);
                next_token(lista, pos, token);
                if token.tipe == "ID" {
                    let par_name_node = Node::new(&token.lexeme);
                    Node::add_node(&parameter_node, &par_name_node);
                    next_token(lista, pos, token);
                    if PARAMS(lista, token, pos, pai) {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    erro("function parameter with missing or invalid name", token);
                    return false;
                }
            } else {
                erro("function parameter declaration missing ':'", token);
                return false;
            }
        } else if token.lexeme != "(" {
            erro("function parameter with missing or invalid type", token);
            return false;
        }
        return true;
    }

    fn func_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        let func_body_node = Node::new("function_block");
        Node::add_node(pai, &func_body_node);
        if CMD(lista, token, pos, &func_body_node) {
            return func_block(lista, token, pos, pai);
        } else {
            return false;
        }
    }

    if token.lexeme == "function" {
        next_token(lista, pos, token);
        if token.lexeme == "main" {
            let main_dec_node = Node::new("main_function_declaration");
            Node::add_node(pai, &main_dec_node);
            if Main(lista, token, pos, &main_dec_node) {
                return true;
            } else {
                return false;
            }
        } else if token.tipe == "ID" {
            let func_dec = Node::new("function_declaration");
            let func_name = Node::new(&token.lexeme);
            let func_node = Node::new("function");
            let id_node = Node::new("ID");
            Node::add_node(pai, &func_dec);
            Node::add_node(&func_dec, &func_node);
            Node::add_node(&func_node, &id_node);
            Node::add_node(&id_node, &func_name);
            next_token(lista, pos, token);
            if token.lexeme == "(" {
                let op_node = Node::new("(");
                Node::add_node(&func_node, &op_node);
                next_token(lista, pos, token);
                let func_par_node = Node::new("function_parameters");
                Node::add_node(&func_node, &func_par_node);
                if PARAMETER(lista, token, pos, &func_par_node) {
                    if token.lexeme == ")" {
                        let cp_node = Node::new(")");
                        Node::add_node(&func_node, &cp_node);
                        next_token(lista, pos, token);
                        if token.lexeme == "{" {
                            let ocb_node = Node::new("{");
                            Node::add_node(&func_node, &ocb_node);
                            next_token(lista, pos, token);
                            if func_block(lista, token, pos, &func_node) {
                                if token.lexeme == "}" {
                                    let ccb_node = Node::new("}");
                                    Node::add_node(&func_node, &ccb_node);
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
            erro("Function declaration with missing or invalid name", token);
            return false;
        }
    } else {
        return false;
    }
}

fn is_declaration(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn DEC_TYPE(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.tipe == "Reserved_FLOAT"
            || token.tipe == "Reserved_INT"
            || token.tipe == "Reserved_CHAR"
            || token.tipe == "Reserved_BOOL"
            || token.tipe == "Reserved_VOID"
        {
            let type_node = Node::new(&token.lexeme);
            Node::add_node(pai, &type_node);
            next_token(lista, pos, token);
            return true;
        } else {
            return false;
        }
    }

    fn DECLARATION(lista: &Vec<Token>, pos: &mut usize, token: &mut Token, pai: &NodeRef) -> bool {
        if token.lexeme == "function" {
            if FUNC(lista, token, pos, pai) {
                return true;
            }
            return false;
        } else if token.lexeme == "scanln" {
            if scanln(lista, token, pos, pai) {
                return true;
            }
            return false;
        } else if VAR(lista, pos, token, pai) {
            return true;
        } else {
            return false;
        }
    }

    let dec_call_node = Node::new("declaration_call");
    Node::add_node(pai, &dec_call_node);
    if DEC_TYPE(lista, token, pos, &dec_call_node) {
        if token.lexeme == ":" {
            let dp_node = Node::new(":");
            Node::add_node(&dec_call_node, &dp_node);
            next_token(lista, pos, token);
            if DECLARATION(lista, pos, token, &dec_call_node) {
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

fn is_if(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn if_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "}" {
            return true;
        }

        if CMD(lista, token, pos, pai) {
            return if_block(lista, token, pos, pai);
        } else {
            return false;
        }
    }

    fn EXP_EL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if (token.lexeme == "&") {
            next_token(lista, pos, token);
            if (token.lexeme == "&") {
                let and_node = Node::new("&&");
                Node::add_node(pai, &and_node);
                next_token(lista, pos, token);
                if COMPARATION(lista, token, pos, pai) {
                    if EXP_EL(lista, token, pos, pai) {
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

    fn EXP_E(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if COMPARATION(lista, token, pos, pai) {
            if EXP_EL(&lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn EXP_OUL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "|" {
            next_token(lista, pos, token);
            if token.lexeme == "|" {
                let or_node = Node::new("||");
                Node::add_node(pai, &or_node);
                next_token(lista, pos, token);
                if EXP_E(lista, token, pos, pai) {
                    if EXP_OUL(lista, token, pos, pai) {
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

    fn EXP_OU(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if EXP_E(lista, token, pos, pai) {
            if EXP_OUL(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn COND(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if EXP_OU(lista, token, pos, pai) {
            return true;
        } else {
            return false;
        }
    }

    fn is_elseif(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if is_if(lista, token, pos, pai) {
            return true;
        } else if (token.lexeme == "{") {
            let ocb_node = Node::new("{");
            Node::add_node(pai, &ocb_node);
            next_token(lista, pos, token);
            let if_body_node = Node::new("if_block");
            Node::add_node(pai, &if_body_node);
            if if_block(&lista, token, pos, &if_body_node) {
                if (token.lexeme == "}") {
                    let ccb_node = Node::new("}");
                    Node::add_node(pai, &ccb_node);
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

    fn is_else(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "else" {
            let else_node = Node::new("else");
            Node::add_node(pai, &else_node);
            next_token(lista, pos, token);
            if is_elseif(lista, token, pos, &else_node) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    let if_condition = Node::new("if_condition");
    if token.lexeme == "if" {
        let if_node = Node::new("if");
        Node::add_node(pai, &if_condition);
        Node::add_node(&if_condition, &if_node);
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            let cond_node = Node::new("condition");
            Node::add_node(&if_condition, &op_node);
            Node::add_node(&if_condition, &cond_node);
            next_token(lista, pos, token);
            if COND(&lista, token, pos, &cond_node) {
                if token.lexeme == ")" {
                    let cp_node = Node::new(")");
                    Node::add_node(&if_condition, &cp_node);
                    next_token(lista, pos, token);
                    if token.lexeme == "{" {
                        let ocb_node = Node::new("{");
                        Node::add_node(&if_condition, &ocb_node);
                        next_token(lista, pos, token);
                        let if_body_node = Node::new("if_block");
                        Node::add_node(&if_condition, &if_body_node);
                        if if_block(lista, token, pos, &if_body_node) {
                            if token.lexeme == "}" {
                                let ccb_node = Node::new("}");
                                Node::add_node(&if_condition, &ccb_node);
                                next_token(lista, pos, token);
                                if is_else(&lista, token, pos, &if_condition) {
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

fn scanln(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    let scanln_call_node = Node::new("scanln_call");
    let scanln_node = Node::new("scanln");
    if token.tipe == "Reserved_scanln" {
        Node::add_node(pai, &scanln_call_node);
        Node::add_node(&scanln_call_node, &scanln_node);
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            Node::add_node(&scanln_call_node, &op_node);
            next_token(lista, pos, token);
            if token.lexeme == ")" {
                let cp_node = Node::new(")");
                Node::add_node(&scanln_call_node, &cp_node);
                next_token(lista, pos, token);
                return true;
            } else {
                erro("missing closing ')' in scanln", token);
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

fn println(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn vars(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "," {
            let coma_node = Node::new(",");
            Node::add_node(pai, &coma_node);
            next_token(lista, pos, token);
            if token.tipe == "ID" || token.tipe == "Integer" || token.tipe == "Floating_Point" {
                let type_node = Node::new(&token.tipe);
                let id_name_nome = Node::new(&token.lexeme);
                Node::add_node(pai, &type_node);
                Node::add_node(&type_node, &id_name_nome);
                next_token(lista, pos, token);
                if vars(lista, token, pos, pai) {
                    return true;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    let println_call_node = Node::new("println_call");
    let println_node = Node::new("println");

    if token.tipe == "Reserved_println" {
        Node::add_node(pai, &println_call_node);
        Node::add_node(&println_call_node, &println_node);
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            Node::add_node(&println_call_node, &op_node);
            next_token(lista, pos, token);
            if token.tipe == "string" {
                let string_node = Node::new("String");
                let string_content_node = Node::new(&token.lexeme);
                let println_vars_node = Node::new("println_vars");
                Node::add_node(&println_call_node, &string_node);
                Node::add_node(&string_node, &string_content_node);
                Node::add_node(&println_call_node, &println_vars_node);
                next_token(lista, pos, token);
                if vars(lista, token, pos, &println_vars_node) {
                    if token.lexeme == ")" {
                        let cp_node = Node::new(")");
                        Node::add_node(&println_call_node, &cp_node);
                        next_token(lista, pos, token);
                        let end_node = Node::new(";");
                        Node::add_node(&println_call_node, &end_node);
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

fn is_while(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn while_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos, pai) {
            return while_block(lista, token, pos, pai);
        } else {
            return false;
        }
    }

    fn E_PARL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "&" {
            next_token(lista, pos, token);
            if token.lexeme == "&" {
                let and_node = Node::new("&&");
                Node::add_node(pai, &and_node);
                next_token(lista, pos, token);
                if COMPARATION(lista, token, pos, pai) {
                    if E_PARL(lista, token, pos, pai) {
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

    fn E_PAR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if COMPARATION(lista, token, pos, pai) {
            if E_PARL(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn OU_PARL(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "|" {
            next_token(lista, pos, token);
            if token.lexeme == "|" {
                let or_node = Node::new("||");
                Node::add_node(pai, &or_node);
                next_token(lista, pos, token);
                if E_PAR(lista, token, pos, pai) {
                    if OU_PARL(lista, token, pos, pai) {
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

    fn OU_PAR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if E_PAR(lista, token, pos, pai) {
            if OU_PARL(lista, token, pos, pai) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn WPARAMETERS(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.tipe == "Reserved_TRUE" {
            let true_node = Node::new("TRUE");
            Node::add_node(pai, &true_node);
            next_token(lista, pos, token);
            return true;
        } else if OU_PAR(lista, token, pos, pai) {
            return true;
        } else {
            return false;
        }
    }
    let while_call_node = Node::new("while_loop");
    if token.tipe == "Reserved_while" {
        let while_node = Node::new("while");
        Node::add_node(pai, &while_call_node);
        Node::add_node(&while_call_node, &while_node);
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            Node::add_node(&while_node, &op_node);
            next_token(lista, pos, token);
            let while_par_node = Node::new("while_parameters");
            Node::add_node(&while_node, &while_par_node);
            if WPARAMETERS(lista, token, pos, &while_par_node) {
                if token.lexeme == ")" {
                    let cp_node = Node::new(")");
                    Node::add_node(&while_node, &cp_node);
                    next_token(lista, pos, token);
                    if token.lexeme == "{" {
                        let ocb_node = Node::new("{");
                        Node::add_node(&while_node, &ocb_node);
                        next_token(lista, pos, token);
                        let while_block_node = Node::new("while_block");
                        Node::add_node(&while_node, &while_block_node);
                        if while_block(lista, token, pos, &while_block_node) {
                            if token.lexeme == "}" {
                                let ccb_node = Node::new("}");
                                Node::add_node(&while_node, &ccb_node);
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

fn is_for(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    fn for_block(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        if token.lexeme == "}" {
            return true;
        }
        if CMD(lista, token, pos, pai) {
            return for_block(lista, token, pos, pai);
        } else {
            return false;
        }
    }

    fn COMPARATOR(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
        let it_node = Node::new("iterator");
        Node::add_node(pai, &it_node);
        if Type(lista, token, pos, &it_node) {
            let type_node = Node::new(&token.lexeme);
            Node::add_node(&it_node, &type_node);
            next_token(lista, pos, token);
            if token.lexeme == ":" {
                let dp_node = Node::new(":");
                Node::add_node(&it_node, &dp_node);
                next_token(lista, pos, token);
                if VAR(lista, pos, token, &it_node) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else if token.tipe == "ID" {
            let id_node = Node::new("ID");
            let id_name_node = Node::new(&token.lexeme);
            Node::add_node(&it_node, &id_node);
            Node::add_node(&id_node, &id_name_node);
            next_token(lista, pos, token);
            if token.lexeme == ";" {
                let end_node = Node::new(";");
                Node::add_node(&it_node, &end_node);
                next_token(lista, pos, token);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    let for_loop_node = Node::new("for_loop");
    if token.tipe == "Reserved_for" {
        let for_node = Node::new("for");
        Node::add_node(pai, &for_loop_node);
        Node::add_node(&for_loop_node, &for_node);
        next_token(lista, pos, token);
        if token.lexeme == "(" {
            let op_node = Node::new("(");
            next_token(lista, pos, token);
            if COMPARATOR(lista, token, pos, &for_node) {
                if COMPARATION(lista, token, pos, &for_node) {
                    if token.lexeme == ";" {
                        let end_node = Node::new(";");
                        Node::add_node(&for_node, &end_node);
                        next_token(lista, pos, token);
                        if is_atribuicao_interna(lista, token, pos, &for_node) {
                            if token.lexeme == ")" {
                                let cp_node = Node::new(")");
                                Node::add_node(&for_node, &cp_node);
                                next_token(lista, pos, token);
                                if token.lexeme == "{" {
                                    let ocb_node = Node::new("{");
                                    Node::add_node(&for_node, &ocb_node);
                                    next_token(lista, pos, token);
                                    let for_block_node = Node::new("for_block");
                                    Node::add_node(&for_node, &for_block_node);
                                    if for_block(lista, token, pos, &for_block_node) {
                                        if token.lexeme == "}" {
                                            let ccb_node = Node::new("}");
                                            Node::add_node(&for_node, &ccb_node);
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

fn CMD(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    if is_if(lista, token, pos, pai) {
        return true;
    } else if Type(lista, token, pos, pai) {
        if is_declaration(lista, token, pos, pai) {
            return true;
        }
        return false;
    } else if token.tipe == "ID" {
        if is_atribuicao(lista, token, pos, pai) {
            return true;
        }
        return false;
    } else if Return(lista, token, pos, pai) {
        return true;
    } else if println(lista, token, pos, pai) {
        return true;
    } else if scanln(lista, token, pos, pai) {
        return true;
    } else if is_while(lista, token, pos, pai) {
        return true;
    } else if Continue(lista, token, pos, pai) {
        return true;
    } else if Break(lista, token, pos, pai) {
        return true;
    } else if is_for(lista, token, pos, pai) {
        return true;
    } else if token.lexeme == "call" {
        if func_call(lista, token, pos, pai) {
            return true;
        }
        return false;
    } else {
        return false;
    }
}

fn bloco(lista: &Vec<Token>, token: &mut Token, pos: &mut usize, pai: &NodeRef) -> bool {
    if token.tipe == "EOF" {
        let eof_node = Node::new("EOF");
        Node::add_node(pai, &eof_node);
        return true;
    }
    if CMD(lista, token, pos, pai) {
        return bloco(lista, token, pos, pai);
    }

    return false;
}

pub fn parser(lista: Vec<Token>) -> (bool, Tree) {
    let mut pos: usize = 0;
    let mut token: Token = Token::new("", "");
    let mut root = Node::new("Bloco");

    next_token(&lista, &mut pos, &mut token);
    let result: bool = bloco(&lista, &mut token, &mut pos, &root);

    let mut arvore: Tree = Tree::new(root);
    if result {
        (true, arvore)
    } else {
        (false, arvore)
    }
}

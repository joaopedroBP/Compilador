use crate::arvore::NodeRef;
use std::cell::RefCell;
use std::fs;
use std::process::Command;
use std::rc::Rc;

fn map_type_to_rust(tipo: &str) -> &'static str {
    match tipo {
        "INT" => "i32",
        "FLOAT" => "f32",
        "CHAR" => "char",
        "VOID" => "()",
        "BOOL" => "bool",
        _ => "i32",
    }
}

fn get_id_or_literal_value(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    if let Some(child_node) = node.nodes.get(0) {
        return child_node.borrow().nome.clone().trim().to_string();
    }

    node.nome.clone().trim().to_string()
}

fn gerar_expressao(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();

    let codigo_filhos: Vec<String> = node
        .nodes
        .iter()
        .map(gerar_codigo)
        .filter(|s| !s.is_empty())
        .collect();

    let mut resultado = codigo_filhos.join(" ");

    resultado = resultado.replace("( ", "(").replace(" )", ")");
    resultado = resultado.replace("- ", "-");

    resultado.trim().to_string()
}

fn extrair_condicao(node_ref: &NodeRef) -> String {
    node_ref
        .borrow()
        .nodes
        .iter()
        .map(gerar_codigo)
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .trim()
        .to_string()
}

fn extrair_corpo_bloco(node_ref: &NodeRef) -> String {
    node_ref
        .borrow()
        .nodes
        .iter()
        .map(gerar_codigo)
        .filter(|s| !s.is_empty())
        .collect::<String>()
}

fn gerar_codigo(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();

    if node.nodes.is_empty()
        && node.nome.chars().next().map_or(false, |c| {
            c.is_digit(10) || c == '-' || c.is_ascii_punctuation()
        })
    {
        return node.nome.clone();
    }

    match node.nome.as_str() {
        "Bloco" => node.nodes.iter().map(gerar_codigo).collect::<String>(),

        "main_function_declaration" | "function_declaration" => {
            node.nodes.iter().map(gerar_codigo).collect::<String>()
        }

        "declaration_call" => gerar_declaracao(node_ref),
        "if_condition" => gerar_if(node_ref),
        "while_loop" => gerar_while(node_ref),
        "for_loop" => gerar_for(node_ref),

        "internal_function_call" => gerar_function_call(node_ref),
        "attribution_call" => gerar_atribuicao(node_ref),
        "internal_attribution_call" => gerar_atribuicao_interna(node_ref),
        "function_call" => gerar_function_call_stmt(node_ref),

        "println_call" => gerar_println(node_ref),
        "scanln_call" => gerar_scanln(node_ref),

        "return_call" => {
            let mut codigo = String::from("return ");
            if let Some(expressao_retorno) = node
                .nodes
                .iter()
                .find(|c| c.borrow().nome != "return" && c.borrow().nome != ";")
            {
                codigo.push_str(&gerar_codigo(expressao_retorno));
            }
            codigo.push_str(";\n");
            codigo
        }
        "break_call" => "break;\n".to_string(),
        "Continue_call" => "continue;\n".to_string(),

        "condition" | "comparation" | "operation" | "direct_comparation" => {
            gerar_expressao(node_ref)
        }

        "iterator" => {
            let iterator_children = &node.nodes;

            let tipo_node = iterator_children.iter().find(|c| {
                c.borrow().nome.chars().all(|ch| ch.is_ascii_uppercase())
                    && c.borrow().nome.len() <= 5
            });

            let var_decl_node = iterator_children
                .iter()
                .find(|c| c.borrow().nome == "variable_declaration");

            if let (Some(t_node), Some(v_node)) = (tipo_node, var_decl_node) {
                let rust_tipo = map_type_to_rust(&t_node.borrow().nome);
                let var_decl_children = &v_node.borrow().nodes;

                let nome_variavel = get_id_or_literal_value(
                    var_decl_children
                        .get(0)
                        .expect("ID esperado em variable_declaration"),
                );

                let valor_atribuido = var_decl_children
                    .get(2)
                    .map(|n| gerar_codigo(n).trim().to_string())
                    .unwrap_or_else(|| "0".to_string());

                return format!(
                    "let mut {}: {} = {};\n",
                    nome_variavel, rust_tipo, valor_atribuido
                );
            }

            node.nodes.iter().map(gerar_codigo).collect::<String>()
        }

        "if_block"
        | "while_block"
        | "for_block"
        | "function_block"
        | "main_block"
        | "else"
        | "while_parameters"
        | "function_parameters"
        | "variable_declaration"
        | "direct_attribution"
        | "simple_atribution"
        | "function_call_arguments" => node.nodes.iter().map(gerar_codigo).collect::<String>(),

        "ID" | "Integer" | "String" | "character" | "FLOAT" => get_id_or_literal_value(node_ref),

        "TRUE" => "true".to_string(),
        "FALSE" => "false".to_string(),
        "comparation_operator" => {
            let mut operador = node
                .nodes
                .iter()
                .map(|c| c.borrow().nome.clone())
                .collect::<String>();

            if operador.len() == 2 && operador.contains('=') && !operador.contains('!') {
                operador = "==".to_string();
            }
            operador
        }

        "reduction" => "-= 1".to_string(),
        "increment" => "+= 1".to_string(),
        "println" | "scanln" | "function" | "while" | "for" | "if" | "else" | "return" | "call" => {
            "".to_string()
        }
        ("(" | ")" | "{" | "}" | ";" | "," | "+" | "-" | "*" | "||" | "=" | ":" | "<" | ">"
        | "/") => node.nome.clone(),

        _ => node.nodes.iter().map(gerar_codigo).collect::<String>(),
    }
}

fn gerar_declaracao(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();

    let tipo_node = node.nodes.get(0).expect("Nó de tipo esperado");
    let tipo = tipo_node.borrow().nome.clone();
    let rust_tipo = map_type_to_rust(&tipo);

    let main_fn_node = node
        .nodes
        .iter()
        .find(|c| c.borrow().nome == "main_function_declaration");
    let func_node = node
        .nodes
        .iter()
        .find(|c| c.borrow().nome == "function_declaration");
    let var_node = node
        .nodes
        .iter()
        .find(|c| c.borrow().nome == "variable_declaration");

    if let Some(main) = main_fn_node {
        let mut codigo = String::from("fn main() {\n");
        if let Some(bloco_main) = main
            .borrow()
            .nodes
            .iter()
            .find(|c| c.borrow().nome == "main_block")
        {
            codigo.push_str(&gerar_codigo(bloco_main));
        }
        codigo.push_str("}\n");
        codigo
    } else if let Some(funcao_node) = func_node {
        let filhos_funcao_declaration = &funcao_node.borrow().nodes;

        let no_function = filhos_funcao_declaration
            .iter()
            .find(|c| c.borrow().nome == "function")
            .expect("Esperado nó 'function' para função comum");
        let filhos_no_function = &no_function.borrow().nodes;

        let no_id = filhos_no_function
            .iter()
            .find(|c| c.borrow().nome == "ID")
            .expect("Função sem nome ID");
        let nome_funcao = get_id_or_literal_value(no_id);

        let mut parametros = String::new();
        if let Some(param_node) = filhos_no_function
            .iter()
            .find(|c| c.borrow().nome == "function_parameters")
        {
            let mut lista_parametros = Vec::new();
            for p_ref in &param_node.borrow().nodes {
                let p = p_ref.borrow();
                if p.nome == "parameter" {
                    let p_tipo = p
                        .nodes
                        .get(0)
                        .map(|n| n.borrow().nome.clone())
                        .unwrap_or("INT".to_string());

                    let p_id_node = p.nodes.get(2).expect("Parâmetro sem ID no índice 2");
                    let p_id = get_id_or_literal_value(p_id_node);

                    let p_rust_tipo = map_type_to_rust(&p_tipo);
                    lista_parametros.push(format!("{}: {}", p_id, p_rust_tipo));
                }
            }
            parametros = lista_parametros.join(", ");
        }

        let mut codigo = format!("fn {}({}) -> {} {{\n", nome_funcao, parametros, rust_tipo);

        if let Some(bloco_funcao) = filhos_no_function
            .iter()
            .find(|c| c.borrow().nome == "function_block")
        {
            codigo.push_str(&gerar_codigo(bloco_funcao));
        }

        codigo.push_str("}\n");
        codigo
    } else if let Some(variavel_node) = var_node {
        let variavel_emprestada = variavel_node.borrow();

        let nome_variavel = get_id_or_literal_value(
            variavel_emprestada
                .nodes
                .get(0)
                .expect("Esperado ID da variável (índice 0)"),
        );

        let valor_atribuido_full = variavel_emprestada
            .nodes
            .get(2)
            .map(|n| gerar_codigo(n))
            .unwrap_or_else(|| match rust_tipo {
                "i32" | "f32" => "0".to_string(),
                "char" => "' '".to_string(),
                _ => "0".to_string(),
            });

        let valor_atribuido = valor_atribuido_full;

        if valor_atribuido.trim().is_empty() {
            return format!(
                "// ERRO: Expressão de inicialização inválida para: {} \n",
                nome_variavel
            );
        }

        format!(
            "let mut {}: {} = {};\n",
            nome_variavel, rust_tipo, valor_atribuido
        )
    } else {
        node.nodes.iter().map(gerar_codigo).collect::<String>()
    }
}

fn gerar_if(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let mut codigo = String::new();

    codigo.push_str("if ");
    if let Some(no_condicao) = node.nodes.iter().find(|c| c.borrow().nome == "condition") {
        codigo.push_str(&gerar_codigo(no_condicao));
    }
    codigo.push_str(" {\n");

    if let Some(bloco_if) = node.nodes.iter().find(|c| c.borrow().nome == "if_block") {
        codigo.push_str(&gerar_codigo(bloco_if));
    }
    codigo.push_str("}");

    if let Some(no_else) = node.nodes.iter().find(|c| c.borrow().nome == "else") {
        codigo.push_str(" else ");
        codigo.push_str(&gerar_codigo(no_else));
    } else {
        codigo.push_str("\n");
    }

    codigo
}

fn gerar_while(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();

    let while_content_node_ref = match node.nodes.get(0) {
        Some(n) => n,
        None => return "/* Erro: while sem conteúdo (nó literal 'while' ausente) */\n".to_string(),
    };
    let content_children = &while_content_node_ref.borrow().nodes;

    let mut codigo = String::from("while ");

    if let Some(no_parametros) = content_children
        .iter()
        .find(|c| c.borrow().nome == "while_parameters")
    {
        codigo.push_str(&extrair_condicao(no_parametros));
    } else if content_children.iter().any(|c| c.borrow().nome == "TRUE") {
        codigo.push_str("true");
    } else {
        codigo.push_str("/* Condição Desconhecida */");
    }

    codigo.push_str(" {\n");

    if let Some(bloco_while) = content_children
        .iter()
        .find(|c| c.borrow().nome == "while_block")
    {
        codigo.push_str(&extrair_corpo_bloco(bloco_while));
    }

    codigo.push_str("}\n");
    codigo
}

fn gerar_for(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let mut codigo = String::new();

    let for_content_node_ref = match node.nodes.get(0) {
        Some(n) => n,
        None => return "/* Erro: for sem conteúdo (nó literal 'for' ausente) */\n".to_string(),
    };
    let content_children = &for_content_node_ref.borrow().nodes;

    if let Some(no_iterator) = content_children
        .iter()
        .find(|c| c.borrow().nome == "iterator")
    {
        codigo.push_str(&gerar_codigo(no_iterator));
    }

    codigo.push_str("while ");

    if let Some(no_condicao) = content_children
        .iter()
        .find(|c| c.borrow().nome == "comparation")
    {
        codigo.push_str(&gerar_expressao(no_condicao));
    } else {
        codigo.push_str("true");
    }

    codigo.push_str(" {\n");

    if let Some(bloco_for) = content_children
        .iter()
        .find(|c| c.borrow().nome == "for_block")
    {
        codigo.push_str(&extrair_corpo_bloco(bloco_for));
    }

    if let Some(no_passo) = content_children
        .iter()
        .filter(|c| c.borrow().nome == "internal_attribution_call")
        .last()
    {
        codigo.push_str(&gerar_codigo(no_passo));
    }

    codigo.push_str("}\n");
    codigo
}

fn gerar_function_call_stmt(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let mut codigo = String::new();

    if let Some(no_chamada) = node.nodes.get(0) {
        codigo.push_str(&gerar_function_call(no_chamada));
    }
    codigo.push_str(";\n");
    codigo
}

fn gerar_function_call(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let mut codigo = String::new();

    if let Some(no_id) = node.nodes.iter().find(|c| c.borrow().nome == "ID") {
        codigo.push_str(&get_id_or_literal_value(no_id));
        codigo.push_str("(");

        if let Some(no_argumentos) = node
            .nodes
            .iter()
            .find(|c| c.borrow().nome == "function_call_arguments")
        {
            let argumentos: Vec<String> = no_argumentos
                .borrow()
                .nodes
                .iter()
                .filter(|c| c.borrow().nome != ",")
                .map(gerar_codigo)
                .collect();
            codigo.push_str(&argumentos.join(", "));
        }
        codigo.push_str(")");
    }

    codigo
}

fn gerar_println(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let mut codigo = String::from("println!");

    if let Some(no_string) = node.nodes.iter().find(|c| c.borrow().nome == "String") {
        let valor_string = get_id_or_literal_value(no_string);

        let mut argumentos = vec![valor_string];

        if let Some(no_variaveis) = node
            .nodes
            .iter()
            .find(|c| c.borrow().nome == "println_vars")
        {
            let variaveis: Vec<String> = no_variaveis
                .borrow()
                .nodes
                .iter()
                .filter(|c| c.borrow().nome != ",")
                .map(gerar_codigo)
                .collect();
            argumentos.extend(variaveis);
        }

        codigo.push_str(&format!("({});\n", argumentos.join(", ")));
    } else {
        codigo.push_str("();\n");
    }

    codigo
}

fn gerar_scanln(_node_ref: &NodeRef) -> String {
    String::from(
        r#"{
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Falha ao ler linha"); 
    input.trim().parse().unwrap_or_else(|_| 0) 
}"#,
    )
}

fn gerar_atribuicao_interna(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();

    let nome_variavel = get_id_or_literal_value(
        node.nodes
            .get(0)
            .expect("Esperado ID da variável na atribuição (índice 0)"),
    );

    let operador_node = node
        .nodes
        .get(1)
        .expect("Esperado operador de atribuição (índice 1)");
    let operador_str = operador_node.borrow().nome.clone();

    if operador_str == "=" {
        let valor_atribuido = gerar_codigo(
            node.nodes
                .get(2)
                .expect("Esperado valor de atribuição para '=' (índice 2)"),
        )
        .trim()
        .to_string();
        return format!("{} = {};\n", nome_variavel, valor_atribuido);
    }

    if node
        .nodes
        .iter()
        .any(|c| c.borrow().nome == "simple_atribution")
    {
        return format!("{} += 1;\n", nome_variavel);
    }

    if node.nodes.len() > 3 {
        let operador_composto = node
            .nodes
            .get(1)
            .map(|c| c.borrow().nome.clone())
            .unwrap_or_default();

        let valor = node
            .nodes
            .iter()
            .find(|c| c.borrow().nome == "direct_attribution")
            .map(|c| gerar_codigo(c))
            .unwrap_or("1".to_string());

        return format!(
            "{} {}= {};\n",
            nome_variavel,
            operador_composto.trim(),
            valor.trim()
        );
    }

    format!("// ERRO: Atribuição malformada para: {} \n", nome_variavel)
}

fn gerar_atribuicao(node_ref: &NodeRef) -> String {
    let node = node_ref.borrow();
    let codigo = gerar_codigo(
        node.nodes
            .get(0)
            .expect("Esperado nó filho 'internal_attribution_call' em attribution_call"),
    );
    codigo
}

pub fn salvar_e_formatar(root_node: &NodeRef, caminho: &str) -> std::io::Result<()> {
    let codigo_gerado = gerar_codigo(root_node);

    let mut codigo_final = String::from("use std::io;\n\n");
    codigo_final.push_str(&codigo_gerado);

    fs::write(caminho, codigo_final)?;

    println!("Código salvo em: {}", caminho);
    println!("Formatando com cargo fmt...");

    let status = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg(caminho)
        .status()
        .expect("Falha ao executar cargo fmt");

    if status.success() {
        println!("Código formatado com sucesso!");
    } else {
        eprintln!("cargo fmt não finalizou com sucesso");
    }

    Ok(())
}

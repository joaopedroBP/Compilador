mod arvore;
mod lexer;
mod parser;

use std::fs::File;
//use std::io::Write;
fn main() {
    //    {
    //        let mut file = File::create("Code.txt").unwrap();
    //        file.write(b"Hello World\n this is another line").unwrap();
    //    }

    let file = File::open("Code.txt").unwrap();
    let lista: Vec<lexer::Token> = lexer::get_tokens(file);

    for token in &lista {
        println!("{}", token.to_string());
    }

    let parser_result = parser::parser(lista);

    if (parser_result) {
        println!("codigo válido");
    } else {
        println!("codigo inválido");
    }

    let mut nodeA: arvore::Node = arvore::Node::new("A");
    let mut nodeB: arvore::Node = arvore::Node::new("B");
    let mut nodeC: arvore::Node = arvore::Node::new("C");
    let mut nodeD: arvore::Node = arvore::Node::new("D");
    let mut nodeE: arvore::Node = arvore::Node::new("E");
    let mut nodeF: arvore::Node = arvore::Node::new("F");

    nodeA.add_node_name(nodeB.clone());
    nodeA.add_node_name(nodeC.clone());
    nodeA.add_node_name(nodeD.clone());
    nodeC.add_node_name(nodeE.clone());
    nodeC.add_node_name(nodeB.clone());

    let mut arvore: arvore::Tree = arvore::Tree::new(nodeA);
    arvore.pre_ordem_raiz();
    arvore.print_code_root();
    arvore.print_tree();
}

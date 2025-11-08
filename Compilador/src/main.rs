mod arvore;
mod lexer;
mod parser;
use arvore::Node;
use arvore::Tree;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

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

    if parser_result {
        println!("codigo válido");
    } else {
        println!("codigo inválido");
    }

    let node_a = Node::new("A");
    let node_b = Node::new("B");
    let node_c = Node::new("C");
    let node_d = Node::new("D");
    let node_e = Node::new("E");
    let node_f = Node::new("F");

    Node::add_node(&node_a, &node_b);
    Node::add_node(&node_a, &node_c);
    Node::add_node(&node_a, &node_d);
    Node::add_node(&node_c, &node_e);
    Node::add_node(&node_c, &node_f);

    let arvore = Tree::new(node_a);
    arvore.print_tree();
}

use std::cell::RefCell;
use std::fs;
use std::process::Command;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
    pub nome: String,
    pub nodes: Vec<NodeRef>,
    pub enter: String,
    pub exit: String,
}

impl Node {
    pub fn new(nome: &str) -> NodeRef {
        Rc::new(RefCell::new(Node {
            nome: nome.to_string(),
            nodes: Vec::new(),
            enter: "".to_string(),
            exit: "".to_string(),
        }))
    }

    pub fn add_node(self_ref: &NodeRef, new_node: &NodeRef) {
        self_ref.borrow_mut().nodes.push(Rc::clone(new_node));
    }

    pub fn add_node_name(self_ref: &NodeRef, nome: &str) -> NodeRef {
        let new_node = Node::new(nome);
        self_ref.borrow_mut().nodes.push(Rc::clone(&new_node));
        return new_node;
    }

    pub fn add_node_full(self_ref: &NodeRef, enter: &str, nome: &str, exit: &str) -> NodeRef {
        let new_node = Node::new(nome);
        {
            let mut n = new_node.borrow_mut();
            n.enter = enter.to_string();
            n.exit = exit.to_string();
        }
        self_ref.borrow_mut().nodes.push(Rc::clone(&new_node));
        return new_node;
    }

    pub fn to_string(&self) -> String {
        let tostr = format!("{} {} {}", self.enter, self.nome, self.exit);
        return tostr;
    }

    pub fn get_tree(self_ref: &NodeRef) -> String {
        println!("AST");
        let mut buffer = String::with_capacity(50);
        Self::print(self_ref, &mut buffer, "", "");
        return buffer;
    }

    fn print(self_ref: &NodeRef, buffer: &mut String, prefix: &str, children_prefix: &str) {
        let node = self_ref.borrow();
        buffer.push_str(prefix);
        buffer.push_str(&node.nome);
        buffer.push('\n');

        let total = node.nodes.len();
        for (i, child) in node.nodes.iter().enumerate() {
            let (new_prefix, new_children_prefix) = if i < total - 1 {
                (
                    format!("{}+-- ", children_prefix),
                    format!("{}|   ", children_prefix),
                )
            } else {
                (
                    format!("{}'-- ", children_prefix),
                    format!("{}    ", children_prefix),
                )
            };

            Self::print(child, buffer, &new_prefix, &new_children_prefix);
        }
    }
}

pub struct Tree {
    pub root: NodeRef,
}

impl Tree {
    pub fn new(root: NodeRef) -> Tree {
        Tree { root: root }
    }

    pub fn pre_ordem_raiz(&self) {
        Self::pre_ordem(&self.root);
        println!();
    }

    pub fn pre_ordem(node_ref: &NodeRef) {
        let node = node_ref.borrow();
        print!("{}", node.to_string());
        for n in &node.nodes {
            Self::pre_ordem(n);
        }
    }

    pub fn print_code_root(&self) {
        Self::print_code(&self.root);
    }

    pub fn print_code(node_ref: &NodeRef) {
        let node = node_ref.borrow();
        print!("{}", node.enter);
        if node.nodes.is_empty() {
            print!("{}", node.to_string());
        }
        for n in &node.nodes {
            Self::print_code(n);
        }
        print!("{}", node.exit);
    }

    pub fn print_tree(&self) {
        println!("{}", Node::get_tree(&self.root));
    }

    pub fn gerar_codigo_rust(&self) -> String {
        Self::gerar_rec(&self.root)
    }

    fn gerar_rec(node_ref: &NodeRef) -> String {
        let node = node_ref.borrow();
        let mut codigo = String::new();

        codigo.push_str(&node.enter);

        if node.nodes.is_empty() {
            codigo.push_str(&node.nome);
        } else {
            for child in &node.nodes {
                codigo.push_str(&Self::gerar_rec(child));
            }
        }

        codigo.push_str(&node.exit);
        codigo
    }
}

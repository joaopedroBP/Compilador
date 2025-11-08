#[derive(Clone)]
pub struct Node {
    pub nome: String,
    pub nodes: Vec<Node>,
    pub enter: String,
    pub exit: String,
}

impl Node {
    pub fn new(nome: &str) -> Node {
        Node {
            nome: nome.to_string(),
            nodes: Vec::new(),
            enter: "".to_string(),
            exit: "".to_string(),
        }
    }

    pub fn add_node_name(&mut self, new_node: Node) {
        self.nodes.push(new_node);
    }

    pub fn add_node(&mut self, nome: &str) -> &mut Node {
        let mut newNode: Node = Node::new(nome);
        self.nodes.push(newNode);
        let index = self.nodes.len() - 1;
        return &mut self.nodes[index];
    }

    pub fn add_node_full(&mut self, enter: &str, nome: &str, exit: &str) -> &mut Node {
        let mut newNode: Node = Node::new(nome);
        newNode.enter = enter.to_string();
        newNode.exit = exit.to_string();
        self.nodes.push(newNode);
        let index = self.nodes.len() - 1;
        return &mut self.nodes[index];
    }

    pub fn to_string(&self) -> String {
        let tostr = format!("{} {} {}", self.enter, self.nome, self.exit);
        return tostr;
    }

    pub fn get_tree(&self) -> String {
        println!("AST");
        let mut buffer = String::with_capacity(50);
        self.print(&mut buffer, "", "");
        buffer
    }

    fn print(&self, buffer: &mut String, prefix: &str, children_prefix: &str) {
        buffer.push_str(prefix);
        buffer.push_str(&self.nome);
        buffer.push('\n');

        let total = self.nodes.len();
        for (i, child) in self.nodes.iter().enumerate() {
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

            child.print(buffer, &new_prefix, &new_children_prefix);
        }
    }
}

pub struct Tree {
    pub root: Node,
}

impl Tree {
    pub fn new(root: Node) -> Tree {
        Tree { root: root }
    }

    pub fn pre_ordem_raiz(&self) {
        Self::pre_ordem(&self.root);
        println!();
    }

    pub fn pre_ordem(node: &Node) {
        print!("{}", node.to_string());
        for n in &node.nodes {
            Self::pre_ordem(n);
        }
    }

    pub fn print_code_root(&self) {
        Self::print_code(&self.root);
    }

    pub fn print_code(node: &Node) {
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
        println!("{}", self.root.get_tree());
    }
}

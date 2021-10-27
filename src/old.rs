extern crate generational_arena;
use generational_arena::Arena;
pub use generational_arena::Index;

pub struct DomainLookupTree {
    nodes: Vec<Node>,
}

pub struct Node {
    children: Vec<Node>,
    data: String,
}

impl DomainLookupTree {
    pub fn new() -> Self {
        DomainLookupTree {
            nodes: Vec::from([]),
        }
    }

    pub fn insert(&mut self, data: &str) {
        self.nodes.push(Node {
            children: Vec::new(),
            data: data.to_owned(),
        })
    }

    pub fn traverse(&self, domain: &str) -> Option<Node> {
        let mut segments: Vec<&str> = domain.split(".").collect::<Vec<&str>>();
        segments.reverse();

        let mut head = &self.nodes;
        'sl: for d in segments {
            println!("{}", d);
            let mut found = false;
            for node in head {
                if node.data == d {
                    head = &node.children;
                    found = true;
                    break;
                }
                continue;
            }

            if !found {
                return None;
            }

            return 
        }
        return None;
    }
}

#[test]
fn create_tree_and_traverse() {
    let mut tree = DomainLookupTree::new();
    tree.insert("test.com");

    assert_eq!(tree.nodes.len(), 1);
}

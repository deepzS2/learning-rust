use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    val: String,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: String) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: String) {
        if val > self.val {
            match &self.right {
                None => self.right = Some(Rc::new(RefCell::new(Self::new(val)))),
                Some(node) => node.borrow_mut().insert(val.to_string()),
            }
        } else {
            match &self.left {
                None => self.left = Some(Rc::new(RefCell::new(Self::new(val)))),
                Some(node) => node.borrow_mut().insert(val.to_string()),
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct BinarySearchTree {
    root: Node,
}

impl BinarySearchTree {
    fn new(val: String) -> Self {
        BinarySearchTree {
            root: Node::new(val.to_string()),
        }
    }

    fn insert(&mut self, val: String) {
        self.root.insert(val.to_string());
    }
}

struct DisplayLobby {
    stack: Vec<Rc<RefCell<Node>>>,
}

impl DisplayLobby {
    fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
        let mut stack = Vec::new();

        Self::push_all_left(root.clone(), &mut stack);

        DisplayLobby { stack }
    }

    fn push_all_left(mut p: Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
        while let Some(link) = p.clone() {
            stack.push(p.clone().unwrap());

            p = link.borrow().left.clone();
        }
    }

    fn next_name(&mut self) -> String {
        let node = self.stack.pop().unwrap();
        let name = &node.borrow().val;

        let next_node = node.borrow().right.clone();

        Self::push_all_left(next_node, &mut self.stack);
        name.to_string()
    }

    fn next_page(&mut self) -> Vec<String> {
        let mut resultant_names = Vec::new();

        for _ in 0..10 {
            if !self.stack.is_empty() {
                resultant_names.push(self.next_name());
            } else {
                break;
            }
        }

        resultant_names
    }
}

fn main() {
    let mut bst = BinarySearchTree::new("Jaenetter".to_string());

    let names = vec![
        "Latasha",
        "Elvira",
        "Caryl",
        "Alan",
        "Cassie",
        "Charity",
        "Lyn",
        "Lya",
        "Anya",
        "Albert",
        "Antionetter",
        "Cherlyn",
        "Lala",
        "Andice",
        "Iliana",
        "Nouman",
        "Azam",
    ];

    for name in names.into_iter() {
        bst.insert(name.to_string());
    }

    let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));

    println!("Participants on the first page {:?}", display.next_page());
    println!("Participants on the second page {:?}", display.next_page());
}

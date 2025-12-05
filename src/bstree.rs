use std::fmt::Display;

#[allow(dead_code)]
pub struct Bstree<T :PartialEq+Display> {
    root : Box<Node<T>>,
}

#[allow(dead_code)]
enum Node<T :PartialEq+Display> {
    Value {
        value:T,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
    Nil,
}

impl<T :PartialEq+Display> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Nil, Node::Nil) => true,
            (Node::Value { value: v1,.. }, Node::Value { value, .. }) => {
                v1 == value
            },
            _ => false,
        }
    }
}

#[allow(dead_code)]
impl<T :Ord+Display> Bstree<T> {
    pub fn new() -> Self {
        Bstree { root: Box::new(Node::Nil) }
    }

    fn empty() -> Box<Node<T>> {
        Box::new(Node::Nil)
    }

    pub fn insert(&mut self, v :T) {
        if Node::Nil == *self.root {
            self.root = Box::new(Node::Value { value: v, left: Self::empty(), right: Self::empty() });
            return;
        };

        let mut b = &mut self.root;
        loop {
            let Node::Value { ref value, ref mut left, ref mut right } = **b else {
                *b = Box::new(Node::Value { value: v, left: Self::empty(), right: Self::empty() });
                break
            };

            if v < *value {
                b = left;
            } else if v > *value {
                b = right;
            } else {
                let l = std::mem::replace(left, Box::new(Node::Nil));
                let new_node = Node::Value {
                    value: v,
                    left: l,
                    right: Self::empty(),
                };
                *left = Box::new(new_node);
                break;
            }
        }
    }

    fn traverse_sorted(node :&Node<T>) {
        match *node {
            Node::Value { ref value, ref left, ref right } => {
                Self::traverse_sorted(left);
                println!("{} ", *value);
                Self::traverse_sorted(right);
            },
            _ => (),
        };
    }

    pub fn traverse(&self) {
        Self::traverse_sorted(&self.root);
    }
}

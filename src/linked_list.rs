use std::ops::Index;
use std::fmt::{Display, Write};

pub struct List<T> {
    head: Box<Node<T>>,
    tail: *mut Node<T>,
}

#[derive(PartialEq, Debug,Clone)]
pub enum Node<T> {
    Val { value: T, next: Box<Node<T>> },
    Nil,
}

impl<T :Copy+PartialEq> List<T> {
    pub fn new() -> List<T> {
        List {
            head: Box::new(Node::Nil),
            tail: std::ptr::null_mut(),
        }
    }

    fn empty() -> Box<Node<T>> {
        Box::new(Node::Nil)
    }

    pub fn insert(&mut self, v: T) {
        let old_head = std::mem::replace(&mut self.head, Self::empty());
        let is_first = matches!(*old_head, Node::Nil);
        let node = Node::Val {
            value: v,
            next: old_head,
        };
        *self.head = node;
        if is_first {
            self.tail = &mut *self.head as *mut Node<T>;
        }
    }

    pub fn delete_nth(&mut self, index: usize) -> Result<T, &str> {
        if index == 0 {
            let Node::Val { value, next } = std::mem::replace(&mut *self.head, Node::Nil) else {
                return Err("index out of range");
            };
            if matches!(*next, Node::Nil) {
                self.tail = std::ptr::null_mut();
            }
            self.head = next;
            return Ok(value);
        }

        let mut current = &mut self.head;
        let mut prev: Option<*mut Node<T>> = None;

        for _ in 0..index {
            prev = Some(&mut **current as *mut Node<T>);
            let Node::Val { next, .. } = &mut **current else {
                return Err("index out of range");
            };
            current = next;
        }

        let Node::Val { value, next } = std::mem::replace(&mut **current, Node::Nil) else {
            return Err("index out of range");
        };

        // Check if we're about to move the tail node
        let moving_tail = !self.tail.is_null() && 
            std::ptr::eq(self.tail, &*next as *const Node<T> as *mut Node<T>);
        **current = *next;

        // Update tail pointer if the tail node was moved
        if moving_tail {
            self.tail = &mut **current as *mut Node<T>;
        }

        if matches!(**current, Node::Nil) {
            self.tail = prev.unwrap_or(std::ptr::null_mut());
        }

        Ok(value)
    }

    pub fn insert_at(&mut self, v :T, index :usize) -> Result<(), &str> {
        if index == 0 {
            self.insert(v);
            return Ok(());
        }

        let mut current = &mut self.head;

        for _ in 0..index {
            let Node::Val { value:_, next } = &mut **current else {
                return Err("out of range");
            };

            current = next;
        }

        let n1 = std::mem::replace(&mut *current, Box::new(Node::Nil));
        let is_at_end = matches!(*n1, Node::Nil);
        let node = Node::Val { value: v, next:n1};
        **current = node;

        if is_at_end {
            self.tail = &mut **current as *mut Node<T>;
        }

        Ok(())
    }

    pub fn get_head(&self) -> Option<T> {
        let Node::Val { value, next:_ } = *self.head else {
            return None;
        };
        Some(value.clone())
    }

    pub fn get_tail(&self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                return None;
            }
            let Node::Val { value, next:_ } = *self.tail else {
                return None;
            };
            return Some(value)
        }
    }
}

impl<T :Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = &self.head;
        let mut s = String::new();

        while let Node::Val { ref value, ref next } = **node {
            write!(&mut s, "{} -> ", value)?;
            node = next;
        }
        write!(f, "[{}]", s.as_str())
    }
}

impl<T: PartialEq> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut node = &self.head;

        for _ in 0..index {
            let Node::Val { ref next, .. } = **node else {
                panic!("index out of bound");
            };

            node = next;
        }

        if let Node::Val { ref value, .. } = **node {
            return value;
        }

        panic!("index out of bound");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: List<i32> = List::new();
        // A new list should be Nil
        assert_eq!(*list.head, Node::Nil);
    }

    #[test]
    #[should_panic(expected = "index out of bound")]
    fn test_index_on_empty_list() {
        let list: List<i32> = List::new();
        let _ = list[0]; // should panic
    }

    #[test]
    fn test_insert_and_index() {
        let mut list = List::new();
        list.insert(10);
        list.insert(11);
        list.insert(12);

        assert_eq!(list[0], 12);
        assert_eq!(list[1], 11);
        assert_eq!(list[2], 10);
    }

    #[test]
    #[should_panic(expected = "index out of bound")]
    fn test_index_out_of_bounds() {
        let mut list = List::new();
        list.insert(10);
        list.insert(11);

        let _ = list[2]; // Index 2 is out of bounds for a list with 2 elements.
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut list = List::new();
        list.insert(10);
        list.insert(20);

        assert!(list.insert_at(5, 0).is_ok());
        assert_eq!(list[0], 5);
        assert_eq!(list[1], 20);
        assert_eq!(list[2], 10);
    }

    #[test]
    fn test_insert_at_middle() {
        let mut list = List::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert!(list.insert_at(25, 1).is_ok());
        assert_eq!(list[0], 30);
        assert_eq!(list[1], 25);
        assert_eq!(list[2], 20);
        assert_eq!(list[3], 10);
    }

    #[test]
    fn test_insert_at_end() {
        let mut list = List::new();
        list.insert(10);
        list.insert(20);

        assert!(list.insert_at(5, 2).is_ok());
        assert_eq!(list[0], 20);
        assert_eq!(list[1], 10);
        assert_eq!(list[2], 5);
    }

    #[test]
    fn test_insert_at_empty_list() {
        let mut list: List<i32> = List::new();

        assert!(list.insert_at(42, 0).is_ok());
        assert_eq!(list[0], 42);
    }

    #[test]
    fn test_insert_at_out_of_range() {
        let mut list = List::new();
        list.insert(10);
        list.insert(20);

        let result = list.insert_at(99, 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "out of range");
    }

    #[test]
    fn test_insert_at_out_of_range_empty_list() {
        let mut list: List<i32> = List::new();

        let result = list.insert_at(42, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "out of range");
    }
}

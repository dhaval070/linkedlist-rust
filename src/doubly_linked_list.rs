use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Display;
use anyhow::{Result, bail};

#[allow(dead_code)]
pub struct Dlist<T :Display> {
    head :Rc<RefCell<Node<T>>>,
    tail :Weak<RefCell<Node<T>>>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Node<T :Display> {
    Value { value:T, next :Rc<RefCell<Node<T>>>, prev :Weak<RefCell<Node<T>>> },
    Nil,
}

impl<T :PartialEq+Display> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (Node::Nil, Node::Nil) => true,
            (
                Node::Value { value :v1, next :n1, .. },
                Node::Value { value :v2, next :n2, .. },
            ) =>  v1 == v2 && n1 == n2,
            _ => false,
        }
    }

}

#[allow(dead_code)]
impl<T :PartialEq+Display> Dlist<T> {
    pub fn new() -> Dlist<T> {
        let node = Rc::new(RefCell::new(Node::Nil));
        Dlist{
            head: Rc::clone(&node),
            tail: Rc::downgrade(&node),
        }
    }

    fn empty() -> RefCell<Node<T>> {
        RefCell::new(Node::Nil)
    }

    pub fn insert(&mut self, value :T) {
        if *self.head.borrow() == Node::Nil {
            let h = Rc::new(RefCell::new(Node::Value{
                value :value,
                next: Rc::new(RefCell::new(Node::Nil)),
                prev: Weak::new(),
            }));
            self.head = Rc::clone(&h);
            self.tail = Rc::downgrade(&h);
            return;
        }

        let mut node = Rc::clone(&self.head);

        loop {
            let n :Rc<RefCell<Node<T>>>;

            if let Node::Value { value:_, ref mut next, prev:_ } = *node.borrow_mut() {
                if *next.borrow() != Node::Nil {
                    n = Rc::clone(next);
                } else {
                    let n = Rc::new(RefCell::new(Node::Value {
                        value: value,
                        next: Rc::new(Self::empty()),
                        prev: Rc::downgrade(&node),
                    }));
                    *next = Rc::clone(&n);
                    self.tail = Rc::downgrade(&n);
                    break;
                }
            } else {
                unreachable!()
            };
            node = n;
        }
    }

    pub fn print(&mut self) {
        let mut node = Rc::clone( &self.head);

        loop {
            let n = {
                let Node::Value { ref value, ref mut next, .. } = *node.borrow_mut() else {
                    break;
                };

                println!("{}", *value);
                Rc::clone(next)
            };
            node = n;
        }
    }

    pub fn print_reverse(&mut self) {
        let mut node = Weak::clone(&self.tail);
        let mut n : Option<Weak<RefCell<Node<T>>>>;

        loop {
            let Some(rc) = node.upgrade() else {
                break;
            };

            n = match *rc.borrow_mut() {
                Node::Value { ref value, next:_, ref prev } => {
                    println!("{}", *value);
                    Some(Weak::clone(prev))
                },
                _ => None,
            };

            let Some(wc) = n else {
                break;
            };
            node = wc;
        }
    }

    pub fn insert_at(&mut self, val :T, pos: u32) -> Result<()> {
        let mut node = Rc::clone(&self.head);

        for _ in 1..pos {
            let n = match &*node.borrow() {
                Node::Value { next, .. } => next.clone(),
                _ => bail!("index out of bound"),
            };
            node = n;
        }

        if *node.borrow() == *self.head.borrow() {
            let new_node = Rc::new(RefCell::new(Node::Value {
                value: val,
                next: Rc::clone(&self.head),
                prev: Weak::new(),
            }));

            if let Node::Value {  ref mut prev,.. } = *self.head.borrow_mut() {
                *prev = Rc::downgrade(&new_node.clone());
            };
            self.head = new_node;
            return Ok(());
        }

        let Node::Value { value:_, ref mut next,  prev:_ } = *node.borrow_mut() else {
            bail!("index out of bound");
        };

        let new_node = Rc::new(RefCell::new(Node::Value {
            value: val,
            next: Rc::clone(next),
            prev: Rc::downgrade(&node),
        }));

        if let Node::Value { ref mut prev, .. } = *next.borrow_mut() {
            *prev = Rc::downgrade(&new_node);
        } else {
            // next is Node::NIl, so make new node the tail
            self.tail = Rc::downgrade(&new_node);
        };

        *next = new_node.clone();
        Ok(())
    }

    pub fn delete_nth(&mut self, pos: u32) -> Result<()> {
        let mut node = Rc::clone(&self.head);
        for _ in 0..pos {
            let n = match *node.borrow() {
                Node::Value { ref next, .. } => Rc::clone(next),
                _ => bail!("index out of bound"),
            };
            node = n;
        }

        let Node::Value { ref next, ref prev, .. } = *node.borrow() else {
            bail!("index out of bound");
        };

        // Update prev node's next pointer, or update head if deleting first node
        if let Some(prev_rc) = prev.upgrade() {
            if let Node::Value { next: ref mut prev_next, .. } = *prev_rc.borrow_mut() {
                *prev_next = next.clone();
            }
        } else {
            self.head = next.clone();
        }

        // Update next node's prev pointer, or update tail if deleting last node
        if let Node::Value { prev: ref mut next_prev, .. } = *next.borrow_mut() {
            *next_prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: Dlist<i32> = Dlist::new();
        assert_eq!(*list.head.borrow(), Node::Nil);
    }

    #[test]
    fn test_insert_single() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);

        if let Node::Value { value, .. } = &*list.head.borrow() {
            assert_eq!(*value, 10);
        } else {
            panic!("Expected Value node");
        }
    }

    #[test]
    fn test_insert_multiple() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Check head is 10
        if let Node::Value { value, next, .. } = &*list.head.borrow() {
            assert_eq!(*value, 10);
            // Check second is 20
            if let Node::Value { value, next, .. } = &*next.borrow() {
                assert_eq!(*value, 20);
                // Check third is 30
                if let Node::Value { value, .. } = &*next.borrow() {
                    assert_eq!(*value, 30);
                } else {
                    panic!("Expected third Value node");
                }
            } else {
                panic!("Expected second Value node");
            }
        } else {
            panic!("Expected first Value node");
        }
    }

    #[test]
    fn test_tail_after_insert() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Tail should point to 30
        if let Some(rc) = list.tail.upgrade() {
            if let Node::Value { value, .. } = &*rc.borrow() {
                assert_eq!(*value, 30);
            } else {
                panic!("Expected tail to be Value node");
            }
        } else {
            panic!("Tail should be valid");
        }
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);

        list.insert_at(5, 0).unwrap();

        // Head should now be 5
        if let Node::Value { value, .. } = &*list.head.borrow() {
            assert_eq!(*value, 5);
        } else {
            panic!("Expected Value node at head");
        }
    }

    #[test]
    fn test_insert_at_middle() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Insert 25 at position 2 (between 20 and 30)
        list.insert_at(25, 2).unwrap();

        // Verify order: 10 -> 20 -> 25 -> 30
        if let Node::Value { value, next, .. } = &*list.head.borrow() {
            assert_eq!(*value, 10);
            if let Node::Value { value, next, .. } = &*next.borrow() {
                assert_eq!(*value, 20);
                if let Node::Value { value, next, .. } = &*next.borrow() {
                    assert_eq!(*value, 25);
                    if let Node::Value { value, .. } = &*next.borrow() {
                        assert_eq!(*value, 30);
                    }
                }
            }
        }
    }

    #[test]
    fn test_insert_at_end() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);

        // Insert at position 2 (end)
        list.insert_at(30, 2).unwrap();

        // Tail should be 30
        if let Some(rc) = list.tail.upgrade() {
            if let Node::Value { value, .. } = &*rc.borrow() {
                assert_eq!(*value, 30);
            } else {
                panic!("Expected tail to be Value node");
            }
        }
    }

    #[test]
    fn test_insert_at_out_of_bounds() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);

        let result = list.insert_at(99, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_nth_first() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        list.delete_nth(0).unwrap();

        // Head should now be 20
        if let Node::Value { value, .. } = &*list.head.borrow() {
            assert_eq!(*value, 20);
        } else {
            panic!("Expected Value node at head");
        }
    }

    #[test]
    fn test_delete_nth_middle() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        list.delete_nth(1).unwrap();

        // Order should be: 10 -> 30
        if let Node::Value { value, next, .. } = &*list.head.borrow() {
            assert_eq!(*value, 10);
            if let Node::Value { value, .. } = &*next.borrow() {
                assert_eq!(*value, 30);
            }
        }
    }

    #[test]
    fn test_delete_nth_last() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        list.delete_nth(2).unwrap();

        // Order should be: 10 -> 20
        if let Node::Value { value, next, .. } = &*list.head.borrow() {
            assert_eq!(*value, 10);
            if let Node::Value { value, next, .. } = &*next.borrow() {
                assert_eq!(*value, 20);
                assert_eq!(*next.borrow(), Node::Nil);
            }
        }
    }

    #[test]
    fn test_delete_nth_out_of_bounds() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);

        let result = list.delete_nth(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_prev_links() {
        let mut list: Dlist<i32> = Dlist::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Navigate to tail and check prev links back
        if let Some(tail_rc) = list.tail.upgrade() {
            if let Node::Value { value, prev, .. } = &*tail_rc.borrow() {
                assert_eq!(*value, 30);
                // Prev should be 20
                if let Some(prev_rc) = prev.upgrade() {
                    if let Node::Value { value, prev, .. } = &*prev_rc.borrow() {
                        assert_eq!(*value, 20);
                        // Prev should be 10
                        if let Some(prev_rc) = prev.upgrade() {
                            if let Node::Value { value, .. } = &*prev_rc.borrow() {
                                assert_eq!(*value, 10);
                            }
                        }
                    }
                }
            }
        }
    }
}

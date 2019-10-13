// First List version is a FILO stack
// public interface
pub struct List {
    head: Link, // actually TOP of stack, default private
}

// hidden data
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// new
impl List {
    pub fn new()->Self {
        return List {head: Link::Empty}; // return empty node: zero value, zero ref.
    }
}

use std::mem;
// push to head
impl List {
    pub fn push(&mut self, elem: i32) { // need modify, so take &mut
        let new_node = Box::new (Node { // create new Box pointer to Node
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        // assign new head of type Link, which take 1 out of 2 values [Empty/More]
        // More will need to link to old head first [see up]
        self.head = Link::More(new_node); 
    }
}

// pop from head
impl List {
    // return None or Some<i32>
    pub fn pop(&mut self) -> Option<i32> { // may or may not exist
        // take value
        match mem::replace(&mut self.head, Link::Empty) { // replace self.head and return old value
            Link::Empty => None, // comma is ok here
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()
    }
}

// drop to avoid recursive drop
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // self.head points to Empty
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// test
#[cfg(test)] // suppress warning for use statement
mod tests {
    // use crate::first::List; // equivalent to next line
    use super::List; // must introduce into this module
    // self and super for current and parent mod
    #[test]
    fn basics() {
        let mut list = List::new();

        // pop from empty list
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
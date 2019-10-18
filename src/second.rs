// Second List version 
// 1. Use Option as Link
// 2. Generics
// 3. peek, peek_mut
// 4.
// 5.
// public interface
pub struct List<T> {
    head: Link<T>, // actually TOP of stack, default private
}

// type alias
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T, // data type is the one templated...
    next: Link<T>,
}

// new
impl <T> List<T> { // NOTE: only need <T> here, but both need follow impl and List
    pub fn new()->Self {
        List {head: None}
    }

    pub fn push(&mut self, elem: T) { // need modify, so take &mut
        let new_node = Box::new (Node { // create new Box pointer to Node
            elem: elem,
            next: self.head.take(), // self.head = None
        });
        // assign new head of type Link, which take 1 out of 2 values [Empty/More]
        // More will need to link to old head first [see up]
        self.head = Some(new_node); 
    }

    // return None or Some<i32>
    pub fn pop(&mut self) -> Option<T> { // may or may not exist
        // take value
        self.head.take().map(|node| { // match option { None => None, Some(x) => Some(y) }
            self.head = node.next;
            node.elem
        })
        // map function of Option
        // pub fn map<U, F> (self, f: F) -> Option<U>
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { // as_ref() return Option<&T> which again has map
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| { // as_ref() return Option<&T> which again has map
            &mut node.elem
        })
    }
}

// drop to avoid recursive drop
impl <T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3)); // return reference to 3!
    assert_eq!(list.peek_mut(), Some(&mut 3));
    // list.peek_mut().map(|&mut value| { // pattern matching
    list.peek_mut().map(|value| {
        *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}
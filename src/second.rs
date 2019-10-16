// Second List version 
// 1. Use Option as Link
// 2. Generics
// 3. 
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
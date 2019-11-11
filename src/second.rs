// Second List version 
// 1. Use Option as Link [take & map]
// 2. Generics [location of <T> for impl]
// 3. peek [build test for peek_mut]
// 4. IntoIter
// 5. Iter [lifetime]
// 6. IterMut [&mut -> take / copy]
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

    // add peek
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
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

// iterator
// lists::second::IntoIter
pub struct IntoIter<T>(List<T>); // new type! see vec::IntoIter

impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self) // move
    }
}

impl <T> Iterator for IntoIter <T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, // Option to reference of a node!
}

impl <T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node)} // way to improve?
        // as_ref convert &Option<T> to Option<&T>, so node is &T, need one * to deref...
        // T is Box<T>, so we need second *
        // Then take & and return
    }
}

// implement next for Iter
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| { // ok to not use take, because & is copy
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// mutable iterator
pub struct IterMut <'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T; 
    fn next(&mut self) -> Option<Self::Item> {
        // return option to &mut
        self.next.take().map(|node| { // not as_mut
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut itr = list.into_iter();
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut itr = list.iter();
        assert_eq!(itr.next(), Some(&3));
        assert_eq!(itr.next(), Some(&2));
        assert_eq!(itr.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut itr = list.iter_mut();
 
        let mut _a = &mut List::new(); // ok to make "_a" identifier mutable
        _a.push(1);
        _a = &mut List::new(); // assign to another List

        assert_eq!(itr.next(), Some(&mut 3));
        assert_eq!(itr.next(), Some(&mut 2));
        assert_eq!(itr.next(), Some(&mut 1));
    }

    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|node|{
            *node = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}

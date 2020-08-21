pub struct List<T> {
    head : Link<T>,
}

struct Node<T> {
    elem : T,
    next : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head : None }
    }

    pub fn add(&mut self, elem : T) {
        let new_head = Box::new(Node {
            elem,
            next: self.head.take(),
        } );

        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map( |node| {
            self.head = node.next;
            node.elem
        } )
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        } )
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        } )
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> ListIter<T> {
        let p = self.head.as_ref().map(|boxed_node| {
            &**boxed_node
        } );
        ListIter{ p }
    }

    pub fn iter_mut(&mut self) -> ListIterMut<T> {
        let p = self.head.as_mut().map(|boxed_node| {
            &mut **boxed_node
        } );
        ListIterMut{ p }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        loop {
            match cur {
                None => break,
                Some(mut p) => {
                    cur = p.next.take();
                }
            }
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct ListIter<'a, T> {
    p : Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.p {
            None => None,
            Some(node) => {
                self.p = match &node.next {
                    None => None,
                    Some(box_next_p) => Some(&*box_next_p),
                };
                Some(&node.elem)
            }
        }
    }
}

pub struct ListIterMut<'a, T> {
    p : Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.p.take() {
            None => None,
            Some(node) => {
                self.p = match node.next.as_mut() {
                    None => None,
                    Some(box_next_p) => Some(&mut **box_next_p),
                };
                Some(&mut node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let list: List<i32> = List::new();
        assert_eq!(None, list.peek());
    }

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(None, list.peek());
        list.add(0);
        assert_eq!(Some(&mut 0), list.peek_mut());
        list.add(1);
        assert_eq!(Some(&1), list.peek());
        list.peek_mut().map( |elem| {
            *elem += 100;
        } );

        list.add(-2);
        assert_eq!(Some(&-2), list.peek());
        assert_eq!(Some(-2), list.pop());
        assert_eq!(Some(&101), list.peek());
        list.add(-1);
        assert_eq!(Some(&-1), list.peek());
        list.add(2);
        assert_eq!(Some(&2), list.peek());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(&-1), list.peek());
        assert_eq!(Some(-1), list.pop());
        assert_eq!(Some(&101), list.peek());
        list.add(3);
        assert_eq!(Some(&3), list.peek());
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(&101), list.peek());
        assert_eq!(Some(101), list.pop());
        assert_eq!(Some(&0), list.peek());
        assert_eq!(Some(0), list.pop());
        assert_eq!(None, list.peek());
        assert_eq!(None, list.pop());
        assert_eq!(None, list.peek_mut());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.into_iter();
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter_mut();
        assert_eq!(Some(&mut 3), iter.next());
        let mid = iter.next();
        assert_eq!(mid, Some(&mut 2));
        *mid.unwrap() = 5;
        // assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }
}

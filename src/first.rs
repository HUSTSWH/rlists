use std::mem;

pub struct List {
    head : Link,
}

struct Node {
    elem : i32,
    next : Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {

    pub fn new() -> Self {
        Self { head : Link::Empty }
    }

    pub fn add(&mut self, elem : i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        } );

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        /*
        match &mut self.head {
            Link::Empty => None,
            Link::More(p) => {
                let elem = p.elem;
                let rest_list = mem::replace(&mut p.next, Link::Empty);
                let _front_node = mem::replace(&mut self.head, rest_list);
                Some(elem)
            }
        }
        */
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(p) => {
                self.head = p.next;
                Some(p.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let list = List::new();
        match list.head {
            Link::Empty => (),
            Link::More(_) => panic!(),
        }
        // assert_eq!(list.head, Link::Empty);
    }

    #[test]
    fn basics() {
        let mut list = List::new();
        list.add(0);
        list.add(1);
        list.add(-2);
        assert_eq!(Some(-2), list.pop());
        list.add(-1);
        list.add(2);
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(-1), list.pop());
        list.add(3);
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(Some(0), list.pop());
        assert_eq!(None, list.pop());
    }
}

use std::boxed::Box;
use std::mem::replace;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List::<T> { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(box_node) => {
                let node = *box_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl <T> Drop for List<T> {

    fn drop(&mut self) {
        let mut current = replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current {
            current = replace(&mut boxed_node.next, Link::Empty);
        }
    }

}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn base() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);

        list.push(20);
        list.push(30);
        list.push(40);

        assert_eq!(list.pop(), Some(40));
        assert_eq!(list.pop(), Some(30));

        list.push(345);

        assert_eq!(list.pop(), Some(345));
        assert_eq!(list.pop(), Some(20));
    }
}




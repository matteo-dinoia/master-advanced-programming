use std::mem;

pub(crate) fn test_all() {
    let mut list = List::new(None);

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

struct Node<T>{
    el: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(el: T, next: Link<T>) -> Self {
        Self { el, next}
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct List<T>{
    head: Option<Box<Node<T>>>
}


impl<T> List<T> {
    pub fn new(head: Option<Box<Node<T>>>) -> Self {
        Self{head}
    }

    pub fn push(&mut self, el: T) {
        self.head = Some(Box::new(Node::new(el,
                                            self.head.take()
        )));
    }
}

impl<T: Clone> List<T> {

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.el
        })
    }
}
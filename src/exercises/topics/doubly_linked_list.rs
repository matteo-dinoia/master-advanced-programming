use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;


pub fn test_all() {
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    debug_assert_eq!(list.size, 3);
    println!("{}", list.size);
    list.print_list();
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 1);
    println!("{}", list.head.clone().unwrap().borrow().element);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 3);
    println!("{}", list.tail.clone().unwrap().borrow().element);
}
type Link<T> =  Option<Rc<RefCell<Node<T>>>>;

struct Node<T>{
    element: T,
    prev: Link<T>,
    next: Link<T>
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self { element, prev: None, next: None }
    }

    fn new_link(element: T, prev: Link<T>, next: Link<T>) -> Link<T>{
        Some(Rc::new(RefCell::new(Node{
            element, prev, next
        })))
    }
}

struct List<T>{
    size: usize,
    head: Link<T>,
    tail: Link<T>
}



impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl<T: Display> Display for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl<T: Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.element)
    }
}

impl<T: PartialEq> PartialEq for List<T>{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size { return false; }

        let mut c1 = self.head.clone();
        let mut c2 = self.head.clone();

        while c1.is_some() && c2.is_some(){
            let node1 = c1.clone().unwrap();
            let node2 = c2.clone().unwrap();
            if node1.borrow().element != node2.borrow().element {
                return false;
            }

            c1 = node1.borrow().next.clone();
            c2 = node2.borrow().next.clone();
        }

        true
    }
}

impl<T: Debug> Debug for List<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut c1 = self.head.clone();
        while c1.is_some(){
            let node1 = c1.clone().unwrap();
            write!(f, "{:?}", node1.borrow())?;
            c1 = node1.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T: Display> List<T>{
    fn print_list(&self){
        let mut c1 = self.head.clone();
        while c1.is_some(){
            let node1 = c1.clone().unwrap();
            println!("{}", node1.borrow());
            c1 = node1.borrow().next.clone();
        }
    }
}

impl<T> List<T>{
    pub fn new() -> Self {
        Self { size: 0, head: None, tail: None }
    }

    fn push(&mut self, element: T){
        let newNodeLink = Node::new_link(element, 
                 None, 
                 self.head.clone());
        self.size += 1;

        if self.head.is_none() {
            self.head = newNodeLink.clone();
            self.tail = newNodeLink.clone();
            return;
        }
        
        self.head.clone().unwrap().borrow_mut().prev = newNodeLink.clone();
        self.head = newNodeLink;
        
    }

    fn push_back(&mut self, element: T){
        let newNodeLink = Node::new_link(element, 
                     self.tail.clone(),
                     None);
        self.size += 1;
        
        if self.tail.is_none() {
            self.head = newNodeLink.clone();
            self.tail = newNodeLink.clone();
            return;
        }

        self.tail.clone().unwrap().borrow_mut().next = newNodeLink.clone();
        self.tail = newNodeLink;
    }
}

impl<T: Clone> List<T>{
    fn pop(&mut self) -> Option<T>{
        let head = self.head.clone()?;
        
        if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else {
            self.head = head.borrow().next.clone();
            self.head.clone()?.borrow_mut().prev = None;
        }
        

        self.size -= 1;
        Some(head.clone().borrow().element.clone())
    }

    fn pop_back(&mut self) -> Option<T>{
        let tail = self.tail.clone()?;

        if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else {
            self.tail = tail.borrow().prev.clone();
            self.tail.clone()?.borrow_mut().next = None;
        }
        
        self.size -= 1;
        Some(tail.clone().borrow().element.clone())
    }
}


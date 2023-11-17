use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(i: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: i,
            next: None,
            prev: None,
        }))
    }
}

type Pointer = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct LinkedList {
    head: Pointer,
    tail: Pointer,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, i: i32) {
        let node = Node::new(i);
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(head.clone());
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }

    fn push_back(&mut self, i: i32) {
        let node = Node::new(i);
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(tail.clone());
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("The list is empty");
        }
        else {
            self.head.take().map(|old| {
                match old.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail.take();
                        println!("List is empty after removal");
                        None
                    }
                }
            });
        }
    }
    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("the list is empty");
        }
        else {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head.take();
                        panic!("list is empty after removal");
                        None
                    }
                }
            }
            );
        }
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("[]");
            return;
        } else {
            let mut traversal = self.head.clone();
            while !traversal.is_none() {
                println!("{}",traversal.as_ref().unwrap().borrow().data);
                traversal = traversal.unwrap().borrow().next.clone();
            }
        }
    }
    // fn remove_front(&mut self) {

    //     match self.head.take() {
    //         Some(head  ) => {
    //             match head.borrow_mut().next.take() {
    //                 Some(new_head) =>
    //                 {
    //                     new_head.borrow_mut().prev=None;
    //                     self.head=Some(new_head);
    //                 }
    //                 None => {
                        
    //                 }
    //             }
    //         }
    //         None => {

    //         }
    //     }
    // }

    // fn remove_back(&mut self) {

    //     match self.tail.take() {
    //         Some(tail) => {
    //             match tail.borrow_mut().prev.take() {
    //                 Some(new_tail) => {
    //                     new_tail.borrow_mut().next=None;
    //                     self.tail = Some(new_tail)
    //                 }
    //                 None => {

    //                 }
    //             }

    //         }
    //         None => {

    //         }
    //     }

    
    // }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    // list.print_forward();
    // println!("{:?}",list); it wont work as debug impl is like that we can do that using weak pointer
    list.print();
}

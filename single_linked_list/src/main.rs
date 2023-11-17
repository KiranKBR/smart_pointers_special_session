
type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug + std::marker::Copy>  {
    head : Pointer<T>
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    data : T,
    next : Pointer<T>
}

impl<T: std::fmt::Debug + std::marker::Copy>  LinkedList<T> {
    fn New() -> LinkedList<T> {
        LinkedList { head: None }
    }

    // fn add(&mut self,element:i32) {
    //     let mut new_node = Node {
    //         data:element,
    //         next:None,
    //     };
    //     match self.head {
    //         Some(node) => {
    //             new_node.next=Some(node);
    //             self.head = Some(Box::new(new_node));
    //         }

    //         None => {
    //             self.head = Some(Box::new( new_node));
    //         }
    //     }
    // }

    fn add(&mut self,data:T) {
        let previous_head = self.head.take();
        let new_node = Node {
            data:data,
            next:previous_head,
        };
        self.head = Some(Box::new(new_node));
    }

    fn remove_at_beginning(&mut self) -> Option<T> {
        let head = self.head.take();//it will take the copy
        match head {
            Some(head) =>{
                self.head = head.next;
                Some(head.data)
            }

            None => None,
        }

    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node)  => Some(node.data),
            None => None,
        }
    }

    fn print(&self) {
        let mut iter = &self.head;
        while true {
            match iter {
                Some(node) => {
                    println!("{:?}",node.data);
                    iter = &node.next;
                }
                None => break,
            }
        }

    }
}

fn main() {
    
    let head = LinkedList { head : Some(Box:: new (Node { data: 9, next:None}))};
    println!("{head:?}");

    let node = Node {
        data : 6,
        next : None
    };
    let head = LinkedList { head : Some (Box::new(Node {
        data:9,
        next : Some(Box::new(node)),
    }))};

    println!("{head:?}");
    println!();
    // println!("{node:?}");here it will give u error as it has been moved there

    let mut linked_list = LinkedList::New();
    linked_list.add(3);
    linked_list.add(6);
    linked_list.add(9);
    linked_list.add(12);
    linked_list.add(15);
    linked_list.add(18);
    linked_list.add(21);
    // linked_list.remove_at_beginning();
    println!("{:?}",linked_list);//elements printed in reversed order as we insert at beginning
    println!("element at peek {:?}",linked_list.peek());
    linked_list.print();
}

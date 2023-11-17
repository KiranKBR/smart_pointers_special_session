use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32,Rc<List>),
    Nil,
}

fn make_rc() -> Rc<String> {
    let s1: Rc<String>= Rc::new("Hello".into());
    println!("Count when pointer is created {}",Rc::strong_count(&s1));

    // let s2 = s1.clone();
    let s2 = Rc::clone(&s1);
    println!("Count after clone for s1 {} and s2 {}",Rc::strong_count(&s1),Rc::strong_count(&s2));

    s2
}

use crate::List::{Cons,Nil};
//Box pointer wont work for graphs,trees like data structures
fn main() {
    let a = Rc::new(Cons(1,Rc::new(Cons(2,Rc::new(Nil)))));
    println!("strong count counter after creating a {}",Rc::strong_count(&a));

    let b = Rc::new(Cons(3,Rc::clone(&a)));
    println!("strong count counter after creating b {} count of b {}",Rc::strong_count(&a),Rc::strong_count(&b));

    let c = Rc::new(Cons(4,Rc::clone(&a)));
    println!("strong count counter after creating b {} count of b {} count of c {}",Rc::strong_count(&a),Rc::strong_count(&b),Rc::strong_count(&c));

    println!();
    let s = make_rc();
    println!("strong count after function call {}",Rc::strong_count(&s));
}
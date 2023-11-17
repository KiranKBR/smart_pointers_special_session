use std::cell::{RefCell, RefMut};
use std::rc::Rc;

//two things in refcell
    // 1.Borrow rules checked at runtime
    // 2.Internal Mutability
fn main() {
    // let mut a =5;
    // let b = &a;
    // let c = &a;
    // let d = &mut a;

    // println!("{}",b);

    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    drop(b);
    drop(c);
    
    let d = a.borrow_mut();
    // drop(d);

    // println!("{}",b);

    let x = RefCell::new(3);
    let mut y:RefMut<i32> = x.borrow_mut();
    // *y=9;
    drop(y);
    println!("{:?}",x);
    //if print the x without droping y it will say borrowed   RefCell { value: <borrowed> }

    //lets combine Rc and RefCell
//     let a = Rc::new(RefCell::new(10));
//     let b = a.clone();
//     *b.borrow_mut()=9;
// // drop(b);
//     println!("Strong count of a {} and b {} ",Rc::strong_count(&a),Rc::strong_count(&b));

//     println!("{:?}",a);

}

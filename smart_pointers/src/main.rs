//stack reference vs heap reference(Box)

// #[derive(Debug)]
// enum List {
//     // Cons(i32,List)
//     Cons(i32,Box<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32,Option<Box<List>>),
}
 use List::Cons;
fn main() {
    
    let mut stack_var = 4;
    let stack_ref = &stack_var;

    let heap_var = Box::new(stack_var);//we can put stack ref also

    stack_var =6;
    println!("{},{}",stack_var,heap_var);

    let stack_ref = &stack_var;
    let heap_stack_ref = Box::new(stack_ref);
    println!("{},{}",stack_var,heap_stack_ref);
    let stack_ref = &stack_var;
    
    let heap_var = Box::new(stack_var);//we can put stack ref also
    
    stack_var =6;
    println!("{},{}",stack_var,heap_var);
    
    let stack_ref = &stack_var;
    let heap_stack_ref = Box::new(stack_ref);
    println!("{},{}",stack_var,heap_stack_ref);

    // let list = Cons(6,Box::new(Cons(3,Box::new(Cons(9,Box::new(Nil))))));//here we are allocating heap space for nil varian talso
    // //to get rid of this we can use Option enum
    // println!("{:?}",list)

    let list = Cons(6,Some(Box::new(Cons(3,Some(Box::new(Cons(9,None)))))));//here we are allocating heap space for nil varian talso
    //to get rid of this we can use Option enum
    println!("{:?}",list)
}
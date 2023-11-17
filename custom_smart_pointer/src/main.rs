//Deref trait
// trait Deref {
//     type Target:type_name;
//     fn deref(&self) -> &self::Target;
// }

//Drop trsit
// pub trait Drop {
//     fn drop(&mut self) ;
// }

struct MySmartPointer {
    value : i32,
}

impl MySmartPointer{
    fn new(value:i32) -> MySmartPointer {
        MySmartPointer { value }
    }
}

impl Deref for MySmartPointer {
    type Target= i32;
    fn deref(&self) -> &i32 {   
        &self.value
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("drop has been called to MySmartPointer object {:?} ",self.value);
    }
}
fn main() {
    // println!("Hello, world!");
    //smart pointers implements deref trait and drop trait
    //without explicit calling of drop also it has been executed
    //we can call drop explicitly also by drop

    let a=50;
    let b= &a;
    println!("{}",a);
    println!("{}",*b);

    let mysptr1 = MySmartPointer::new(3);
    let mysptr2 = MySmartPointer::new(a);
    let mysptr3 = MySmartPointer::new(7);

    println!("{}",a==*mysptr2);//*(mysptr1.deref()) 
    // mysptr1.drop();it cant be called 
    // drop(mysptr1);

    //result without explicit calling
    // drop has been called to MySmartPointer object 7 
    // drop has been called to MySmartPointer object 50 
    // drop has been called to MySmartPointer object 3 

    //result with explcit calling
    // drop(mysptr1);
    // drop has been called to MySmartPointer object 3 
    // drop has been called to MySmartPointer object 7 
    // drop has been called to MySmartPointer object 50
}

use std::ops::Deref;


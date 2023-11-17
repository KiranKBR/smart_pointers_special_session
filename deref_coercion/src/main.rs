//Deref trait
// trait Deref {
//     type Target:type_name;
//     fn deref(&self) -> &self::Target;
// }

//Drop trsit
// pub trait Drop {
//     fn drop(&mut self) ;
// }
use std::ops::Deref;

struct MySmartPointer<T: std::fmt::Debug> {
    value : T,
}

impl<T: std::fmt::Debug> MySmartPointer<T>{
    fn new(value:T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target= T;
    fn deref(&self) -> &T {   
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("drop has been called to MySmartPointer object {:?} ",self.value);
    }
}

fn show_data(str:&str) {
    println!("the string is {}",str);
}
fn main() {
    // println!("Hello, world!");
    //smart pointers implements deref trait and drop trait
    //without explicit calling of drop also it has been executed
    //we can call drop explicitly also by drop

    // let a=50;
    // let b= &a;
    // println!("{}",a);
    // println!("{}",*b);

    // let mysptr1 = MySmartPointer::new(3);
    // let mysptr2 = MySmartPointer::new(a);
    // let mysptr3 = MySmartPointer::new(7);

    // println!("{}",a==*mysptr2);//*(mysptr1.deref()) 
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

    let sptr = MySmartPointer::new("Kiran");
    show_data(&sptr);//&MySmartPointer -> &String -> &str

    let sptr2 = MySmartPointer::new(9);
    // show_data(&sptr2);

    let sptr3 = MySmartPointer::new(vec![3,6,9]);
    for i in &*sptr3 {
        print!("{:?} ",i);
    }
    println!();
}




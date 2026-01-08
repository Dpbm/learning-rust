use std::fmt::Debug;
use std::ops::Deref;
use std::any::type_name;

fn get_typename<T: ?Sized>(_:&T) -> &'static str{
    type_name::<T>()
}


#[derive(Debug)] struct Test{
    pub name: String,
    pub age: u8
}


#[derive(Debug)]
struct CustomBox<T: Debug>(T); // a tuple struct

impl<T:Debug> CustomBox<T>{
    fn new(inp:T) -> CustomBox<T> {
        CustomBox(inp)
    }
}

//impl<T: Debug> Deref for CustomBox<T>{
//    type Target = T;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0        
//    }
//}

impl Deref for CustomBox<i32>{
    type Target = i32;

    fn deref(&self) -> &i32 {
        println!("Inside i32");
        &self.0
    }
}

impl Deref for CustomBox<String>{
    type Target = str;

    fn deref(&self) -> &str {
        println!("Inside string");
        &self.0.as_str()
    }

}

impl<T: Debug> Drop for CustomBox<T>{
    
    fn drop(&mut self){
        println!("Dropping {:?}", self);
    }

}


fn main(){
    let box1 = Box::new(10);

    println!("Default box 1: {}", box1);
    println!("Default box 1 dereference: {}", *box1);

    let box2 = Box::new(
        Test{
            name: String::from("test"),
            age: 10
        }
    );

    println!("Default Box 2: {:?}", box2);
    println!("Default Box 2 name: {}", box2.name);
    println!("Default Box 2 age: {}", box2.age);

    println!("Default Box 2 dereference: {:?}", *box2);
    println!("------------------------");

    let custom_box = CustomBox::new(32);
    println!("Testing custombox!");


    let custom_box_2 = CustomBox::new(30);
    std::mem::drop(custom_box_2);
    println!("Testing custombox drop before (30)!");
    println!("------------------------");

    //println!("Failed deref: {}", *custom_box); // if not implement Deref it won't work
    println!("Deref custom box: {}", *custom_box);
    println!("Type for Deref i32: {}", get_typename(&*custom_box));

    let custom_box_3_str = CustomBox::new(String::from("OOOOO"));
    //println!("Deref custom box 3 str: {}", *custom_box_3_str); // can't know the size during
    //compile time since it returns a str instead of String
    println!("Type for Deref String: {}", get_typename(&*custom_box_3_str));





}

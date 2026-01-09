use std::rc::Rc;
use std::cell::RefCell;

fn main(){
    let data = Rc::new(String::from("test"));
    let first = Rc::clone(&data);
    let second = Rc::clone(&data);

    let third = Rc::downgrade(&data);
    println!("strong references: {}", Rc::strong_count(&data));
    println!("weak references: {}", Rc::weak_count(&data));

    std::mem::drop(first);
    println!("strong references: {}", Rc::strong_count(&data));
    std::mem::drop(second);
    println!("strong references: {}", Rc::strong_count(&data));
    std::mem::drop(data);

    println!("---------------------------------------");
    
    let data = Rc::new(RefCell::new(String::from("test")));
    println!("new data: {}", data.borrow());

    data.borrow_mut().push_str(" new string data");
    println!("new data: {}", data.borrow());
}

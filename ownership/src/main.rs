fn main() {
    let a = 10;
    let b = get_and_return(a);
    println!("a:{}\nb:{}", a, b);
    
    let c = String::from("hello, world!");
    string_ownership(c.clone());
    println!("older string: {}",c);
}

fn get_and_return(a:u8) -> u8{
    a
}

fn string_ownership(a:String){
    println!("Inside string: {}",a);
}

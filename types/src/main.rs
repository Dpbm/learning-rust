fn main() {
    let chr : char = '🦛';
    let int : u8 = 10;
    let float: f32 = 30.9;

    let tuple: (char, u8, f32) = (chr, int, float);
    let list: [u8;5] = [5;5];

    println!("chr: {}", chr);
    println!("int: {}", int);
    println!("float: {}", float);
    println!("");
    println!("tuple 0: {}", tuple.0);
    println!("tuple 1: {}", tuple.1);
    println!("tuple 2: {}", tuple.2);
    println!("");
    for (i, value) in list.iter().enumerate(){
        println!("list {}: {}", i, value);
    }
}

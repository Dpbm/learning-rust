fn main() {
    println!("5 --> {}", five());
    println!("3 + 1 --> {}", plus_one(3));
}

fn five() -> u8{
    5
}

fn plus_one(number: u8) -> u8{
    number + 1
}

fn five() -> i32 {
    5
}

fn six_plus_x(x: i32) -> i32 {
    return x + 6;
}

fn main() {
    let x = five();
    
    println!("The value of x is: {x}");

    let x = six_plus_x(6);

    println!("The value of x is now {x}");
}

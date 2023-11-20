use std::io;

fn main() {
    println!("Enter a Fahrenheit to convert to Celcius...");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit).expect("Could not read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number");

    let celcius = (fahrenheit - 32.0) * 5.0/9.0;

    println!("Celcius: {celcius}");
}

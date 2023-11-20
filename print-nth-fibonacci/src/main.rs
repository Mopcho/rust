use std::io;

fn main() {
    println!("Select a fibonacci...");

    let mut fibonacci = String::new();

    io::stdin().read_line(&mut fibonacci).expect("Failed to read line");

    let fibonacci: u32 = fibonacci.trim().parse().expect("Expected a valid positive  number");

    let mut counter = 0;
    let mut sum = 0;
    let mut previous_sum = 0;

    let result = loop {
        if fibonacci == 0 {
            break 0;
        }

        if counter == 0 || counter == 1 {
            sum = 1;
            previous_sum = 1;
        } else {
            let sum_holder = sum;

            sum += previous_sum;
            previous_sum = sum_holder;
        }

        counter += 1;

        if counter == fibonacci {
            break sum;
        }
    };

    println!("Result is {result}");
}

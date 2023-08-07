use std::io;

fn main() {
    let mut n = String::new();
    println!("This program calculates the probability that two or more people in a group size of n share the same birthday");
    println!("Enter the group size (between 1 and 365)");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n
        .trim()
        .parse()
        .expect("Please type a number!");

    if n < 2 || n > 365 {
        println!("Please enter a group size between 2 and 365.");
        return;
    } 

    let (probability, number) = birthday_prob(n, 0.50);

    println!("Probability that two or more people share the same birthday in a group of size {} is: {}", n, probability);
    if number != -1 {
        println!("Threshold of 0.50 was met at group size: {}", number);
    }
}

fn birthday_prob(n: i32, threshold: f32) -> (f32, i32) {
    let mut x: f32 = 1.0;
    for number in 1..=n {
        x *= (365.0 - number as f32) / 365.0;
        if x < (1.0 - threshold) {
            return (1.0 - x, number);
        }
    }
    (1.0 - x, -1)
}

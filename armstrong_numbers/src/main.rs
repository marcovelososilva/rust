use std::io;

fn main() {
    println!("ARMSTRONG NUMBER?");
    let input = read_number_from_console();
    let digits: Vec<_> = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let mut total = 0;
    let digits_leng: u32 = digits.len() as u32;

    for dig in digits.iter() {
        total += i32::pow(*dig as i32, digits_leng);
    }
    if total == input {
        println!("IT IS A ARMSTRONG NUMBER!");
    } else {
        println!("IT IS NOT A ARMSTRONG NUMBER!");
    }
}

fn read_number_from_console() -> i32 {
    loop {
        println!("Please input your number.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn main() {
    println!("ARMSTRONG NUMBERS BETWEEEN 1 and 1000");

    for numb in 1..1000 {
        let digits: Vec<_> = numb
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        let mut total = 0;
        let digits_leng: u32 = digits.len() as u32;

        for dig in digits {
            total += i32::pow(dig as i32, digits_leng);
        }

        if numb == total {
            println!("{}", numb);
        }
    }
}

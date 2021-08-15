use std::io;

fn main() {
    println!("ROMAN NUMERAL CONVERTION");
    let num = read_number_from_console();
    let roman = get_roman(num, "".to_string());
    println!("ROMAN NUMERAL = {}", roman);
}

fn get_roman(num: i32, name: String) -> String {
    if num == 0 {
        return name;
    };
    if num > 999 {
        let m_num: i32 = num / 1000;
        let letter: String = get_letter(m_num, "M".to_string(), ".".to_string(), ".".to_string());
        return get_roman(num % 1000, [name, letter].concat());
    } else if num > 99 {
        let c_num: i32 = num / 100;
        let letter: String = get_letter(c_num, "C".to_string(), "D".to_string(), "M".to_string());
        return get_roman(num % 100, [name, letter].concat());
    } else if num > 9 {
        let d_num: i32 = num / 10;
        let letter: String = get_letter(d_num, "X".to_string(), "L".to_string(), "C".to_string());
        return get_roman(num % 10, [name, letter].concat());
    } else {
        let letter: String = get_letter(num, "I".to_string(), "V".to_string(), "X".to_string());
        return get_roman(0, [name, letter].concat());
    }
}

fn get_letter(num: i32, c_one: String, c_five: String, c_ten: String) -> String {
    if num == 1 {
        return c_one;
    } else if num == 2 {
        let one2 = c_one.clone();
        return [c_one, one2].concat();
    } else if num == 3 {
        let one2 = c_one.clone();
        let one3 = c_one.clone();
        return [c_one, one2, one3].concat();
    } else if num == 4 {
        return [c_one, c_five].concat();
    } else if num == 5 {
        return [c_five].concat();
    } else if num == 6 {
        return [c_five, c_one].concat();
    } else if num == 7 {
        let one2 = c_one.clone();
        return [c_five, c_one, one2].concat();
    } else if num == 8 {
        let one2 = c_one.clone();
        let one3 = c_one.clone();
        return [c_five, c_one, one2, one3].concat();
    } else {
        return [c_one, c_ten].concat();
    }
}

fn read_number_from_console() -> i32 {
    loop {
        println!("Please input your number. from 1 to 3999");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                if num > 3999 || num < 1 {
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => continue,
        };
    }
}

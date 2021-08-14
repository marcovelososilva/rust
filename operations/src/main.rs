fn main() {
    //VARIABLES mutability
    //NOT mut
    let not_mut = 5;
    println!("NOT MUTABLE: {}", not_mut);
    let mut yes_mut = 6;
    println!("MUTABLE: {}", yes_mut);
    yes_mut = 10;
    println!("MUTABLE: {}", yes_mut);
    //CONSTANT
    const MAX_THINGS: u32 = 1000;
    println!("CONSTANT: {}", MAX_THINGS);

    //SHADOWING
    let x = 5;
    let x = x + 1; //6=5+1
    let x = x * 2; //12=6*2
    println!("The value of x is: {}", x); //x=12

    let spaces = "        ";
    let spaces = spaces.len();
    println!("SPACES: {}", spaces);
    // addition
    let sum = 5 + 10;
    println!("SUM: {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("DIFFERENCE: {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("PRODUCT: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("QUOTIENT: {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("REMAINDER: {}", remainder);

    //BOOL
    let t = true;
    println!("T: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("F: {}", f);
    //CHAR
    let c = 'z';
    println!("CHAR C: {}", c);
    let z = 'Ƶ';
    println!("CHAR Z: {}", z);
    let heart_eyed_cat = '😻';
    println!("CHAR HEART_EYED_CAT: {}", heart_eyed_cat);
    //TUPLE
    //ex1
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    //ARRAY
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; //same as "let a = [3, 3, 3, 3, 3];""
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    //acessing
    println!("array a: 0-> {}", a[0]);
    println!("array b: 1-> {}", b[1]);
    println!("array c: 3-> {}", c[3]);
    println!("array months: 5-> {}", months[5]);

    another_function();

    another_function_two(5, 6);

    let x = function_with_return(125);
    println!("return function: {}", x);

    //IF STATEMENT
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //LOOPS
    let mut counter = 1;
    loop {
        counter += 1;
        println!("LOOP until 10 counter = {}", counter);
        if counter == 10 {
            break;
        }
    }

    //WHILE
    while counter != 0 {
        println!("WHILE loop counter= {}", counter);
        counter -= 1;
    }

    //FOR
    let my_collec = ["one", "two", "three", "four", "five", "six"];
    for item in my_collec.iter() {
        println!("{}", item);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_two(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_with_return(x: i32) -> i32 {
    x + 1
}

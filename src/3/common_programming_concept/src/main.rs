const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Const {}", THREE_HOURS_IN_SECONDS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; //(u8 only)
    println!("{} {} {} {} {}", decimal, hex, octal, binary, byte);

    let char = 'c';
    let char1 = 'þ';
    let string = "string";
    println!("{} {} {}", char, char1, string);
    tuples();
    arrays();
    let result = print_labeled_measurement(5, 'h');
    println!("{}", result);

    if_statement();
    breaking_with_label();
    while_loop();
    for_loop();
    reverse_for_loop();
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = [3; 5];
    println!("{}", a[3]);
}

fn tuples() {
    let _: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    println!("{}", tup.0);
    let (_, y, _) = tup;
    println!("{}", y);
}

fn reverse_for_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn if_statement() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn breaking_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn print_labeled_measurement(value: i32, unit_label: char) -> i32 {
    println!("The measurement is: {}{}", value, unit_label);
    7
}

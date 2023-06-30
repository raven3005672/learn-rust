fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main_bool() {
    let number = 3;

    if number {
        // 报错：if 条件必须是 bool 类型
        println!("number was three");
    }
}

fn main_if_else() {
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
}

fn main_if_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn main_if_expression_2() {
    let condition = true;
    let number = if condition { 5 } else { "six" }; // 报错：if 表达式必须是相同类型

    println!("The value of number is: {}", number);
}

fn main_loop() {
    loop {
        println!("again!");
    }
}

fn main_loop_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // 20
}

fn main_loop_label() {
    'outer: loop {
        'inner: loop {
            println!("Hello, world!");
            break 'outer;
        }
    }
}

fn main_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn main_for_in() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn main_for_in_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

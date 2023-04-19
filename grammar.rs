use std::io;
use rand::Rng;  // 随机数
use std::cmp::Ordering;  // 比较

let x = 123;  // immutable
let mut y = 456;  // mutable

i32 // signed 32-bit integer  // 默认
u32 // unsigned 32-bit integer
i64 // signed 64-bit integer

&x  // immutable reference

println!("xxx = {}", x);  // {} 占位符}

1..=10; // 1 ~ 10

// match value {
//     pattern => expression,
//     pattern => expression,
//     pattern => expression,
// }
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}

// 无限循环
loop {
    println!("again!");
    break;  // 跳出循环
    continue;  // 跳过本次循环
}
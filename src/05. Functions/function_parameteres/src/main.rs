fn main() {
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
}

fn say_hello() {
    println!("hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}

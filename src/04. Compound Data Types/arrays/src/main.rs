fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letters = letters[0];
    println!("first_letters is {}", first_letters);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]);
}

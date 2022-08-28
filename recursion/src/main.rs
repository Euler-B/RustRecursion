fn factorial(n: u128) -> u128 {
    if n < 2 {
        1
    } else {
        n * factorial(n -1)
    }
}



fn main() {
    println!("{}", factorial(31 as u128));
}

fn main() {
    println!("Hello, world!");
    let n = 10;
    let fibonnaci = generate_fibonnaci(n);
    println!("The {}th Fibonacci number is {}", n, fibonnaci);
}

fn generate_fibonnaci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
       return generate_fibonnaci(n - 1) + generate_fibonnaci(n - 2);
    }
}
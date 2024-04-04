fn is_prime_rust(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 17;
    if is_prime_rust(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

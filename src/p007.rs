fn is_prime(n: u64) -> bool {
    !(2u64..n).any(|x| n % x == 0)
}

fn main() {
    println!("{}",
    (1u64..)
        .filter(|&x| is_prime(x))
        .nth(10001)
        .unwrap()
    );
}

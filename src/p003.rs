fn is_prime(n: u64) -> bool {
    !(2u64..n).any(|x| n % x == 0)
}

fn main() {
    let ceil = 13195u64;
    println!("{}", // from the smallest possible prime
        (2u64..ceil)
            .filter(|&x| ceil % x == 0 && is_prime(x))
            .last().unwrap());
    
    let ceil = 600851475143u64;
    println!("{}", // from the larger possible prime 
    ceil / (2..ceil)
        .filter(|&x| ceil % x == 0 && is_prime(ceil / x))
        .next().unwrap());
}

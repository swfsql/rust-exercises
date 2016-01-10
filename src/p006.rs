fn main() {
    let top = 101u32;
    println!("{}",
        (1..top)
            .fold(0, |sum, x| sum + x)
            .pow(2)
        -
        (1..top)
            .map(|x| x * x)
            .fold(0, |sum, x| sum + x)
    );
    // correct way = algebra
}

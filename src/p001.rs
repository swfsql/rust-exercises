fn main() {

    let mut sum: u32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }     
    }
    println!("{}", sum);

    println!("{}", 
        (1..1000)
            .filter(|&x| x % 3  == 0 || x % 5 == 0)
            .fold(0, |sum, x| sum + x)

        );
}


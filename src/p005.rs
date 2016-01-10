fn main() {
    println!("{}", 
    (1..)
        .map(|x| x * 20)
        .filter(|&x| (2..20+1)
                .all(|y| x % y == 0)
            )
        .next()
        .unwrap()
    );
}


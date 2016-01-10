fn main() {
    let (mut a, mut b, mut c) = (1, 2, 3);
    let mut sum = 0;
    loop {
        if a > 4_000_000 { break; }
        sum += if a % 2 == 0 { a } else { 0 };
        //print!("{},", a);
        a = b;
        b = c;
        c += a;
    }
    println!("{}", sum);


    println!("{}", 
        fib()
            .filter(|&x| x % 2 == 0)
            .take_while(|&x| x < 4_000_000)
            .fold(0, |sum, x| sum + x)
        );
}



struct Fib {
    curr: u32,
    next: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fib() -> Fib {
    Fib {curr: 1, next: 1}
}

fn digits(num: u32) -> Vec<u32> {
    let upper = (num as f64).log(10.0).ceil() as u32;
    (0u32..upper)
        .filter_map(|x| match num / 10u32.pow(x) { 
            0u32 => None,
            y => Some(y % 10u32),
        })
        .collect::<Vec<_>>()
}

fn is_pal(num: u32) -> bool {
    let d = digits(num);
    !(0..d.len() / 2)
        .any(|x| d[x] != d[d.len() - x - 1])
}

fn main() {
    let top = 10u32.pow(3) - 1;
    let mut min_res = 0;
    println!("{}", 
        (0..top)
            .filter_map(|x| {
                (0..top - x + 1)
                    .filter_map(|y| {
                        let res = (top - x) * (top - x - y );
                        if min_res < res && is_pal(res) {
                            min_res = res;
                            Some(res)
                        } else { 
                            None 
                        }
                    })
                    .next() 
            })
            .max()
            .unwrap()
    );
}

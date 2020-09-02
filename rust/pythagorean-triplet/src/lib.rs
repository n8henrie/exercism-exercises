pub fn find() -> Option<u32> {
    for a in 1..=998 {
        for b in 1..=(998-a) {
            let c = 1000 - a - b;
            if a*a + b*b == c*c {
                let product = a * b * c;
                println!("a: {}, b: {}, c: {}, product: {}", a, b, c, product);
                return Some(product)
            }
        }
    }
    None
}

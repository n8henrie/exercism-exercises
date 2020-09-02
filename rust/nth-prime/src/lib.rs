pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None
    }

    let mut primes: Vec<u32> = vec![2];
    let mut counter = 3;

    while (primes.len() as u32) < n {
        if primes.iter().filter(|&x| counter % x == 0).collect::<Vec<_>>().is_empty() {
            primes.push(counter);
        }
        counter += 2
    }
    if let Some(n) = primes.last() {
        Some(*n)
    } else {
        None
    }
}

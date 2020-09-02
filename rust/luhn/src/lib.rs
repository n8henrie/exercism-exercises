pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&c| !c.is_whitespace())
        .rev()
        .enumerate()
        .try_fold(0u32, |acc, (idx, c)| {
            let n = c.to_digit(10)?;
            if idx % 2 == 1 {
                let doubled = n * 2;
                if doubled > 9 {
                    Some(acc + doubled - 9)
                } else {
                    Some(acc + doubled)
                }
            } else {
                Some(acc + n)
            }
        })
        .map_or(false, |s| code.trim().len() > 1 && s % 10 == 0)
}

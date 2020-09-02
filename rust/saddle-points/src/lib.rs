// It's called a "saddle point" because it is greater than or equal to
// every element in its row and less than or equal to every element in
// its column.
#![feature(test)]

extern crate test;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, &val)| (row_idx, col_idx, val))
        }).filter_map(|(r, c, v)| {
            if v >= *input[r].iter().max().unwrap() && input.iter().all(|row| v <= row[c]) {
                Some((r, c))
            } else {
                None
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_saddle_points(b: &mut Bencher) {
        let input = vec![vec![8, 7, 10, 7, 9], vec![8, 7, 13, 7, 9]];
        b.iter(|| find_saddle_points(&input));
    }
}

fn get_adjacent_count(pos: (usize, usize), minefield: &[Vec<char>]) -> u32 {
    let min_x = pos.0.saturating_sub(1);
    let min_y = pos.1.saturating_sub(1);
    let max_x = {
        let newx = pos.0 + 1;
        if newx < minefield[0].len() {
            newx
        } else {
            pos.0
        }
    };
    let max_y = {
        let newy = pos.1 + 1;
        if newy < minefield.len() {
            newy
        } else {
            pos.1
        }
    };
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .fold(
            0,
            |acc, (x, y)| {
                if minefield[y][x] == '*' {
                    acc + 1
                } else {
                    acc
                }
            },
        )
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield: Vec<Vec<char>> = minefield.iter().map(|&s| s.chars().collect()).collect();

    let mut output = Vec::new();
    for (ri, row) in minefield.iter().enumerate() {
        let mut row_str = String::new();
        for (ci, &val) in row.iter().enumerate() {
            let outval = match val {
                '*' => '*',
                _ => {
                    let count = get_adjacent_count((ci, ri), &minefield);
                    if count > 0 {
                        std::char::from_digit(count, 10)
                            .expect(&format!("invalid count: {}", count))
                    } else {
                        ' '
                    }
                }
            };
            row_str.push(outval);
        }
        output.push(row_str)
    }
    output
}

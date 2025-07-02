pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::<String>::with_capacity(minefield.len());
    for r in 0..minefield.len() {
        let mut row_str = minefield[r].to_owned();
        let row = unsafe { row_str.as_bytes_mut() };
        for c in 0..minefield[0].len() {
            if row[c] == b'*' {
                continue;
            } else {
                let count: u8 = count_mines(r as isize, c as isize, minefield);
                if count > 0 {
                    row[c] = b'0' + count;
                }
            }
        }
        result.push(row_str);
    }
    result
}

fn count_mines(r: isize, c: isize, minefield: &[&str]) -> u8 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut sum = 0;
    for (dr, dc) in directions {
        sum += count_mines_on(r + dr, c + dc, minefield);
    }
    sum
}

fn count_mines_on(r: isize, c: isize, minefield: &[&str]) -> u8 {
    if r < 0 || c < 0 || r >= minefield.len() as isize || c >= minefield[0].len() as isize {
        return 0;
    }
    let c: usize = c.try_into().unwrap();
    let r: usize = r.try_into().unwrap();
    if minefield[r].as_bytes()[c] == b'*' {
        1
    } else {
        0
    }
}

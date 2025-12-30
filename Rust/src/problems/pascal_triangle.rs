pub fn pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![1]];
    for _ in 1..num_rows {
        match res.last() {
            Some(last) => {
                let mut new_row = Vec::with_capacity(last.len() + 1);
                new_row.push(1);
                for i in 1..last.len() {
                    new_row.push(last[i] + (last[i - 1]));
                }
                new_row.push(1);
                res.push(new_row);
            }
            None => (),
        }
    }

    res
}

pub fn pascal_triangle_at(row: u32, column: u32) -> u32 {
    if row < 2 || (column < 1 || column >= row) {
        return 1;
    }

    pascal_triangle_at(row - 1, column) + pascal_triangle_at(row - 1, column - 1)
}

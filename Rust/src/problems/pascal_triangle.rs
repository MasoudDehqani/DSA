use std::collections::HashMap;

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

// pub fn pascal_triangle_at_mem(row: u32, column: u32) -> u32 {
//     let mut memo = HashMap::new();

//     fn aux(r: u32, c: u32, mem: &mut HashMap<(u32, u32), u32>) -> u32 {
//         if r < 2 || (c < 1 || c >= r) {
//             return 1;
//         }

//         let top_right_key = (r - 1, c);
//         let maybe_v1 = mem.get(&top_right_key);
//         let v1 = match maybe_v1 {
//             Some(v) => *v,
//             None => {
//                 let v = aux(top_right_key.0, top_right_key.1, mem);
//                 mem.insert(top_right_key, v);
//                 v
//             }
//         };

//         let top_left_key = (r - 1, c - 1);
//         let maybe_v2 = mem.get(&top_left_key);
//         let v2 = match maybe_v2 {
//             Some(v) => *v,
//             None => {
//                 let v = aux(top_left_key.0, top_left_key.1, mem);
//                 mem.insert(top_left_key, v);
//                 v
//             }
//         };

//         v1 + v2
//     }

//     aux(row, column, &mut memo)
// }

pub fn pascal_triangle_at_mem(row: u32, column: u32) -> u64 {
    let mut memo = HashMap::new();

    fn aux(r: u32, c: u32, mem: &mut HashMap<(u32, u32), u64>) -> u64 {
        if c == 0 || c == r {
            return 1;
        }

        if let Some(&v) = mem.get(&(r, c)) {
            return v;
        }

        let v = aux(r - 1, c, mem) + aux(r - 1, c - 1, mem);
        mem.insert((r, c), v);
        v
    }

    aux(row, column, &mut memo)
}

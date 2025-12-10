// pub fn pyramid_of_stars(number_of_rows: u16) {
//     let number_of_columns = (number_of_rows * 2) - 1;
//     let mut start = number_of_rows;
//     let mut end = number_of_rows;

//     for _ in 1..=number_of_rows {
//         for c in 1..=number_of_columns {
//             if c >= start && c <= end {
//                 print!("*");
//             } else {
//                 print!(" ");
//             }
//         }

//         start -= 1;
//         end += 1;

//         print!("\n");
//     }
// }

pub fn pyramid_of_stars(number_of_rows: u16) {
    for r in 0..number_of_rows {
        for c in 1..(number_of_rows * 2) {
            if c >= number_of_rows - r && c <= number_of_rows + r {
                print!("*");
            } else {
                print!(" ");
            }
        }

        print!("\n");
    }
}

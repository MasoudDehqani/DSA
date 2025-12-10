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

use std::io::stdin;

pub fn pyramid_of_stars() {
    let mut rows = String::new();
    stdin()
        .read_line(&mut rows)
        .expect("Couldn't read user input");

    let rows = match rows.trim().parse::<u16>() {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    for r in 0..rows {
        for c in 1..(rows * 2) {
            if c >= rows - r && c <= rows + r {
                print!("*");
            } else {
                print!(" ");
            }
        }

        print!("\n");
    }
}

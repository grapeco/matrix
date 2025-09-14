use std::{
    io, 
    ops::Neg, 
    str::FromStr
};
use num::Num;

// first square brackets = line
// second square brackets = column

struct Size {
    width: usize,
    height: usize
}

fn read_matrix<T>(size: &Size) -> Vec<Vec<T>>
where T: Num + FromStr, <T as FromStr>::Err: std::fmt::Debug
{
    let mut array = Vec::with_capacity(size.height);

    for _ in 0..size.height {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("read error");
        let row = str
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        array.push(row);
    }

    array
}

fn sarrus_rule<T: Copy + Num>(vec: &Vec<Vec<T>>) -> T {
    (vec[0][0] * vec[1][1] * vec[2][2]) +
    (vec[0][1] * vec[1][2] * vec[2][0]) +
    (vec[0][2] * vec[1][0] * vec[2][1]) -
    (vec[0][1] * vec[1][0] * vec[2][2]) -
    (vec[0][0] * vec[1][2] * vec[2][1]) -
    (vec[0][2] * vec[1][1] * vec[2][0])
}

fn algebraic_complement<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>, row_index: usize, column_index: usize) -> T {
    let mut erased_vec = vec.clone();

    while erased_vec[0].len() > 3 && erased_vec.len() > 3 {
        erased_vec.remove(0);
        for i in 0..erased_vec.len() {
            erased_vec[i].remove(column_index);
        }
    }

    if (row_index + column_index) % 2 == 0 {
        sarrus_rule(&erased_vec)
    } else {
        -sarrus_rule(&erased_vec)
    }
}

fn order_determinant<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>, size: &Size) -> T {
    match (size.width, size.height) {
        (2, 2) => (vec[0][0] * vec[1][1]) - (vec[1][0] * vec[0][1]),
        (3, 3) => sarrus_rule(vec),
        (4, 4) => {
            vec[0][0] * algebraic_complement(&vec, 0, 0) +
            vec[0][1] * algebraic_complement(&vec, 0, 1) +
            vec[0][2] * algebraic_complement(&vec, 0, 2) +
            vec[0][3] * algebraic_complement(&vec, 0, 3)
        }
        _ => todo!(),
    }
}

fn main() -> io::Result<()> {
    let _test3 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let _test4 = vec![
        vec![1, -2, 3, 9],
        vec![2, -4, 1, 0],
        vec![3, -5, 2, 80],
        vec![7, -10, 4, 3]
    ];

    let _test5 = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9 ,10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25]
    ];

    let mut size_text = String::new();
    println!("Choose your matrix size(2, 3, 4):");
    io::stdin().read_line(&mut size_text)?;

    let width: usize = size_text.trim().parse().unwrap();
    let height: usize = size_text.trim().parse().unwrap();

    println!("Wrie your matrix");

    let size = Size { width, height };
    let vec = read_matrix::<f64>(&size);

    println!("{:.2}", order_determinant(&vec, &size));

    println!("Press Enter to close programm");
    io::stdin().read_line(&mut "".to_string())?;

    Ok(())
}

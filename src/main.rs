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
    let mut array: Vec<Vec<T>> = Vec::new();

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

fn erase_vec<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>, column_index: usize) -> Vec<Vec<T>> {
    let mut erased_vec = vec.clone();

    erased_vec.remove(0);
    for i in 0..erased_vec.len() {
        erased_vec[i].remove(column_index);
    }

    erased_vec
}

fn sarrus_rule<T: Copy + Num>(vec: &Vec<Vec<T>>) -> T {
    (vec[0][0] * vec[1][1] * vec[2][2]) +
    (vec[0][1] * vec[1][2] * vec[2][0]) +
    (vec[0][2] * vec[1][0] * vec[2][1]) -
    (vec[0][1] * vec[1][0] * vec[2][2]) -
    (vec[0][0] * vec[1][2] * vec[2][1]) -
    (vec[0][2] * vec[1][1] * vec[2][0])
}

fn four<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>) -> T {
    let mut result = T::zero();

    for i in 0..vec.len() {
        if i % 2 == 0 {
            result = result + vec[0][i] * sarrus_rule(&erase_vec(vec, i));
        } else {
            result = result + vec[0][i] * -sarrus_rule(&erase_vec(vec, i));
        }
    }

    result   
}

fn five<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>) -> T {
    let mut result = T::zero();

    for i in 0..vec.len() {
        if i % 2 == 0 {
            result = result + vec[0][i] * four(&erase_vec(vec, i));
        } else {
            result = result + vec[0][i] * -four(&erase_vec(vec, i));
        }
    }

    result
}

fn order_determinant<T: Copy + Num + Neg<Output = T>>(vec: &Vec<Vec<T>>, size: &Size) -> T {
    match (size.width, size.height) {
        (2, 2) => (vec[0][0] * vec[1][1]) - (vec[1][0] * vec[0][1]),
        (3, 3) => sarrus_rule(vec),
        (4, 4) => four(&vec),
        (5, 5) => five(&vec),
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
        vec![4, 2, 0, -2, 1],
        vec![1, 1, 2, 4, 0],
        vec![2, -1, 0, 4, 1],
        vec![5, 2, 1, -2, 4],
        vec![1, 5, 0, 2, 3]
    ];

    let mut size_text = String::new();
    println!("Choose your matrix size(2, 3, 4, 5):");
    io::stdin().read_line(&mut size_text)?;

    println!("Wrie your matrix");
    let size = Size {
        width: size_text.trim().parse().unwrap(),
        height: size_text.trim().parse().unwrap()
    };
    let vec = read_matrix::<f32>(&size);
    
    println!("{:.2}", order_determinant(&vec, &size));

    println!("Press Enter to close programm");
    io::stdin().read_line(&mut "".to_string())?;
    
    Ok(())
}

use std::{io, ops::{Mul, Sub}, str::FromStr};

// first square brackets = line
// second square brackets = column

// This is queue of inputs
// 1 2
// 3 4

// 1 2 3
// 4 5 6
// 7 8 9

struct Size {
    width: usize,
    height: usize
}

fn read_matrix<T>(size: &Size) -> Vec<Vec<T>>
where T: Mul + FromStr,
    <T as FromStr>::Err: std::fmt::Debug
{
    let mut array = Vec::with_capacity(size.height);

    for _ in 0..size.height {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("read error");
        let row: Vec<T> = str
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        array.push(row);
    }

    array
}

fn order_determinant<T>(vec: &Vec<Vec<T>>, size: &Size) -> T
where T:
    Copy +
    Mul<Output = T> +
    Sub<Output = T>
{
    match size {
        Size { width: 2, height: 2 } => (vec[0][0] * vec[1][1]) - (vec[1][0] * vec[0][1]),
        Size { width: 3, height: 3 } => {
            (vec[0][0] * vec[1][1] * vec[2][2])
        }
        _ => todo!(),
    }
}

fn main() -> io::Result<()> {
    let size = Size { width: 3, height: 3 };
    let vec: Vec<Vec<i32>> = read_matrix(&size);

    for row in &vec {
        println!("{:?}", row);
    }

    println!("{}", order_determinant(&vec, &size));

    Ok(())
}

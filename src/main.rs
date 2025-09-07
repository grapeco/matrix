use std::{io, ops::{Mul, Sub, Add}, str::FromStr};

// first square brackets = line
// second square brackets = column

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
    Sub<Output = T> +
    Add<Output = T>
{
    match size {
        Size { width: 2, height: 2 } => {
            (vec[0][0] * vec[1][1]) - (vec[1][0] * vec[0][1])
        }
        Size { width: 3, height: 3 } => {
            (vec[0][0] * vec[1][1] * vec[2][2]) +
            (vec[0][1] * vec[1][2] * vec[2][0]) +
            (vec[0][2] * vec[1][0] * vec[2][1]) -
            (vec[0][1] * vec[1][0] * vec[2][2]) -
            (vec[0][0] * vec[1][2] * vec[2][1]) -
            (vec[0][2] * vec[1][1] * vec[2][0])
        }
        _ => todo!(),
    }
}

fn main() -> io::Result<()> {
    let _test = vec![
        vec![1, -2, 3],
        vec![2, -4, 1],
        vec![3, -5, 2]
    ];

    let mut size_text = String::new();
    println!("Choose your matrix size(2 or 3):");
    io::stdin().read_line(&mut size_text)?;

    let width: usize = size_text.trim().parse().unwrap();
    let height: usize = size_text.trim().parse().unwrap();

    println!("Wrie your matrix");
    println!("Example:\n1 2\n3 4");

    let size = Size { width, height };
    // Don't forget to choose matrix type of numbers
    let vec: Vec<Vec<i32>> = read_matrix(&size);

    println!("{}", order_determinant(&vec, &size));

    Ok(())
}

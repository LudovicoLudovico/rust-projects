use super::polynomials;
use super::rational::Rational;

use std::ops::{AddAssign, Mul};

pub struct Matrix(Vec<Vec<Rational>>);

impl Matrix {
    pub fn new_from_string(input: String) -> Matrix {
        parse_to_matrix(input)
    }

    pub fn print(&self) {
        for row in self.0.iter() {
            for col in row.iter() {
                print!("|  ");
                col.print();
                print!("  |");
            }
            print!("\n");
        }
    }
    pub fn gauss(&self) {
        todo!();
    }
    pub fn invert(&self) {
        todo!();
    }
    pub fn check_linear_indipendency(first: &Vec<Rational>, second: &Vec<Rational>) -> bool {
        todo!()
    }
}

fn parse_to_matrix(mut input: String) -> Matrix {
    let mut row_num = 0;
    let mut col_num = 0;
    let mut stop_counting_cols = false;

    for char in input.chars() {
        if char == '{' {
            row_num += 1;
            continue;
        } else if char == ',' && !stop_counting_cols {
            col_num += 1;
            continue;
        } else if char == '}' {
            stop_counting_cols = true;
        }
    }

    col_num += 1;

    input.retain(|c| (c != '{' && c != '}' && !c.is_whitespace()));

    let mut result = Matrix(vec![vec![Rational::new_default(); col_num]; row_num]);

    let tokens: Vec<&str> = input.split(',').collect();

    let mut k = 0;
    for row in result.0.iter_mut() {
        for col in row.iter_mut() {
            *col = Rational::new_from_string(&tokens[k].to_string());

            k += 1;
        }
    }

    result
}

impl AddAssign<&Matrix> for Matrix {
    fn add_assign(&mut self, rhs: &Matrix) {
        for (i, row) in self.0.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col += &rhs.0[i][j];
            }
        }
    }
}

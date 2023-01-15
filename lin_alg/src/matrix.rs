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

// INPUT 1,3,4 | 5,6,7 | 9,10,11
fn parse_to_matrix(mut input: String) -> Matrix {
    input.retain(|c| !c.is_whitespace());
    let tokens: Vec<&str> = input.split('|').collect();

    let mut result = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < tokens.len() {
        let tks: Vec<&str> = tokens[i].split(',').collect();
        result.push(vec![]);
        j = 0;

        while j < tks.len() {
            result[i].push(Rational::new_from_string(&tks[j].to_string()));
            j += 1;
        }
        i += 1;
    }

    Matrix(result)
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

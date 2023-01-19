use super::polynomials;
use super::rational::Rational;
use core::num;
use std::clone::Clone;
use std::mem::swap;
use std::ops::{AddAssign, Mul};

pub struct Matrix(Vec<Vec<Rational>>);

impl Matrix {
    pub fn new_from_string(input: String) -> Matrix {
        parse_to_matrix(input)
    }

    pub fn new_identity(row_num: i32) -> Matrix {
        let mut id: Vec<Vec<Rational>> = vec![];
        for i in 0..row_num {
            let mut row = vec![];

            for j in 0..row_num {
                let el = if i == j {
                    Rational::new_from_int(&1)
                } else {
                    Rational::new_from_int(&0)
                };

                row.push(el)
            }

            id.push(row);
        }

        Matrix(id)
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
    pub fn gauss(&mut self) {
        self.reorder();

        let mut offset = 0;

        for i in 0..(self.0.len() - 1) {
            let a = self.0[i][offset];

            if !a.is_zero() {
                for j in (i + 1)..self.0.len() {
                    let b = self.0[j][offset] * -1;

                    for k in offset..self.0[0].len() {
                        let mut res = self.0[i][k];
                        res *= b / a;
                        self.0[j][k] += res;
                    }
                }
                offset += 1;
            }
        }
    }

    pub fn reorder(&mut self) {
        for i in 0..self.0.len() - 1 {
            for j in 0..self.0[i].len() {
                if self.0[i][j].num() == 0 && self.0[i + 1][j].num() != 0 {
                    self.0.swap(i, i + 1);
                    break;
                }
            }
        }
    }
    pub fn invert(&mut self) -> Result<Self, &str> {
        if self.0.len() != self.0[0].len() {
            return Err("Can't invert a non-square matrix");
        }

        self.reorder();
        let mut id = Matrix::new_identity(self.0.len() as i32);
        let mut offset = 0;

        for i in 0..(self.0.len() - 1) {
            let a = self.0[i][offset];

            if !a.is_zero() {
                for j in (i + 1)..self.0.len() {
                    let b = self.0[j][offset] * -1;

                    for k in offset..self.0[0].len() {
                        let mut res = self.0[i][k];
                        res *= b / a;
                        self.0[j][k] += res;

                        let mut res_id = id.0[i][k];
                        res_id *= b / a;
                        id.0[j][k] += res_id;
                    }
                }
                offset += 1;
            }
        }

        print!("PARTIAL RESULT\n");
        self.print();
        print!("\n");
        id.print();
        print!("\n\n");

        let mut num_of_rows = offset + 1;
        for row in (0..num_of_rows).rev() {
            let b = self.0[row][offset];

            for other_row in 0..row {
                let a = self.0[other_row][offset] * -1;

                let div = a / b;

                for col in 0..self.0[0].len() {
                    let mut el = self.0[row][col];
                    el *= div;
                    self.0[other_row][col] += el;

                    let mut el_id = id.0[row][col];
                    el_id *= div;
                    id.0[other_row][col] += el_id;
                }
            }
        }

        self.print();
        Ok(id)
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

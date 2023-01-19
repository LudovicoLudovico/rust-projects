use super::polynomials;
use super::rational::Rational;
use core::num;
use std::clone::Clone;
use std::mem::swap;
use std::ops::{AddAssign, Mul};

pub struct Matrix(Vec<Vec<Rational>>);

impl Matrix {
    pub fn new_from_string(input: &str) -> Self {
        parse_to_matrix(input.to_string())
    }

    pub fn new_identity(row_num: i32) -> Self {
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

        Self(id)
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

    fn gauss_with_adjacent(&mut self, other: &mut Self) {
        let mut offset = 0;

        for i in 0..(self.0.len() - 1) {
            let a = self.0[i][offset];

            if !a.is_zero() {
                for j in (i + 1)..self.0.len() {
                    let b = self.0[j][offset] * -1;

                    for k in 0..self.0[0].len() {
                        let mut res = self.0[i][k];
                        res *= b / a;
                        self.0[j][k] += res;

                        let mut res_other = other.0[i][k];
                        res_other *= b / a;
                        other.0[j][k] += res_other;
                    }
                }
                offset += 1;
            }
        }
    }

    pub fn reorder(&mut self) {
        let mut offset = 0;
        for row in 0..self.0.len() - 1 {
            for j in 0..offset {
                if self.0[row][j].num() == 0 && self.0[row + 1][j].num() != 0 {
                    self.0.swap(row, row + 1);
                    offset += 1;
                    break;
                }
            }
        }
    }

    fn inverted_gauss(&mut self) {
        let mut offset = self.0.len() - 1;

        let row_num = self.0.len();

        for row in (0..row_num).rev() {
            let b = self.0[row][offset];

            for upper_row in 0..row {
                let a = self.0[upper_row][offset];

                for col in offset..self.0[0].len() {
                    let div = a / b * -1;

                    let mut result = self.0[row][col];
                    result *= div;

                    self.0[upper_row][col] += result;
                }
            }

            for col in 0..self.0[0].len() {
                self.0[row][col] = self.0[row][col] / b;
            }
            if offset != 0 {
                offset -= 1;
            }
        }
    }
    fn inverted_gauss_with_adjacent(&mut self, other: &mut Self) {
        let mut offset = self.0.len() - 1;

        let row_num = self.0.len();

        for row in (0..row_num).rev() {
            let b = self.0[row][offset];

            for upper_row in 0..row {
                let a = self.0[upper_row][offset];

                for col in 0..self.0[0].len() {
                    let div = a / b * -1;

                    let mut result = self.0[row][col];
                    result *= div;
                    self.0[upper_row][col] += result;

                    let mut res = other.0[row][col];
                    res *= div;
                    other.0[upper_row][col] += res;
                }
            }

            for col in 0..self.0[0].len() {
                self.0[row][col] = self.0[row][col] / b;
                other.0[row][col] = other.0[row][col] / b;
            }
            if offset != 0 {
                offset -= 1;
            }
        }
    }

    pub fn invert(&mut self) -> Result<Self, &str> {
        if self.0.len() != self.0[0].len() {
            return Err("Can't invert a non-square matrix");
        }

        self.reorder();

        let mut id = Matrix::new_identity(self.0.len() as i32);

        self.gauss_with_adjacent(&mut id);
        self.print();
        print!("\n");
        id.print();
        self.inverted_gauss_with_adjacent(&mut id);

        Ok(id)
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

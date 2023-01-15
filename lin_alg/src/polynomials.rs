/*

  POLYNOMIALS

*/

use super::rational::Rational;
use std::{
    ops::{Add, AddAssign, Mul},
    vec,
};

pub struct Pol(Vec<Rational>);

impl Pol {
    pub fn new(input: &Vec<Rational>) -> Pol {
        Pol(input.to_vec())
    }
    pub fn new_int(input: &Vec<i32>) -> Pol {
        let mut vec: Vec<Rational> = vec![];

        for i in input {
            vec.push(Rational::new_from_int(i));
        }

        Pol(vec)
    }

    // pub fn print(&self) {
    //     for (i, el) in self.0.iter().enumerate() {
    //         el.print();
    //         print!(" x^{}  ", i);
    //     }
    // }
}

impl Add for Pol {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result: Vec<Rational> = vec![
            Rational::new_default();
            if self.0.len() >= other.0.len() {
                self.0.len()
            } else {
                other.0.len()
            }
        ];

        for (i, el) in result.iter_mut().enumerate() {
            let first: Rational = if i < self.0.len() {
                self.0[i]
            } else {
                Rational::new_default()
            };
            let second: Rational = if i < other.0.len() {
                other.0[i]
            } else {
                Rational::new_default()
            };

            *el = &first + &second;
        }

        Pol(result)
    }
}

impl AddAssign for Pol {
    fn add_assign(&mut self, rhs: Self) {
        if self.0.len() >= rhs.0.len() {
            for (i, el) in rhs.0.iter().enumerate() {
                self.0[i] += el;
            }

            return;
        } else {
            for (i, el) in self.0.iter_mut().enumerate() {
                *el += &rhs.0[i];
            }

            for i in self.0.len()..rhs.0.len() {
                self.0.push(rhs.0[i]);
            }
        }
    }
}

impl Mul for Pol {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Pol(vec![
            Rational::new_default();
            self.0.len() + rhs.0.len() - 1
        ]);

        for (i, s_el) in self.0.iter().enumerate() {
            let mut inter: Vec<Rational> = vec![Rational::new_default(); i];
            for r_el in rhs.0.iter() {
                inter.push(s_el * r_el);
            }

            result += Pol(inter);
        }

        assert!(result.0.len() == self.0.len() + rhs.0.len() - 1);
        result
    }
}

impl PartialEq for Pol {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        } else {
            for i in 0..self.0.len() {
                if self.0[i] != other.0[i] {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test_equals() {
    let a: Pol = Pol::new_int(&vec![1, 1]);
    let b: Pol = Pol::new_int(&vec![1, 1, 3]);
    let c: Pol = Pol::new_int(&vec![1, 1, 3]);

    assert_eq!(false, a == b);
    assert_eq!(true, c == b);
}

#[test]
fn test_add() {
    let a: Pol = Pol::new_int(&vec![1, 1]);
    let b: Pol = Pol::new_int(&vec![1, 1, 3]);
    let result: Pol = Pol::new_int(&vec![2, 2, 3]);

    assert_eq!(true, result == a + b);

    let a: Pol = Pol::new_int(&vec![1, 1]);
    let b: Pol = Pol::new_int(&vec![1, 1, 3]);
    assert_eq!(true, result == b + a);
}

#[test]
fn test_multiply() {
    let a: Pol = Pol::new_int(&vec![0, 1, 2]);
    let b: Pol = Pol::new_int(&vec![1, 3]);
    let result: Pol = Pol::new_int(&vec![0, 1, 5, 6]);

    assert_eq!(true, a * b == result);
}

use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    pub fn new(numerator: &i32, denominator: &i32) -> Rational {
        let mut num = *numerator;
        let mut den = *denominator;

        if den == 0 {
            den = 1;
        }
        if den < 0 {
            den *= -1;
            num *= -1;
        }

        let gcd = gcd(&num, &den);

        assert!(den > 0);

        Rational {
            num: num / gcd,
            den: den / gcd,
        }
    }

    pub fn new_default() -> Rational {
        Rational { num: 0, den: 1 }
    }

    pub fn new_from_int(&num: &i32) -> Rational {
        Rational { num, den: 1 }
    }
    pub fn new_from_string(input: &String) -> Rational {
        let tokens: Vec<&str> = input.split('/').collect();

        let num = tokens[0].parse::<i32>().unwrap();

        let den = if tokens.len() == 2 {
            tokens[1].parse().unwrap()
        } else {
            1
        };

        Rational { num, den }
    }
    pub fn print(&self) {
        assert!(self.den > 0);

        if self.den != 1 && self.num != 0 {
            print!("{} / {}", self.num, self.den);
        } else {
            print!("{}", self.num);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn num(&self) -> i32 {
        self.num
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.den == other.den && self.num == other.num {
            return true;
        }

        false
    }
}
impl Add<&Rational> for &Rational {
    type Output = Rational;

    fn add(self, other: &Rational) -> Rational {
        assert!(self.den > 0 && other.den > 0);

        let mut result: Rational = Rational { num: 1, den: 1 };

        result.den = lcm(&self.den, &other.den);
        result.num = result.den / self.den * self.num + result.den / other.den * other.num;

        let gcd = gcd(&result.num, &(result.den));

        result.num /= gcd;
        result.den /= gcd;

        result
    }
}
impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, other: &Rational) {
        let original_den = self.den;

        self.den = lcm(&self.den, &other.den);
        self.num = self.den / original_den * self.num + self.den / other.den * other.num;

        let gcd = gcd(&self.num, &self.den);

        self.num /= gcd;
        self.den /= gcd;
    }
}
impl AddAssign<Rational> for Rational {
    fn add_assign(&mut self, other: Rational) {
        let original_den = self.den;

        self.den = lcm(&self.den, &other.den);
        self.num = self.num * self.den / original_den + other.num * self.den / other.den;

        let gcd = gcd(&self.num, &self.den);

        self.num /= gcd;
        self.den /= gcd;
    }
}
impl Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Rational {
        let mut result = Rational::new(&self.num, &self.den);
        result.num *= rhs.num;
        result.den *= rhs.den;

        let gcd = gcd(&result.num, &result.den);

        result.num /= gcd;
        result.den /= gcd;

        result
    }
}

impl Mul<i32> for Rational {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        let mut result: Rational = Rational {
            num: self.num,
            den: self.den,
        };
        result.num *= rhs;

        let gcd = gcd(&result.num, &result.den);

        result.num /= gcd;
        result.den /= gcd;

        result
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.den *= rhs.den;

        let gcd = gcd(&self.num, &self.den);

        self.num /= gcd;
        self.den /= gcd;
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Rational) -> Rational {
        let mut result: Rational = Rational {
            num: self.num,
            den: self.den,
        };

        result.num *= rhs.den;
        result.den *= rhs.num;

        let gcd = gcd(&result.num, &result.den);

        result.num /= gcd;
        result.den /= gcd;

        if result.den < 0 {
            result.den *= -1;
            result.num *= -1;
        }

        result
    }
}

fn gcd(first: &i32, second: &i32) -> i32 {
    let mut max = first.abs();
    let mut min = second.abs();

    if min > max {
        max += min;
        min = max - min;
        max -= min;
    }

    if *first == 0 || *second == 0 {
        return 1;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
fn lcm(first: &i32, second: &i32) -> i32 {
    first * second / gcd(first, second)
}

#[test]
fn test_gcd() {
    assert_eq!(true, gcd(&32, &8) == 8);
    assert_eq!(true, gcd(&-1, &2) == 1);
}

#[test]
fn test_lcm() {
    assert_eq!(true, lcm(&3, &2) == 6);
}

#[test]
fn test_addition() {
    let mut a = Rational::new(&3, &5);
    let b = Rational::new(&2, &7);
    let c = Rational::new(&31, &35);

    assert_eq!(true, &a + &b == c);

    a += &b;

    assert_eq!(true, a == c);
}

#[test]
fn test_multiplication() {
    let a = Rational::new(&2, &1);
    let b = Rational::new(&1, &2);
    let result = Rational::new(&1, &1);

    assert_eq!(true, &a * &b == result)
}

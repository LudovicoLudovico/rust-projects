pub mod poly {
    use std::cmp::PartialEq;
    use std::ops::{Add, AddAssign, Mul};

    pub struct Pol(Vec<i32>);

    impl Pol {
        pub fn new(input: &Vec<i32>) -> Pol {
            if input.len() == 0 {
                return Pol(vec![0]);
            }
            Pol(input.to_vec())
        }

        pub fn print(&self) {
            for (i, el) in self.0.iter().enumerate() {
                print!("|   {} x^{}   |", el, i);
            }
        }
    }

    impl Add for Pol {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let mut result: Vec<i32> = vec![
                0;
                if self.0.len() >= other.0.len() {
                    self.0.len()
                } else {
                    other.0.len()
                }
            ];

            for (i, el) in result.iter_mut().enumerate() {
                let first: i32 = if i < self.0.len() { self.0[i] } else { 0 };
                let second: i32 = if i < other.0.len() { other.0[i] } else { 0 };

                *el = first + second;
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
                    *el += rhs.0[i];
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
            let mut result = Pol(vec![0; self.0.len() + rhs.0.len() - 1]);

            for (i, s_el) in self.0.iter().enumerate() {
                let mut inter: Vec<i32> = vec![0; i];
                for r_el in rhs.0.iter() {
                    inter.push(s_el * r_el);
                }

                result += Pol(inter);
            }

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
}

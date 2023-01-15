use rational;

pub mod poly {
    use std::cmp::PartialEq;
    use std::ops::{Add, AddAssign, Mul};

    pub struct Pol(Vec<i32>);

    pub enum RootErrors {
        LessThanZeroDiscriminant,
        UnableToDecompose,
        Unknown,
    }

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

        pub fn find_roots(&self) -> Result<Vec<i32>, RootErrors> {
            if self.0.len() == 3 {
                let mut result: Vec<i32> = vec![0, 0];
                let a = self.0[2];
                let b = self.0[1];
                let c = self.0[0];

                if b * b - 4 * a * c < 0 {
                    return Err(RootErrors::LessThanZeroDiscriminant);
                }

                result[0] = (-b + ((b.pow(2) - 4 * a * c) as f32).sqrt() as i32) / (2 * a);
                result[1] = (-b - ((b.pow(2) - 4 * a * c) as f32).sqrt() as i32) / (2 * a);

                Ok(result)
            } else {
                return Err(RootErrors::Unknown);
            }
        }

        pub fn decompose(&self) -> Result<Vec<Vec<i32>>, RootErrors> {
            if self.0.len() == 3 {
                let roots = self.find_roots();

                match roots {
                    Ok(res) => Ok(vec![vec![res[0] * -1, 1], vec![res[1] * -1, 1]]),
                    Err(_) => Err(RootErrors::UnableToDecompose),
                }
            } else {
                Err(RootErrors::UnableToDecompose)
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

    #[test]
    fn test_equals() {
        let a: Pol = Pol::new(&vec![1, 1]);
        let b: Pol = Pol::new(&vec![1, 1, 3]);
        let c: Pol = Pol::new(&vec![1, 1, 3]);

        assert_eq!(false, a == b);
        assert_eq!(true, c == b);
    }

    #[test]
    fn test_add() {
        let a: Pol = Pol::new(&vec![1, 1]);
        let b: Pol = Pol::new(&vec![1, 1, 3]);
        let result: Pol = Pol::new(&vec![2, 2, 3]);

        assert_eq!(true, result == a + b);

        let a: Pol = Pol::new(&vec![1, 1]);
        let b: Pol = Pol::new(&vec![1, 1, 3]);
        assert_eq!(true, result == b + a);
    }

    #[test]
    fn test_multiply() {
        let a: Pol = Pol::new(&vec![0, 1, 2]);
        let b: Pol = Pol::new(&vec![1, 3]);
        let result: Pol = Pol::new(&vec![0, 1, 5, 6]);

        assert_eq!(true, result == a * b);
    }

    #[test]
    fn test_find_roots() {
        let a = Pol::new(&vec![6, 5, 1]);
        let result = a.find_roots();

        let result = match result {
            Ok(res) => res,
            Err(_) => panic!("Something wrong in find_roots"),
        };

        assert_eq!(true, result[0] == -2 && result[1] == -3);
    }

    #[test]
    fn test_decompose() {
        let a = Pol::new(&vec![-9, 0, 1]);
        let a = a.decompose();

        let result = match a {
            Ok(res) => res,
            Err(_) => panic!("Something wrong in decompose"),
        };

        assert_eq!(true, result[0] == vec![-3, 1] && result[1] == vec![3, 1]);
    }
}

pub mod rational {
    use std::cmp::PartialEq;
    use std::ops::{Add, Mul};

    pub struct Rational {
        num: i32,
        den: i32,
    }

    impl Rational {
        pub fn new(numerator: &i32, denominator: &i32) -> Self {
            let mut den = *denominator;

            if den < 0 {
                den *= -1;
            }

            Rational {
                num: *numerator,
                den,
            }
        }

        pub fn print(&self) {
            print!("{}\n-\n{}", self.num, self.den);
        }

        pub fn sqrt(&mut self) -> &Self {
            self.num = self.num.sqrt();
            self.den = self.den.sqrt();
        }
    }

    impl PartialEq for Rational {
        fn eq(&self, other: &Self) -> bool {
            assert!(gcd(&self.den, &self.num) <= 1);
            assert!(gcd(&other.den, &other.num) <= 1);

            if self.num != other.num || self.den != other.num {
                return false;
            }

            true
        }
    }

    impl Add for Rational {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let new_den = lcm(&self.den, &other.den);
            let new_num = new_den / self.den * self.num + new_den / other.den * other.num;
            let gcd = gcd(&new_num, &new_den);

            Self {
                num: new_num / gcd,
                den: new_den / gcd,
            }
        }
    }

    impl Mul for Rational {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            let numerator = self.num * other.num;
            let denominator = self.den * other.num;
            let gcd = gcd(&numerator, &denominator);

            Self {
                num: numerator / gcd,
                den: denominator / gcd,
            }
        }
    }

    fn gcd(first: &i32, second: &i32) -> i32 {
        let mut a = *first;
        let mut b = *second;

        if b > a {
            a = a - b;
            b = a + b;
            a = b - a;
        }

        loop {
            if b == 0 {
                return a;
            }
            let r = a % b;
            a = b;
            b = r;
        }
    }

    fn lcm(first: &i32, second: &i32) -> i32 {
        first * second / gcd(first, second)
    }

    #[test]
    fn test_gcd() {
        let a = 50;
        let b = 20;
        let c = 0;

        assert_eq!(true, gcd(&a, &b) == 10);
        assert_eq!(true, gcd(&b, &a) == 10);
        assert_eq!(true, gcd(&b, &c) == b);
    }

    #[test]
    fn test_lcm() {
        let a = 2;
        let b = 3;

        assert_eq!(true, lcm(&a, &b) == 6);
    }

    #[test]
    fn test_rational_add() {
        let a = Rational::new(&3, &4);
        let b = Rational::new(&1, &4);
        let result = Rational::new(&1, &1);
        assert_eq!(true, a + b == result);
    }
}

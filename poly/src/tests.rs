#[cfg(test)]
mod tests {
    use crate::poly::poly::Pol;

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
}

use crate::poly::poly::Pol;

pub mod poly;
pub mod tests;

fn main() {
    let a = Pol::new(&vec![-1, 0, 1]); // -1 + x^2

    let mut c = Pol::new(&vec![0, 9, 3, 4]);
    c += a;

    c.print();

    let first = Pol::new(&vec![1, 1]);
    let second = Pol::new(&vec![1, 1, 2]);
    print!("\nFIRST\n");
    first.print();
    print!("\nSECOND\n");
    second.print();
    print!("\nRESULT\n");
    (first * second).print();
}

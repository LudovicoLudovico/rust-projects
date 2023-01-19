mod matrix;
mod polynomials;
mod rational;

use crate::matrix::Matrix;

fn main() {
    let mut mat = Matrix::new_from_string("1,-1,3 | 1,1,2 | 2,0,7");

    let inverted = mat.invert().unwrap();
    inverted.print();
}

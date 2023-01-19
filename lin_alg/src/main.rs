mod matrix;
mod polynomials;
mod rational;

use crate::matrix::Matrix;

fn main() {
    let mut mat1 = Matrix::new_from_string("-1/2,2,3 | 4,5,6 | 7,8,9".to_string());
    let mut mat2 = Matrix::new_from_string("0,1,-1,0 | 1,2,0,1 | 2,-1,1,2".to_string());
    let mut mat3 = Matrix::new_from_string("1,3|1,4".to_string());

    mat2.print();
    print!("\n\n");

    let mat3_inv = mat3.invert().unwrap();
    mat3_inv.print();
}

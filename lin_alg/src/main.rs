mod matrix;
mod polynomials;
mod rational;

use crate::matrix::Matrix;

fn main() {
    let mut mat1 = Matrix::new_from_string("{-1/2,2,3},{4,5,6},{7,8,9}".to_string());
    let mat2 = Matrix::new_from_string("{1/2,-2,-3},{-4,-5,-6},{-7,-8,-9}".to_string());

    mat1.print();

    mat1 += &mat2;
    mat1.print();
}

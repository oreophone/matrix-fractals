use std::f64::consts::PI;
use std::ops::{
   Add, AddAssign,
   Sub, SubAssign,
   Mul, MulAssign,
};
use std::fmt::Display;
use num::{
   Num
};
use std::time::Instant;
use libm::*;


/// INFO ///
// Matrix Structure: <Vec<Vec<Num>>
// (row, col) -> (y, x)
// In Matrix:
// - GENERAL        Functions on matrices that modify it, or return some property of it
// - OPERATORS      Mathematical functions on matrices e.g +, -, •
// - CONSTRUCTORS   Functions that build and return matrices, also include special matrices e.g identity, rotation


// trait alias


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
   pub matrix: Vec<Vec<f64>>
}


impl Matrix {


   /// GENERAL ///


   // returns dimensions of matrix
   pub fn shape(&self) -> [usize; 2] {
       let nrows = self.m().len();
       let ncols = self.m()[0].len();
       for (index, row) in self.m().iter().enumerate() {
           if row.len() != ncols {
               panic!("Matrix.shape/Matrix::new: The dimension of the matrix is indeterminate. Make sure all the rows are the same length! (row {})", index + 1);
           }
       }
       if nrows * ncols == 0 {
           panic!("Empty matrices are not supported.")
       }
       return [nrows, ncols];
   }


   // unwrapper
   pub fn m(&self) -> &Vec<Vec<f64>> {
       &self.matrix
   }


   // adds given row to end of matrix
   pub fn push_row(&mut self, row: Vec<f64>) {
       if row.len() != self.shape()[1] {
           panic!("Matrix.push_row: The row given is of a different length to the other rows. ({} != {})", row.len(), self.shape()[1]);
       }
       self.matrix.push(row);
   }


   // adds given col to end of matrix
   pub fn push_col(&mut self, col: Vec<f64>) {
       if col.len() != self.shape()[0] {
           panic!("Matrix.push_col: The column given is of a different length to the other columns. ({} != {})", col.len(), self.shape()[0]);
       }
       for (index, row) in col.iter().enumerate() {
           self.matrix[index].push(*row);
       }
   }


   // inserts given row into index of matrix
   pub fn insert_row(&mut self, row: Vec<f64>, place: usize) {
       let ncols = self.shape()[1];
       if row.len() != ncols {
           panic!("Matrix.insert_row: The row given is of a different length to the other rows. ({} != {})", row.len(), self.shape()[1]);
       }
       if place >= ncols {
           panic!("Matrix.insert_row: Invalid index! Use push_row to insert a row to the end of the matrix. ({})", place);
       }
       self.matrix.insert(place, row);
   }


   // inserts given col into index of matrix
   pub fn insert_col(&mut self, col: Vec<f64>, place: usize) {
       let nrows = self.shape()[0];
       if col.len() != nrows {
           panic!("Matrix.insert_col: The columns given is of a different length to the other columns. ({} != {})", col.len(), self.shape()[0]);
       }
       if place >= nrows {
           panic!("Matrix.insert_row: Invalid index! Use push_col to insert a column at the end of the matrix. ({})", place);
       }
       for (index, row) in col.iter().enumerate() {
           self.matrix[index].insert(place, *row);
       }
   }


   pub fn flip(&mut self) {
       let mut mat: Vec<Vec<f64>> = vec!();
       self.matrix.reverse();
       for row in self.m().iter() {
           let mut r = row.clone();
           r.reverse();
           mat.push(r);
       }
       self.matrix = mat;
   }


   // rotates the matrix 90 degress clockwise rots times
   pub fn rot90(&mut self, rots: u32) {
       let rotations = rots % 4;
       let shape = self.shape();
       let nrows = shape[0];
       let ncols = shape[1];
       let mut mat: Vec<Vec<f64>> = vec!();
       match rotations {
           1 => {
               for col in 0..ncols {
                   let mut newrow: Vec<f64> = vec!();
                   for row in 0..nrows {
                       newrow.push(self.m()[row][col]);
                   }
                   newrow.reverse();
                   mat.push(newrow);
               }
               self.matrix = mat;
           },
           2 => {
               self.flip();
           },
           3 => {
               for col in 0..ncols {
                   let mut newrow = vec!();
                   for row in 0..nrows {
                       newrow.push(self.m()[row][col]);
                   }
                   mat.push(newrow);
               }
               mat.reverse();
               self.matrix = mat;
           },
           _ => (),
       }
   }


   pub fn print(&self) {
       let mut output = String::new();
       let mut pad_len = 0;
       let mut req_len = 0;
       let mut front_req_len = 0;
       let shape = self.shape();
       let mut input_mat = self.matrix.clone();
       let mut row_summarise = false;
       let mut col_summarise = false;
       let mut pad_len_const = 3;
       if shape[0] > 6 {
           row_summarise = true;
           input_mat = self.matrix[..3].to_vec();
           input_mat.append(&mut self.matrix[(shape[0] - 3)..].to_vec());
       }
       if shape[1] > 6 {
           col_summarise = true;
           pad_len_const = 5;
           let mut new_input_mat = vec!();
           for row in input_mat.iter() {
               let mut newrow = row[..3].to_vec();
               newrow.append(&mut row[(shape[1] - 3)..].to_vec());
               new_input_mat.push(newrow);
           }
           input_mat = new_input_mat;
       }  


       for row in input_mat.iter() {
           let first_length = format!("{}", row[0]).len();
           if(first_length > front_req_len) {
               front_req_len = first_length;
           }
           for col in row.iter() {
               let num_length = format!("{}", col).len();
               if num_length > req_len {
                   req_len = num_length;
               }
           }
       }


       for (y, row) in input_mat.iter().enumerate() {
           let mut output_row = String::from("│ ");
           for (x, col) in row.iter().enumerate() {
               let num_padding;
               if x == 3 && col_summarise {
                   output_row.push_str(" ⋯  ");
               }
               if x == 0 {
                   num_padding = " ".repeat(front_req_len - format!("{}", col).len());
               }
               else {
                   num_padding = " ".repeat(req_len - format!("{}", col).len());
               }
               output_row.push_str(&format!("{num_padding}{} ", col));
           }
           pad_len = output_row.len() - pad_len_const;
           output_row.push_str("│\n");
           output.push_str(&output_row);
           if y == 2 && row_summarise {
               let dp = " ".repeat(req_len);
               let fdp = " ".repeat(front_req_len);
               output.push_str(&format!("│{fdp}⋮{dp}⋮{dp}⋮    {dp}⋮{dp}⋮{dp}⋮ │\n"));
           }
       }
       let padding = " ".repeat(pad_len);
       output = format!("┌{padding}┐\n{output}└{padding}┘");
       println!("{}", output);
   }


   /// OPERATORS ///


   pub fn _op_scalar(&self, rhs: f64, op: ScalarOps) -> Self {
       let calc = match op {
           ScalarOps::sadd => |val: f64, sclr: f64| val + sclr,
           ScalarOps::ssub => |val: f64, sclr: f64| val - sclr,
           ScalarOps::smul => |val: f64, sclr: f64| val * sclr,
       };


       let mut matrix = self.matrix.clone();
       for (y, row) in self.matrix.iter().enumerate() {
           for (x, col) in row.iter().enumerate() {
               matrix[y][x] = calc(matrix[y][x], rhs)
           }
       }
       Matrix { matrix }
   }


   // matrix + matrix
   pub fn plus_matrix(&self, rhs: &Matrix) -> Self {
       self + rhs
   }
   // matrix + scalar
   pub fn plus_scalar(&self, rhs: f64) -> Self {
       self._op_scalar(rhs, ScalarOps::sadd)
   }


   // matrix - matrix
   pub fn minus_matrix(&self, rhs: &Matrix) -> Self {
       self - rhs
   }


   // matrix - scalar
   pub fn minus_scalar(&self, rhs: f64) -> Self {
       self._op_scalar(rhs, ScalarOps::ssub)
   }


   // matrix * matrix
   pub fn mul_matrix(&self, rhs: &Matrix) -> Self {
       self * rhs
   }
   // matrix * scalar
   pub fn mul_scalar(&self, rhs: f64) -> Self {
       self._op_scalar(rhs, ScalarOps::smul)
   }


   // private method for multiply impl
   fn _dot(left: &Vec<f64>, right: &Vec<f64>) -> f64 {
       let mut output: f64 = 0.0;
       let vec_len = right.len();
       for n in 0..vec_len {
           output += left[n] * right[vec_len - n - 1];
       }
       output
   }


   pub fn det(&self) -> f64 {
       // from the top row:
       // pos: go downright mod (length of row)
       let [width, height] = self.shape();
       if width != height {
           panic!("You can only calculate the determinant with square matrices! ({width} != {height})")
       }
       let unwrapped = self.m();
       let mut output: f64 = 0.0;
       let is_twotimestwo = match width {
           2 => 1,
           _ => 0
       };
       for col in 0..(width - is_twotimestwo) {
           let mut current = 1.0;
           let mut current_opp = 1.0;
           for row in 0..height {
               current *= unwrapped[row][(col + row) % width];
               current_opp *= unwrapped[row][((width * 999999) - col - row - 1) % width];
           }
           output += current;
           output -= current_opp;
       }
       return output;
   }


   /// CONSTRUCTORS ///


   // general
   pub fn new(matrix: Vec<Vec<f64>>) -> Self {
       let mat = Self { matrix };
       mat.shape();
       return mat;
   }


   // identity matrix constructor
   pub fn identity(size: usize) -> Self {
       let mut matrix: Vec<Vec<f64>> = vec!();
       for s in 0..size {
           let mut row: Vec<f64> = vec![0.0; size];
           row[s] = 1.0;
           matrix.push(row);
       }
       return Matrix { matrix };
   }


   // transformation matrices
   pub fn build_transformation(dim: usize, matrices: Vec<Matrix>) -> Self { // reverse order
       let mut final_matrix = Matrix::identity(dim);
       for m in matrices.iter().rev() {
           final_matrix = &final_matrix * m;
       }
       final_matrix


   }
   pub fn rot_mat2d(angle: f64, using_rad: bool) -> Self {
       let mut angle_rad = match using_rad {
           true => angle,
           false => angle * PI/180.0
       };
       angle_rad = round(angle_rad * 10000.0) / 10000.0;
       let cosine = round(cos(angle_rad) * 10000.0) / 10000.0;
       let sine = round(sin(angle_rad) * 10000.0) / 10000.0;
       let matrix: Vec<Vec<f64>> = vec![
           vec![cosine, -sine],
           vec![sine, cosine]
       ];
       Matrix {
           matrix
       }
   }


   pub fn dil_mat2d(x: f64, y: f64) -> Self {
       Matrix::new(vec![
           vec![x, 0.0], vec![0.0, y]
       ])
   }


   pub fn shear_mat2d(x: f64, y: f64) -> Self {
       Matrix::new(vec![
           vec![1.0,x], vec![y, 1.0]
       ])
   }


   pub fn refl_mat2d(angle: f64, using_rad: bool) -> Self {
       let mut angle_rad = match using_rad {
           true => angle * 2.0,
           false => angle * PI/90.0
       };
       angle_rad = round(angle_rad * 10000.0) / 10000.0;
       let cosine = round(cos(angle_rad) * 10000.0) / 10000.0;
       let sine = round(sin(angle_rad) * 10000.0) / 10000.0;
       let matrix: Vec<Vec<f64>> = vec![
           vec![cosine, sine],
           vec![sine, -cosine]
       ];
       Matrix {
           matrix
       }
   }
}


impl Add for Matrix {
   type Output = Matrix;
   fn add(self, rhs: Matrix) -> Matrix {
       &self + &rhs
   }
}


// Scalar operators
#[derive(Debug)]
pub enum ScalarOps {
   sadd,
   ssub,
   smul,
}


/// OPERATOR TRAITS ///


// matrix + matrix
impl Add for &Matrix {
   type Output = <Matrix as Add>::Output;
   fn add(self, rhs: &Matrix) -> Self::Output {
       if self.shape() != rhs.shape() {
           panic!("Matrix~Add: The shapes of the two matrices don't match! ({:?} != {:?})", self.shape(), rhs.shape());
       }
       let mut matrix = rhs.matrix.clone();
       for (y, row) in self.matrix.iter().enumerate() {
           for (x, col) in row.iter().enumerate() {
               matrix[y][x] += *col;
           }
       }
       return Matrix { matrix }
   }
}


impl Sub for Matrix {
   type Output = Matrix;
   fn sub(self, rhs: Matrix) -> Matrix {
       &self - &rhs
   }
}


// matrix - matrix
impl Sub for &Matrix {
   type Output = <Matrix as Sub>::Output;
   fn sub(self, rhs: &Matrix) -> Self::Output {
       if self.shape() != rhs.shape() {
           panic!("Matrix~Add: The shapes of the two matrices don't match! ({:?} != {:?})", self.shape(), rhs.shape());
       }
       let mut matrix = self.matrix.clone();
       for (y, row) in rhs.matrix.iter().enumerate() {
           for (x, col) in row.iter().enumerate() {
               matrix[y][x] -= *col;
           }
       }
       Matrix { matrix }
   }
}


impl Mul for Matrix {
   type Output = Matrix;
   fn mul(self, rhs: Matrix) -> Matrix {
       &self * &rhs
   }
}


impl Mul for &Matrix {
   type Output = <Matrix as Mul>::Output;
   fn mul(self, rhs: &Matrix) -> Self::Output {
       let lhs_shape = self.shape();
       let rhs_shape = rhs.shape();
       let mut rhs_aligned = Matrix { matrix: rhs.matrix.to_vec()};
       if lhs_shape[1] != rhs_shape[0] {
           self.print();
           rhs.print();
           panic!("Matrix~Mul: The amount of columns in the first matrix needs to match the amount of rows in the second matrix for the result to be defined! ({} != {})", lhs_shape[1], rhs_shape[0])
       }
       rhs_aligned.rot90(1);
       let mut matrix = vec!();


       for row in self.matrix.iter() {
           let mut new_row = vec!();
           for col in rhs_aligned.matrix.iter() {
               new_row.push(Matrix::_dot(row, col));
           }
           matrix.push(new_row);
       }
       Matrix { matrix }
   }
}
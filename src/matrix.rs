//! Module defining the structure and methods of a matrix.

use std::fmt::Debug;
use rand::Rng;
use std::ops;


/// Structure that defines a matrix. It has only one property, a vector of values of type T that is segmented virtually when operating over the matrix using the constant generic parameters.
#[derive(PartialEq, Debug)]
pub struct Matrix<T: Default + Clone + Debug> {
    pub data: Vec<T>,
    rows: usize,
    cols: usize
}


impl<T: Default + Clone + Debug> Matrix<T> {

    
    /// Constructor of a new, empty matrix of size ROWS*COLS. Every value is initialized using T::default() (0 in case of a usize, for example).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// let mat = Matrix::<f64>::new(2, 2);
    /// // Gives: 
    /// // 0.0f64, 0.0f64
    /// // 0.0f64, 0.0f64
    /// ```
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows*cols],
            rows,
            cols
        }
    }

    /// Method that prints the matrix on the standard output, using an optional separator (tabulation if `None`).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// // Building the matrix
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// // Printing the matrix
    /// mat.print(None);
    /// 
    /// // Output: 
    /// // 2.0  3.6
    /// // 1.2  0.2
    /// ```
    pub fn print(& self, sep: Option<&str>) {

        for i in 0..self.rows {
            for j in 0..self.cols {

                if let Some(s) = sep {
                    print!("{}{:?}", s, self.data[i*self.rows+j]);
                }
                else {
                    print!("\t{:?}", self.data[i*self.rows+j]);
                }
            }

            println!("");
        }
    }

    /// Method to get the [row ; column] item of the matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// // Building the matrix
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// let cell: f64 = mat.get(0, 1).unwrap();
    /// assert_eq!(cell, 3.6f64);
    /// ```
    pub fn get(&self, row: usize, column: usize) -> Option<T> {
        let index = row*self.cols + column;

        if index >= self.data.len() {
            None
        }
        else {
            Some(self.data[index].clone())
        }
    }

    /// Method to set the [row ; column] item of the matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// // Building the matrix
    /// let mut mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// mat.set(0.0f64, 0, 1);
    /// 
    /// assert_eq!(mat.get(0, 1).unwrap(), 0.0f64);
    /// ```
    pub fn set(&mut self, value: T, row: usize, column: usize) -> Result<(), &'static str> {
        let index = row*self.cols + column;

        if index >= self.data.len() {
            Err("Index out of range")
        }
        else {
            self.data[index] = value;
            Ok(())
        }
    }

    /// Method that returns a new, inverted matrix (same dimensions).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// // Reversing the matrix and stocking the result
    /// let rev = mat.reverse();
    /// 
    /// // What we're supposed to get
    /// let rev_ = Matrix::<f64>::from(vec![
    ///     vec![0.2f64, 1.2f64], 
    ///     vec![3.6f64, 2.0f64]
    /// ]);
    /// 
    /// assert_eq!(rev, rev_);
    /// ```
    pub fn reverse(&self) -> Self {
        Self {
            data: self.data.clone().into_iter().rev().collect(),
            rows: self.rows.clone(),
            cols: self.cols.clone()
        }
    }

}


impl <T: Default + Clone + Debug + rand::distributions::uniform::SampleUniform> Matrix<T> {

    /// Sets all cells of the matrix to a random value in a certain range R.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// // Building the randomized matrix
    /// let mat = Matrix::<f64>::rand(5, 5, 0.0f64..10.0f64);
    /// 
    /// // Printing the matrix
    /// mat.print(None)
    /// ```
    pub fn rand<R: rand::distributions::uniform::SampleRange<T> + Clone>(rows: usize, cols: usize, range: R) -> Self {

        let mut rng = rand::thread_rng();

        let mut data = Vec::<T>::new();

        for _i in 0..rows*cols {
            data.push(rng.gen_range(range.clone()));
        }

        Self {
            data,
            rows,
            cols
        }
    }
}


impl<T: Default + Clone + Debug + std::ops::Add<Output = T>> 
ops::Add<Matrix<T>> for Matrix<T> 
{

    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {

        let mut out = Matrix::<T>::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                out.data[i*self.cols+j] = self.data[i*self.cols+j].clone() + rhs.data[i*self.cols+j].clone()
            }
        }

        out

    }
}

impl<T: Default + Clone + Debug + std::ops::Add<Output = T>>
ops::Add<T> for Matrix<T>
{
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T> = Matrix::<T>::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                out.data[i*self.cols+j] = self.data[i*self.cols+j].clone() + rhs.clone();
            }
        }

        out
    }
}



impl<T: Default + Clone + Debug + std::ops::Mul<Output = T> + std::ops::Add<Output = T>> 
ops::Mul<Matrix<T>> for Matrix<T> {

    type Output = Result<Matrix<T>, &'static str>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        
        if !(self.cols == rhs.rows) {
            Err("Matrices must have the same inner size to be multiplied.")
        }
        else {

            let mut out = Matrix::<T>::new(self.rows, rhs.cols);

            for i in 0..self.rows {
                for j in 0..rhs.cols {

                    for k in 0..self.cols {
                        out.data[i*rhs.cols+j] = out.data[i*rhs.cols+j].clone() + self.data[i*rhs.rows+k].clone() * rhs.data[k*rhs.cols+j].clone();
                    }
                    
                }
            }

            Ok(out)
        }
    }
}


impl <T: Default + Clone + Debug> From<Vec<Vec<T>>> for Matrix<T> {

    /// Creates a matrix out of a vector of vectors. ROWS and COLS must be consistent with the data provided.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// ```
    fn from(value: Vec<Vec<T>>) -> Self {

        let rows = value.len();
        let cols = value[0].len();

        let mut data = Vec::<T>::new();

        for mut row in value {
            assert!(row.len() == cols); // all inner vectors must have the same size
            data.append(&mut row);
        }

        Self {
            data,
            rows,
            cols
        }
    }
}

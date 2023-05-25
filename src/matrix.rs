//! Module defining the structure and methods of a matrix.

use std::fmt::Debug;
use rand::Rng;
use std::ops;

/// Structure that defines a matrix. It has only one property, a vector of values of type T that is segmented virtually when operating over the matrix using the constant generic parameters.
#[derive(PartialEq, Debug)]
pub struct Matrix<T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> {
    data: Vec<T>
}



impl<T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

    /// Constructor of a new, empty matrix of size ROWS*COLS. Every value is initialized using T::default() (0 in case of a usize, for example).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// let mat = Matrix::<f64, 2, 2>::new();
    /// // Gives: 
    /// // 0.0f64, 0.0f64
    /// // 0.0f64, 0.0f64
    /// ```
    pub fn new() -> Self {
        Self {
            data: vec![T::default(); ROWS*COLS]
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
    /// let mat = Matrix::<f64, 2, 2>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// // Reversing the matrix and stocking the result
    /// let rev = mat.reverse();
    /// 
    /// // What we're supposed to get
    /// let rev_ = Matrix::<f64, 2, 2>::from(vec![
    ///     vec![0.2f64, 1.2f64], 
    ///     vec![3.6f64, 2.0f64]
    /// ]);
    /// 
    /// assert_eq!(rev, rev_);
    /// ```
    pub fn reverse(&self) -> Self {
        Self {
            data: self.data.clone().into_iter().rev().collect()
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
    /// let mat = Matrix::<f64, 2, 2>::from(vec![
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

        for i in 0..ROWS {
            for j in 0..COLS {

                if let Some(s) = sep {
                    print!("{}{:?}", s, self.data[i*COLS+j]);
                }
                else {
                    print!("\t{:?}", self.data[i*COLS+j]);
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
    /// let mat = Matrix::<f64, 2, 2>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// let cell: f64 = mat.get(0, 1).unwrap();
    /// assert_eq!(cell, 3.6f64);
    /// ```
    pub fn get(&self, row: usize, column: usize) -> Option<T> {
        let index = row*COLS + column;

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
    /// let mut mat = Matrix::<f64, 2, 2>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// mat.set(0.0f64, 0, 1);
    /// 
    /// assert_eq!(mat.get(0, 1).unwrap(), 0.0f64);
    /// ```
    pub fn set(&mut self, value: T, row: usize, column: usize) -> Result<(), &'static str> {
        let index = row*COLS + column;

        if index >= self.data.len() {
            Err("Index out of range")
        }
        else {
            self.data[index] = value;
            Ok(())
        }
    }

}


impl <T: Default + Clone + Debug + rand::distributions::uniform::SampleUniform, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

    /// Sets all cells of the matrix to a random value in a certain range R.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// // Building the randomized matrix
    /// let mat = Matrix::<f64, 5, 5>::rand(0.0f64..10.0f64);
    /// 
    /// // Printing the matrix
    /// mat.print(None)
    /// ```
    pub fn rand<R: rand::distributions::uniform::SampleRange<T> + Clone>(range: R) -> Self {

        let mut rng = rand::thread_rng();

        let mut data = Vec::<T>::new();

        for _i in 0..ROWS*COLS {
            data.push(rng.gen_range(range.clone()));
        }

        Self {
            data
        }
    }
}


impl<T: Default + Clone + Debug + std::ops::Add<Output = T>, const ROWS: usize, const COLS: usize> 
ops::Add<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS> 
{

    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: Matrix<T, ROWS, COLS>) -> Self::Output {

        let mut out = Matrix::<T, ROWS, COLS>::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                out.data[i*COLS+j] = self.data[i*COLS+j].clone() + rhs.data[i*COLS+j].clone()
            }
        }

        out

    }
}

impl<T: Default + Clone + Debug + std::ops::Add<Output = T>, const ROWS: usize, const COLS: usize>
ops::Add<T> for Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T, ROWS, COLS> = Matrix::<T, ROWS, COLS>::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                out.data[i*COLS+j] = self.data[i*COLS+j].clone() + rhs.clone();
            }
        }

        out
    }
}



impl<T: Default + Clone + Debug + std::ops::Mul<Output = T> + std::ops::Add<Output = T>, const ROWS: usize, const COLS: usize, const O_ROWS: usize, const O_COLS: usize> 
ops::Mul<Matrix<T, O_ROWS, O_COLS>> for Matrix<T, ROWS, COLS> {

    type Output = Result<Matrix<T, ROWS, O_COLS>, &'static str>;

    fn mul(self, rhs: Matrix<T, O_ROWS, O_COLS>) -> Self::Output {
        
        if !(COLS == O_ROWS) {
            Err("Matrices must have the same inner size to be multiplied.")
        }
        else {

            let mut out = Matrix::<T, ROWS, O_COLS>::new();

            for i in 0..ROWS {
                for j in 0..O_COLS {

                    for k in 0..COLS {
                        out.data[i*O_COLS+j] = out.data[i*O_COLS+j].clone() + self.data[i*O_ROWS+k].clone() * rhs.data[k*O_COLS+j].clone();
                    }
                    
                }
            }

            Ok(out)
        }
    }
}


impl <T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> From<Vec<Vec<T>>> for Matrix<T, ROWS, COLS> {

    /// Creates a matrix out of a vector of vectors. ROWS and COLS must be consistent with the data provided.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::matrix::*;
    /// 
    /// let mat = Matrix::<f64, 2, 2>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// ```
    fn from(value: Vec<Vec<T>>) -> Self {
        assert!(value.len() == ROWS);

        let mut data = Vec::<T>::new();

        for mut row in value {
            assert!(row.len() == COLS);

            data.append(&mut row);
        }

        Self {
            data
        }
    }
}


pub struct RVector<T: Default + Clone + Debug, const COLS: usize>(Matrix<T, 1, COLS>);
pub struct CVector<T: Default + Clone + Debug, const ROWS: usize>(Matrix<T, ROWS, 1>);


impl<T: Default + Clone + Debug, const COLS: usize> From<CVector<T, COLS>> for RVector<T, COLS> {

    fn from(value: CVector<T, COLS>) -> Self {
        Self(Matrix::<T, 1, COLS>{
            data: value.0.data.clone()
        })
    }
}

impl<T: Default + Clone + Debug, const ROWS: usize> From<RVector<T, ROWS>> for CVector<T, ROWS> {

    fn from(value: RVector<T, ROWS>) -> Self {
        Self(Matrix::<T, ROWS, 1>{
            data: value.0.data.clone()
        })
    }
}

impl<T: Default + Clone + Debug, const COLS: usize> From<Vec<T>> for RVector<T, COLS> {

    fn from(value: Vec<T>) -> Self {
        Self(Matrix::<T, 1, COLS>{
            data: value
        })
    }
}

impl<T: Default + Clone + Debug, const ROWS: usize> From<Vec<T>> for CVector<T, ROWS> {

    fn from(value: Vec<T>) -> Self {
        Self(Matrix::<T, ROWS, 1>{
            data: value
        })
    }
}



impl<T: Default + Clone + Debug, const COLS: usize> RVector<T, COLS> {

    pub fn as_mat(&self) -> &Matrix<T, 1, COLS> {
        &self.0
    }

    pub fn as_mut_mat(&mut self) -> &mut Matrix<T, 1, COLS> {
        &mut self.0
    }
}


impl<T: Default + Clone + Debug, const ROWS: usize> CVector<T, ROWS> {

    pub fn as_mat(&self) -> &Matrix<T, ROWS, 1> {
        &self.0
    }

    pub fn as_mut_mat(&mut self) -> &mut Matrix<T, ROWS, 1> {
        &mut self.0
    }
}
//! This crate is a lightweight, rusty matrix library that allows for simple (as in, easy to put into place) matrix handling and operations.

#[cfg(test)]
mod tests;

use std::{fmt::{Debug, Display, Formatter}, iter::zip};
use rand::Rng;
use std::ops;
use num::pow::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
pub enum MatxError {
    SizeError,

}


/// Structure that defines a matrix. It only has two properties, a vector of values of type T that is segmented virtually when operating over the matrix, and the number of rows and columns.
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize
}


impl<T: Debug> Display for Matrix<T> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "\t{:?}", self.data[i*self.cols+j]).unwrap();
            }

            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl<T: num::NumCast + Clone> Matrix<T> {
    
    /// Constructor of a new, empty matrix of numbers of size rows*cols. Every value is initialized using zeros.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::new(2, 2);
    /// // Gives: 
    /// // 0.0f64, 0.0f64
    /// // 0.0f64, 0.0f64
    /// ```
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![num::NumCast::from(0).unwrap(); rows*cols],
            rows,
            cols
        }
    }
}

impl<T: Default + Clone> Matrix<T> {
    
    /// Constructor of a new, empty matrix of numbers of size rows*cols. Every value is initialized using zeros.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::new(2, 2);
    /// // Gives: 
    /// // 0.0f64, 0.0f64
    /// // 0.0f64, 0.0f64
    /// ```
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows*cols],
            rows,
            cols
        }
    }
}

impl<T: Clone> Matrix<T> {

    pub fn apply<F>(&self, f: F) -> Self 
    where F: FnMut(&T,) -> T
    {
        Self {
            data: self.data.iter().map(f).collect(),
            rows: self.rows,
            cols: self.cols
        }
    }

    /// Method that returns a `Rows` object, an iterator that iterates over rows of a matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// for r in mat.rows() {
    ///     println!("{:?}", r);
    /// }
    /// ```
    pub fn rows<'a>(&'a self) -> Rows<'a, T> {
        Rows::<'a>(self, 0, 0)
    }

    /// Method that returns a `Columns` object, an iterator that iterates over columns of a matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// for r in mat.cols() {
    ///     println!("{:?}", r);
    /// }
    /// ```
    pub fn cols<'a>(&'a self) -> Columns<'a, T> {
        Columns::<'a>(self, 0, 0)
    }

    /// Method that returns a `Cells` object, an iterator that iterates over cells of a matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// for r in mat.iter() {
    ///     println!("{:?}", r);
    /// }
    /// ```
    pub fn iter<'a>(&'a self) -> Cells<'a, T> {
        Cells::<'a>(self, self.data.iter())
    }

    /// Method to get the [row ; column] item of the matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
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
    /// use matx::*;
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
    /// use matx::*;
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
            rows: self.rows,
            cols: self.cols
        }
    }

}


impl <T: rand::distributions::uniform::SampleUniform> Matrix<T> {

    /// Sets all cells of the matrix to a random value in a certain range R.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// // Building the randomized matrix
    /// let mat = Matrix::<f64>::rand(5, 5, 0.0f64..10.0f64);
    /// 
    /// // Printing the matrix
    /// println!("{}", mat);
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


impl<T: std::iter::Sum + Clone>  Matrix<T> {

    /// Method that returns the sum of all cells in the matrix.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
    /// 
    /// let mat = Matrix::<f64>::from(vec![
    ///     vec![2.0f64, 3.6f64], 
    ///     vec![1.2f64, 0.2f64]
    /// ]);
    /// 
    /// let expected = 2.0f64 + 3.6f64 + 1.2f64 + 0.2f64;
    /// 
    /// assert_eq!(expected, mat.sum());
    /// ```
    pub fn sum(&self) -> T {
        self.data.clone().into_iter().sum()
    }
}

// Mat_a + Mat_b
impl<T> 
ops::Add<Matrix<T>> for Matrix<T> 
where T: std::ops::Add<Output = T> + num::NumCast + Clone
{

    type Output = Result<Matrix<T>, MatxError>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {

        if self.rows != rhs.rows || self.cols != rhs.cols {
            Err(MatxError::SizeError)
        }
        else {

            let mut out = Matrix::<T>::new(self.rows, self.cols);

            for (i, (a, b)) in zip(self.data.iter(), rhs.data.iter()).enumerate(){
                out.data[i] = a.to_owned() + b.to_owned();
            }

            Ok(out)
        }

    }
}

// Mat_a - Mat_b
impl<T> 
ops::Sub<Matrix<T>> for Matrix<T> 
where T: std::ops::Sub<Output = T> + num::NumCast + Clone
{

    type Output = Result<Matrix<T>, MatxError>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {

        if self.rows != rhs.rows || self.cols != rhs.cols {
            Err(MatxError::SizeError)
        }
        else {

            let mut out = Matrix::<T>::new(self.rows, self.cols);

            for (i, (a, b)) in zip(self.data.iter(), rhs.data.iter()).enumerate(){
                out.data[i] = a.to_owned() - b.to_owned();
            }

            Ok(out)
        }

    }
}

// Mat_a + b
impl<T>
ops::Add<T> for Matrix<T>
where T: std::ops::Add<Output = T> + num::NumCast + Clone
{
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T> = Matrix::<T>::new(self.rows, self.cols);

        for (i, val) in self.data.iter().enumerate() {
                out.data[i] = val.to_owned() + rhs.clone();
        }

        out
    }
}

// Mat_a - b
impl<T>
ops::Sub<T> for Matrix<T>
where T: std::ops::Sub<Output = T> + num::NumCast + Clone
{
    type Output = Matrix<T>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T> = Matrix::<T>::new(self.rows, self.cols);

        for (i, val) in self.data.iter().enumerate() {
                out.data[i] = val.to_owned() - rhs.clone();
        }

        out
    }
}

// -Mat_a
impl<T>
ops::Neg for Matrix<T> 
where T: ops::Neg<Output = T> + Clone
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            data : self.data.iter().map(|x| -x.clone()).collect(),
            rows : self.rows,
            cols : self.cols
        }
    }
}

// Mat_a ** b
impl<T>
Pow<T> for Matrix<T> 
where T: Clone + std::ops::Mul<Output = T> + Pow<T, Output = T> + num::One
{
    type Output = Matrix<T>;

    fn pow(self, rhs: T) -> Self::Output {

        Self::Output {
            data : self.data.iter().map(|x| Pow::<T>::pow(rhs.clone(), x.clone())).collect(),
            rows : self.rows,
            cols : self.cols
        }

    }
}

// Mat_a * Mat_b
impl<T> 
ops::Mul<Matrix<T>> for Matrix<T> 
where T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + num::NumCast + Clone
{

    type Output = Result<Matrix<T>, MatxError>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        
        if self.cols != rhs.rows {
            Err(MatxError::SizeError)
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

// Mat_a * b
impl<T>
ops::Mul<T> for Matrix<T>
where T: std::ops::Mul<Output = T> + num::NumCast + Clone
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T> = Matrix::<T>::new(self.rows, self.cols);

        for (i, val) in self.data.iter().enumerate() {
                out.data[i] = val.to_owned() * rhs.clone();
        }

        out
    }
}

// Mat_a / Mat_b
impl<T> 
ops::Div<Matrix<T>> for Matrix<T> 
where T: std::ops::Div<Output = T> + std::ops::Add<Output = T> + num::NumCast + Clone
{

    type Output = Result<Matrix<T>, MatxError>;

    fn div(self, rhs: Matrix<T>) -> Self::Output {
        
        if self.cols != rhs.rows {
            Err(MatxError::SizeError)
        }
        else {

            let mut out = Matrix::<T>::new(self.rows, rhs.cols);

            for i in 0..self.rows {
                for j in 0..rhs.cols {

                    for k in 0..self.cols {
                        out.data[i*rhs.cols+j] = out.data[i*rhs.cols+j].clone() + self.data[i*rhs.rows+k].clone() / rhs.data[k*rhs.cols+j].clone();
                    }
                    
                }
            }

            Ok(out)
        }
    }
}

// Mat_a / b
impl<T>
ops::Div<T> for Matrix<T>
where T: std::ops::Div<Output = T> + num::NumCast + Clone
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut out: Matrix<T> = Matrix::<T>::new(self.rows, self.cols);

        for (i, val) in self.data.iter().enumerate() {
                out.data[i] = val.to_owned() / rhs.clone();
        }

        out
    }
}

impl <T> From<Vec<Vec<T>>> for Matrix<T> {

    /// Creates a matrix out of a vector of vectors. ROWS and COLS must be consistent with the data provided.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use matx::*;
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

/// An iterator over rows of a matrix.
pub struct Rows<'a, T>(&'a Matrix<T>, usize, usize);

/// An iterator over columns of a matrix.
pub struct Columns<'a, T>(&'a Matrix<T>, usize, usize);

// An iterator over cells of a matrix.
pub struct Cells<'a, T>(&'a Matrix<T>, std::slice::Iter<'a, T>);


impl<T: Clone> Iterator for Columns<'_, T> {

    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {

        if self.1*self.0.rows + self.2*self.0.rows < self.0.data.len() {

            let mut out = Vec::<T>::new();

            for i in 0..self.0.rows {
                let index = i*self.0.rows + self.1;
                out.push(self.0.data[index].clone());
            }
            self.1 += 1;
            Some(out)
        }
        else{
            None
        }
    }
}


impl<T: Clone> DoubleEndedIterator for Columns<'_, T> {

    fn next_back(&mut self) -> Option<Self::Item> {

        if self.1*self.0.rows + self.2*self.0.rows < self.0.data.len() {

            let mut out = Vec::<T>::new();

            for i in 0..self.0.rows {
                let index = i*self.0.rows + (self.0.cols - self.2 - 1);
                out.push(self.0.data[index].clone());
            }
            self.2 += 1;
            Some(out)
        }
        else{
            None
        }
    }
}


impl<T: Clone> Iterator for Rows<'_, T> {

    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {

        let index = self.1*self.0.cols;

        if index + self.2*self.0.cols < self.0.data.len() {
            self.1 += 1;
            Some(self.0.data[index..index+self.0.cols].to_owned())
        }
        else{
            None
        }
    }
}


impl<T: Clone> DoubleEndedIterator for Rows<'_, T> {

    fn next_back(&mut self) -> Option<Vec<T>> {

        let index = (self.0.rows - self.2 - 1)*self.0.cols;

        if self.1*self.0.cols + self.2*self.0.cols < self.0.data.len() {
            self.2 += 1;
            Some(self.0.data[index..index+self.0.cols].to_owned())
        }
        else{
            None
        }
    }
}


impl<'a, T: Clone> Iterator for Cells<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        self.1.next()
    }
}


impl<'a, T: Clone> DoubleEndedIterator for Cells<'a, T> {

    fn next_back(&mut self) -> Option<&'a T> {

        self.1.next_back()
    }
}
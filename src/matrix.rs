

use std::fmt::Debug;
use rand::Rng;
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Matrix<T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> {
    data: Vec<T>
}



impl<T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

    pub fn new() -> Self {
        Self {
            data: vec![T::default(); ROWS*COLS]
        }
    }

    pub fn from(input: Vec<Vec<T>>) -> Self {
        assert!(input.len() == ROWS);

        let mut data = Vec::<T>::new();

        for mut row in input {
            assert!(row.len() == COLS);

            data.append(&mut row);
        }

        Self {
            data
        }
    }

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


    pub fn get(&self, row: usize, column: usize) -> Option<T> {
        let index = row*COLS + column;

        if index >= self.data.len() {
            None
        }
        else {
            Some(self.data[index].clone())
        }
    }

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


/*impl<T: Default + Clone + Debug, const ROWS: usize, const COLS: usize> PartialEq<T> for Matrix<T, ROWS, COLS> {


    fn eq(&self, other: &Matrix<T, O_ROWS, O_COLS>) -> bool {
        
        if self.data != other.data {
            false
        }
        else if O_ROWS != ROWS || O_COLS != COLS {
            false
        }

        true
    }

}*/



impl <T: Default + Clone + Debug + rand::distributions::uniform::SampleUniform, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {

    pub fn rand<R: rand::distributions::uniform::SampleRange<T> + Clone>(&mut self, range: R) {

        let mut rng = rand::thread_rng();

        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i*COLS+j] = rng.gen_range(range.clone());
            }
        }
    }
}


impl<T: Default + Clone + Debug + std::ops::Add<Output = T>, const ROWS: usize, const COLS: usize, const O_ROWS: usize, const O_COLS: usize> 
ops::Add<Matrix<T, O_ROWS, O_COLS>> for Matrix<T, ROWS, COLS> 
{

    type Output = Result<Matrix<T, ROWS, COLS>, &'static str>;

    fn add(self, rhs: Matrix<T, O_ROWS, O_COLS>) -> Self::Output {

        if !(COLS == O_COLS && ROWS == O_ROWS) {
            Err("Matrices must be the same size to be added.")
        }
        else{
            let mut out = Matrix::<T, ROWS, COLS>::new();

            for i in 0..ROWS {
                for j in 0..COLS {
                    out.data[i*COLS+j] = self.data[i*COLS+j].clone() + rhs.data[i*O_COLS+j].clone()
                }
            }

            Ok(out)
        }

    }
}

impl<T: Default + Clone + Debug + std::ops::Add<Output = T>, const ROWS: usize, const COLS: usize>
ops::Add<T> for Matrix<T, ROWS, COLS>
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: T) -> Self::Output {
        let mut out = Matrix::<T, ROWS, COLS>::new();

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

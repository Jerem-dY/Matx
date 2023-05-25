# Matx

A lightweight, rusty matrix library that allows for simple and fast matrix operations.


![build-badge](https://img.shields.io/github/actions/workflow/status/Jerem-dY/Matx/rust.yml)
[![GitHub](https://img.shields.io/github/license/Jerem-dY/matx)](https://github.com/Jerem-dY/Matx/blob/main/LICENSE)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/Jerem-dY/Matx?logo=github)](https://github.com/Jerem-dY/Matx/releases)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2FJB09SI)](https://twitter.com/JB09SI)

## Usage

### Creating matrices
Declaring a matrix is rather straightforward: you only need to specify the **type of elements**, the number of **rows** and the number of **columns**.

```rust
let mut a_ = Matrix::<f64, 2, 3>::new();
```
  
By default, `using Matrix::new()` will initialize the matrix with zeros. If needed, you can specify the whole content of the matrix using `Matrix::from<Vec<Vec<T>>>()`, so with a vector of vectors (with each representing a row).
```rust
// 1 2 3
// 4 5 6
let a = Matrix::<f64, 2, 3>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);
```

### Operators
**Multiplication** and **addition** are implemented _between matrices_, and _between a matrix and an object of type T_ (the type of the elements).  
Matrix-matrix operations may require compatibility between the two (sizewise), so they return a `Result<>`.

No operation is done in-place: they all generate a new matrix.

```rust
let c = (a + b).unwrap(); // adds matrices a and b, as long as they are the same size
```
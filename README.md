# Matx

A lightweight, rusty matrix library that allows for simple (as in, easy to put into place) matrix handling and operations.  
Operations are not done in-place, so most functions, especially mathematical operations, are pure functions that consume their arguments and output a new matrix.

![build-badge](https://img.shields.io/github/actions/workflow/status/Jerem-dY/Matx/rust.yml)
[![GitHub](https://img.shields.io/github/license/Jerem-dY/matx)](https://github.com/Jerem-dY/Matx/blob/main/LICENSE)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/Jerem-dY/matx?include_prereleases)](https://github.com/Jerem-dY/Matx/releases)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2FJB09SI)](https://twitter.com/JB09SI)


- GitHub:   https://github.com/Jerem-dY/Matx
- Crate.io: https://crates.io/crates/matx
- Doc:      https://docs.rs/matx/latest/matx/index.html

## TODO
- [x] Matrix initialization (*filled*, *random*, *custom*)
- [x] Basic operations `Mat` with `Mat` and `Mat` with `Scalar`
- [x] Better error handling (`Results` for operations that may fail because of uncompatible sizes)
- [ ] Matrix rotations
- [ ] Macros for simpler initialization
- [ ] Better recursive matrices (operations, display, etc.)?
- [ ] Computations on GPU?

## Notes on usage

Do not hesitate to go see [`tests.rs`](https://github.com/Jerem-dY/Matx/blob/main/src/tests.rs) for more examples.

### Creating matrices
Declaring a matrix is rather straightforward: you only need to specify the **type of elements**, the number of **rows** and the number of **columns**.

```rust
let mut a = Matrix::<f64>::new(2, 3);
```
  
By default, using `Matrix::new()` will initialize the matrix with zeros (it is thus only available with T: `num::Num`, so a number). If needed, you can specify the whole content of the matrix using `Matrix::from<Vec<Vec<T>>>()`, so with a vector of vectors (with each representing a row).
```rust
// 1 2 3
// 4 5 6
let a = Matrix::<f64>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);
```

### Operators
**Multiplication** and **addition** are implemented _between matrices_, and _between a matrix and an object of type T_ (the type of the elements) if T is a number.  
Matrix-matrix operations may require compatibility between the two (sizewise) ; operations on matrices, like the dot product, return a `Result<>` since the size checks mostly happen at runtime for now.  
As such, inline arithmetics is discouraged: `a + b * (d - e)` should rather be computed step by step as good pratice, or else all 'em unwraps will make things unreadable.

No operation is done in-place: they all generate a new matrix. You'll need to explicitly `.clone()` a matrix if it should be used in several operations.

Currently implemented operations are as follows:
- `Mat * Mat` and `Mat * scal`
- `Mat / Mat` and `Mat / scal`
- `Mat + Mat` and `Mat + scal`
- `Mat - Mat` and `Mat - scal`
- `Mat ** scal`

Summing up a matrix's content is also available for all types that implement `std::iter::Sum`, using `.sum()`.

### Mapping
You can apply a closure on each matrix element using the `.apply()` method. This is a pretty powerful method that uses the data vector's iterator's `.map()`.

### Iterators

The `Matrix` type has three iterators available:
- a `Rows` iterator accessible through `.rows()`
- a `Columns` iterator accessible through `.cols()`
- a `Cells` iterator accessible through `.cells()`

Each one of them is a double-ended iterator.
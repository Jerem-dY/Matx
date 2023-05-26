use crate::matrix::*;

#[test]
fn inits() {

    let mut a_ = Matrix::<f64>::new(2, 3);
    a_.set(1.0f64, 0, 0).expect("Error");
    a_.set(2.0f64, 0, 1).expect("Error");
    a_.set(3.0f64, 0, 2).expect("Error");
    a_.set(4.0f64, 1, 0).expect("Error");
    a_.set(5.0f64, 1, 1).expect("Error");
    a_.set(6.0f64, 1, 2).expect("Error");


    let a = Matrix::<f64>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);

    assert_eq!(a_, a)
}


#[test]
fn multiply() {

    let a = Matrix::<f64>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);

    let b = Matrix::<f64>::from(vec![
        vec![7.0f64, 8.0f64],
        vec![9.0f64, 10.0f64],
        vec![11.0f64, 12.0f64]
    ]);

    let c = (a * b).unwrap();

    let c_ = Matrix::<f64>::from(vec![
        vec![58.0f64, 64.0f64],
        vec![139.0f64, 154.0f64]
    ]);

    assert_eq!(c, c_)

}


#[test]
fn random() {

    let mat = Matrix::<f64>::rand(5, 5, 0.0f64..10.0f64);
    mat.print(None);
}


#[test]
fn add() {

    // 1 1 1 1 1
    // 2 2 2 2 2
    // 3 3 3 3 3
    // 4 4 4 4 4
    // 5 5 5 5 5
    let a = Matrix::from(vec![
        vec![1.0f64; 5],
        vec![2.0f64; 5],
        vec![3.0f64; 5],
        vec![4.0f64; 5],
        vec![5.0f64; 5],
    ]);

    // 1 1 1 1 1
    // 1 1 1 1 1
    // 1 1 1 1 1
    // 1 1 1 0 1 notice the '0'!
    // 1 1 1 1 1
    let b = Matrix::<f64>::from(vec![
        vec![1.0f64; 5],
        vec![1.0f64; 5],
        vec![1.0f64; 5],
        vec![1.0f64, 1.0f64, 1.0f64, 0.0f64, 1.0f64],
        vec![1.0f64; 5],
    ]);

    let c = a + b;

    // a + b should equal this:
    // 2 2 2 2 2
    // 3 3 3 3 3
    // 4 4 4 4 4
    // 5 5 5 4 5
    // 6 6 6 6 6
    let c_ = Matrix::<f64>::from(vec![
        vec![2.0f64; 5],
        vec![3.0f64; 5],
        vec![4.0f64; 5],
        vec![5.0f64, 5.0f64, 5.0f64, 4.0f64, 5.0f64],
        vec![6.0f64; 5],
    ]);

    assert_eq!(c, c_)
}


#[test]
fn iterators() {

    // 1 1 1 1 1
    // 2 2 2 2 2
    // 3 3 3 3 3
    // 4 4 4 4 4
    // 5 5 5 5 5
    let a = Matrix::from(vec![
        vec![1.0f64; 5],
        vec![2.0f64, 2.0f64, 3.0f64, 2.0f64, 2.0f64],
        vec![3.0f64; 5],
        vec![4.0f64; 5],
        vec![5.0f64; 5],
    ]);

    for r in a.rows() {
        println!("{:?}", r);
    }

    println!();

    for c in a.cols() {
        println!("{:?}", c);
    }
}
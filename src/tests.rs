use crate::*;

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
fn add_scal() {

    let a = Matrix::<f64>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);

    // Expected result for a + 1
    let a_p1 = Matrix::<f64>::from(vec![
        vec![2.0f64, 3.0f64, 4.0f64],
        vec![5.0f64, 6.0f64, 7.0f64]
    ]);

    let pl_scal = a + 1.0f64;

    assert_eq!(pl_scal, a_p1); // Addition with a scalar

}

#[test]
fn sub_scal() {

    let a = Matrix::<f64>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);

    // Expected result for a + 1
    let a_p1 = Matrix::<f64>::from(vec![
        vec![0.0f64, 1.0f64, 2.0f64],
        vec![3.0f64, 4.0f64, 5.0f64]
    ]);

    let pl_scal = a - 1.0f64;

    assert_eq!(pl_scal, a_p1); // Substraction with a scalar

}


#[test]
fn random() {

    let mat = Matrix::<f64>::rand(5, 5, 0.0f64..10.0f64);
    println!("{}", mat);
}


#[test]
fn add_mat() {

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
fn sub_mat() {

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

    let c = a - b;

    // a - b should equal this:
    // 0 0 0 0 0
    // 1 1 1 1 1
    // 2 2 2 2 2
    // 3 3 3 4 3
    // 4 4 4 4 4
    let c_ = Matrix::<f64>::from(vec![
        vec![0.0f64; 5],
        vec![1.0f64; 5],
        vec![2.0f64; 5],
        vec![3.0f64, 3.0f64, 3.0f64, 4.0f64, 3.0f64],
        vec![4.0f64; 5],
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



#[test]
fn serialize() {

    let a = Matrix::from(vec![
        vec![1.0f64; 5],
        vec![2.0f64, 2.0f64, 3.0f64, 2.0f64, 2.0f64],
        vec![3.0f64; 5],
        vec![4.0f64; 5],
        vec![5.0f64; 5],
    ]);

    let serialized = serde_json::to_string(&a).unwrap();
    println!("serialized = {}", serialized);

    let output = String::from(
        "{\"data\":[1.0,1.0,1.0,1.0,1.0,2.0,2.0,3.0,2.0,2.0,3.0,3.0,3.0,3.0,3.0,4.0,4.0,4.0,4.0,4.0,5.0,5.0,5.0,5.0,5.0],\"rows\":5,\"cols\":5}"
    );

    assert_eq!(output, serialized);

    let deserialized: Matrix<f64> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(a, deserialized);
}


#[test]
fn complex_mat() {

    let a: Matrix<Matrix<f64>> = Matrix::from(vec![
        vec![Matrix::<f64>::new(2, 2), Matrix::<f64>::new(2, 2)],
        vec![Matrix::from(vec![vec![59.3f64, 8.76555f64]]), Matrix::from(vec![vec![3.02f64, 5.906f64]])]
    ]);

    println!("{}", a);
}
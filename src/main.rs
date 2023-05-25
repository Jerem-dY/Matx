use matx::matrix::*;

fn main() {
    
    let mut a_ = Matrix::<f64, 2, 3>::new();
    a_.set(1.0f64, 0, 0).expect("Error");
    a_.set(2.0f64, 0, 1).expect("Error");
    a_.set(3.0f64, 0, 2).expect("Error");
    a_.set(4.0f64, 1, 0).expect("Error");
    a_.set(5.0f64, 1, 1).expect("Error");
    a_.set(6.0f64, 1, 2).expect("Error");


    let a = Matrix::<f64, 2, 3>::from(vec![
        vec![1.0f64, 2.0f64, 3.0f64],
        vec![4.0f64, 5.0f64, 6.0f64]
    ]);

    assert_eq!(a_, a);

    let mut b = Matrix::<f64, 3, 2>::new();
    b.set(7.0f64, 0, 0).expect("Error");
    b.set(8.0f64, 0, 1).expect("Error");
    b.set(9.0f64, 1, 0).expect("Error");
    b.set(10.0f64, 1, 1).expect("Error");
    b.set(11.0f64, 2, 0).expect("Error");
    b.set(12.0f64, 2, 1).expect("Error");

    a.print(None);
    println!();
    b.print(None);

    //let c = (a + b).unwrap();

    let c = (a * b).unwrap();

    println!();
    c.print(None);

    /*let c = c + 3.0f64;

    println!();
    c.print(None);
    println!();

    let mut d = Matrix::<f64, 20, 20>::new();
    d.rand(0.0..10.0);
    d.print(Some(" "));*/
}

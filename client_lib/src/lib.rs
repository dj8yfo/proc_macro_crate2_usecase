use foo::{BorshSerialize as FooSer, BorshDeserialize as FooDeser};

#[derive(FooSer, FooDeser)]
pub struct Point {
    x: f64,
}

#[test]
fn test_fooser() {
    let pnt = Point {x: 0.7};

    println!("{:?}", foo::to_vec(&pnt));
    
}

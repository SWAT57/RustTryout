fn main() {
    let x: u8 = 5;
    println!("Hello, world! {}", x);
    let deci = b'C';
    println!("Hello, world! {}", deci);
    let ca = 'C';
    println!("ca = {}", ca);
    println!("Hello, world! {}", deci % x);
    let tup = (50, 'h', true);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    let vec = "hello";
    println!("{:?}", vec);
}

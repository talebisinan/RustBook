fn main() {
    let tup: (i32, f64, u8) = (500, 5.4, 3);
    let (x, y, z) = tup;
    // println!("tup:{:#?}, x:{}, y:{}, z:{}", tup, x, y, z);
    // println!("tuple => 0:{}, 1:{}, 2:{}", tup.0, tup.1, tup.2);

    let foo = ["bar", "Baz"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let omg = [42; 10]; // 10 times 42!
    println!("{:#?}", foo);
    println!("{:#?} , a[0]:{}", a, a[0]);
    println!("{:?}", omg);


}

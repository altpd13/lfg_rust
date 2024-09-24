fn main() {
    // needs type annotation!!!
    // let someParsing = "42".parse().expect("Not a number!");
    let integer_num: i32 = 32;
    let float_num: f32 = 32.0;

    // Tuples
    let tup: (u32, i32, f64) = (500, -300, 1.0);

    //destruct tuple using pattern match
    let (first_num, y, z) = tup;
    println!("the value of y is {}", y);
    println!("the value of first_num is {}", first_num);

    let other_tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = other_tup.0;

    let six_point_four = other_tup.1;

    let one = other_tup.2;

    println!("{}",one)

    // array
    let a = [1,2,3,4,5];
    let first = a[0]
}

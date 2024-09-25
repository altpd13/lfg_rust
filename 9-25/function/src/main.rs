fn some_functoin(x: i32) {
    println!("x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; //  with semicolon mismatched types compile error emits
}

fn main() {
    println!("Hello, world!");

    another_function();
    some_functoin(5);
    other_function(2,'h');
    foo_function();
    let x = five();
    println!("The value of five is {x}");
    let y = plus_one(6);
    println!("plus_one is {y}");
}

fn another_function() {
    println!("another function")
}

fn other_function(x:i32,y:char) {
    println!("x is {x} and y is {y}")
}

fn foo_function() {
    let y = 6; // This is statement plus this function itself is statement thus this function does not return value
    // let x = (let z = 6); // this causes compile error because let x = 6 is statement, no values are returned!
    let x = 5;
    let z = {
        let x = 3;
        x + 1 // expressions don't have semicolon if there is it turns into statement
    };

    println!("foo z is {}",z);

}
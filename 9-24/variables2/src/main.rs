fn main() {
    // Shadowing
    // bind 5
    let x = 5;
    // let x = shadows first x and + 1 -> x becomes 6
    let x = x + 1;
    let x = x * 2;
    println!("x is {}", x);

    // this spaces variable is string
    let spaces = "    ";
    // now this is a new variable with number data type
    let spaces = spaces.len();

    println!("the spaces are {}", spaces)

    // results compile error
    let mut mutSpaces = "    ";
    mutSpaces = mutSpaces.len();

    println!("mutspaces: {}" , mutSpaces);
}

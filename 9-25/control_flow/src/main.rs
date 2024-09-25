fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    complex_loop();
    println!("the value is {number}");

    loop {
        println!("loop de loop");
        break;
    }
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result is {loop_result}");

    let mut number_2 = 3;

    while number_2 != 0 {
        println!("{number_2}");

        number_2 = number_2 - 1;
    }
    println!("LIFTOFF!!!");
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        // this is dangerous
        println!("value is {}", arr[index]);

        index = index + 1;
    }
    for_loop();
}
// loop with label
fn complex_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //  this will break the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {element}");
    }
}

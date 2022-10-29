fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_a_parameter(42);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");

    let z = plus_one(z);
    println!("The new value of z is {z}")
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_a_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// notice no semicolon, which would make these return values statements - unless we use `return`
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

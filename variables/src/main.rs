const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutable variable can have value changed at will
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    // Can shadow and reuse the variable `y` by using `let` again
    let y = y + 1;
    {
        // New `y` variable exists for life of the scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    // `y` variable returns to value outside of the scope (6)
    println!("The value of y is: {y}");

    // We can use shadowing to change the variable's type
    let spaces = "   ";
    let spaces = spaces.len();
}

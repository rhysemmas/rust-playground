use std::io;

fn main() {
    println!("Please input nth fibonacci number");

    let mut f_n = String::new();

    io::stdin()
        .read_line(&mut f_n)
        .expect("Failed to read line");

    let f_n: usize = f_n.trim().parse().expect("Failed to convert string to int");

    let result: u128 = compute_fibonacci(f_n);
    println!("the result is: {result}");
}

fn compute_fibonacci(f_n: usize) -> u128 {
    if f_n == 0 {
        return 0;
    } else if f_n == 1 {
        return 1;
    }

    let mut fib = vec![0, 1];
    let mut counter: usize = 0;
    let mut result: u128 = 0;

    while counter < f_n - 1 {
        let num_1: u128 = fib[counter];
        let num_2: u128 = fib[counter + 1];

        result = num_1 + num_2;
        fib.push(result);

        counter += 1;
    }

    return result;
}

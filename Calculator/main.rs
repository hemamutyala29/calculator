fn add(a: &f64, b: &f64) -> f64 {
    a + b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    a - b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    a * b
}

fn divide(a: &f64, b: &f64) -> f64 {
    if *b == 0.0 {
        panic!("Division by zero");
    }
    a / b
}

fn main() {
    let num1 = 10.0;
    let num2 = 5.0;

    let result_add = add(&num1, &num2);
    let result_subtract = subtract(&num1, &num2);
    let result_multiply = multiply(&num1, &num2);
    let result_divide = divide(&num1, &num2);

    println!("Addition: {}", result_add);
    println!("Subtraction: {}", result_subtract);
    println!("Multiplication: {}", result_multiply);
    println!("Division: {}", result_divide);
}

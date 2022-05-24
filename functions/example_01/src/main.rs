fn main() {
    my_function();
    function_with_parameters(10, 500);

    println!("In rust each piece of code can be either a statement or a expression");
    println!("statements perform an action, but do not return a value");
    println!("while expressions do");

    let sum = sum_two_values(10,20);
    println!("this must be 30 -> {}", sum);
}

fn my_function() {
    println!("Rust uses snake_case convention to define the name of function!");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("this function receives a parameter x with value: {}", x);
    println!("and a parameter y with value: {}", y);
}

fn sum_two_values(x: i32, y: i32) -> i32 {
    println!("in a function that returns a value we can either use the word 'return' and then the value or by default");
    println!("since we already define that this function is going to return a value when we did '-> i32' then rust");
    println!("is going to return the last expression");

    x + y

}
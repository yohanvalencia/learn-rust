fn main() {
    // Control flow
    // rust only accepts boolean conditions in each statement

    let number = 11;

    if number < 10 {
        println!("{} is less than 10", number)
    } else if number > 10 {
        println!("{} is greater than 10", number)
    } else {
        println!("else statement")
    };
}

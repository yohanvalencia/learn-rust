fn main() {
    //by default rust give a data type of i32
    let x = 5;
    println!("The value of x is: {}", x);

    //in this case I am shadowing the unmutable variable
    //this is good because we can preserve its unmutability
    //but I am able to change the data type
    let x = "six";
    println!("The value of x is: {}", x);


    // constants won't change! ever!
    // by convention const are UPPERCASE
    // you can separate every thounsand using the underscore symbol
    const SUBRSCRIBER_COUNT: u32 = 100_000;
    println!("{}",SUBRSCRIBER_COUNT)
}

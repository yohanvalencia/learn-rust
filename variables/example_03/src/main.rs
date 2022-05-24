fn main() {
    //DATA TYPES:

    // Integers
    // Floating-point numbers
    // Booleans
    // Character

    //Compound data types:
    let tup:(&str, i32) = ("Hola Yohan", 100_000);

    println!("unpack a tuple using the dot notation tup.INDEX");
    println!("{}",tup.0);
    println!("{}",tup.1);

    println!("OR you can destructure it (destructuring assignment)");

    let (channel, sub_count) = tup;
    println!("destructure channel variable: {}",channel);
    println!("destructure sub_count variable: {}",sub_count);

    println!();

    println!("now this are arrays. They are fixed by nature");

    let error_codes: [i32; 3] =[200,404,500];
    let not_found: i32 = error_codes[1];

    println!("{}", not_found);

    // this is another way to create an array
    // all 8 values set to 0
    let byte = [0; 8];

    println!("eigth value: {}", byte[7]);
}

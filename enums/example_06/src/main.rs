fn main() {
    // optional enum is set by default in rust-lang. 
    // enum Option<T> {
    //        Some(T),
    //        Nonem
    //}
    // this way rust-lang forces us to control the None option.

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{:#?}", some_number);
    println!("{:#?}", some_string);


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("{:#?}", sum);

    let z: Option<i8> = None;

    let sum = x + z.unwrap_or(0);

    println!("{:#?}", sum);
}

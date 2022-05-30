fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("{:?}", six);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None, // match is a exhaustive pattern so for any other data type we can use underscore as a placeholder
    }
}

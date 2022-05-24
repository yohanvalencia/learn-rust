fn main() {
    
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("value after the loop: {}", result);
}

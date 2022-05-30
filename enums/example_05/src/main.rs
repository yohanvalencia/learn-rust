#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String), 
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write (String),
    ChangeColor (i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}

fn main() {
    
    let localhost = IpAddrKind::V4(127,0,0,1);
    route(localhost);

    let msg = Message::some_function();

}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}",ip_kind);
}
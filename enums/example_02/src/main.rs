#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6, 
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    route(localhost);

}

fn route(ip_kind: IpAddr) {
    println!("{:?}",ip_kind);
}
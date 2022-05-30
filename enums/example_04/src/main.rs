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


fn main() {
    
    let localhost = IpAddrKind::V4(127,0,0,1);
    route(localhost);

}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}",ip_kind);
}
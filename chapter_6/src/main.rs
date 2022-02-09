enum IpAddr {
    V4(String), 
    V6(String)
}


fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));


    println!("home: {}", route_type(&home));
    println!("loopback: {}", route_type(&loopback));
}

fn route_type(ip_kind: &IpAddr) -> String{
    match ip_kind {
        V4 => String::from("IPv4"),
        V6 => String::from("IPv6")
    }
}

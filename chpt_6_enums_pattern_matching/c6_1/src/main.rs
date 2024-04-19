fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // better initalization
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddr { // note types and # of elements can be different for each instance; not true for a struct
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {} // note the function here could take any kind of IpAddrKind

// here's how the std library actually defines IP address enum
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// another enum example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
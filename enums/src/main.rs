#![allow(warnings)]
fn main() {

	// Can either be V4 or V6 not booth
    enum IpAddrKind {
    	V4,
     	V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using Enums
    enum IpAddr1 {
    	V4(u8,u8,u8,u8),
     	V6(String),
    }
    let home1 = IpAddr1::V4(127,0,0,1);
    let loopback1 = IpAddr1::V6(String::from("::1"));
    
    // store data using structs
    struct IpAddr {
    kind: IpAddrKind,
    adress: String,
    }

    let home = IpAddr {
    kind: IpAddrKind::V4,
    adress: String::from("127.0.0.1"),
    };

    let looback = IpAddr {
    kind: IpAddrKind::V6,
    adress: String::from("::1"),
    };
}

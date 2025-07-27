#![warn(dead_code)]
fn main(){
    enum IpAddrKind{
        V4,
        V6,
    }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // fn route(ip_kind:IpAddrKind){}

    // route(IpAddrKind::V4)


    struct IpAddr{
        kind:IpAddrKind,
        address:String,
    }

    let _home = IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1")
    };

    
}